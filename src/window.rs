//! Provides APIs to create windows, communicate with other windows, and manipulate the current window.
//!
//! ## Window events
//! Events can be listened to using [`Window::listen`].
use crate::{
    dpi,
    event::{self, Event},
};
use futures::{
    channel::{
        mpsc::{self, UnboundedSender},
        oneshot,
    },
    Future, FutureExt, Stream, StreamExt,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{any::Any, collections::HashMap, path::PathBuf};
use wasm_bindgen::{prelude::Closure, JsValue};

/// Events that are emitted right here instead of by the created window.
const LOCAL_TAURI_EVENTS: &'static [&'static str; 2] = &["tauri://created", "tauri://error"];

trait SenderVec: Any {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T> SenderVec for Vec<mpsc::UnboundedSender<T>>
where
    T: DeserializeOwned + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}

pub(crate) struct Listen<T> {
    pub rx: mpsc::UnboundedReceiver<T>,
}

impl<T> Stream for Listen<T> {
    type Item = T;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.rx.poll_next_unpin(cx)
    }
}

pub(crate) struct DragDropListen {
    pub rx: mpsc::UnboundedReceiver<Event<DragDropEvent>>,
    pub unlisten_enter: js_sys::Function,
    pub unlisten_drop: js_sys::Function,
    pub unlisten_over: js_sys::Function,
    pub unlisten_leave: js_sys::Function,
}

impl Drop for DragDropListen {
    fn drop(&mut self) {
        log::debug!("Calling unlisten for listen callback");
        self.unlisten_enter
            .call0(&wasm_bindgen::JsValue::NULL)
            .unwrap();
        self.unlisten_drop
            .call0(&wasm_bindgen::JsValue::NULL)
            .unwrap();
        self.unlisten_over
            .call0(&wasm_bindgen::JsValue::NULL)
            .unwrap();
        self.unlisten_leave
            .call0(&wasm_bindgen::JsValue::NULL)
            .unwrap();
    }
}

impl Stream for DragDropListen {
    type Item = Event<DragDropEvent>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.rx.poll_next_unpin(cx)
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct WindowLabel {
    label: String,
}

#[derive(Deserialize, Debug)]
pub enum DragDropEvent {
    Enter(DragDropPayload),
    Over(DragOverPayload),
    Drop(DragDropPayload),
    Leave,
}

#[derive(Deserialize, Debug)]
pub struct DragDropPayload {
    paths: Vec<PathBuf>,
    position: dpi::PhysicalPosition,
}

impl DragDropPayload {
    pub fn paths(&self) -> &Vec<PathBuf> {
        &self.paths
    }

    pub fn position(&self) -> &dpi::PhysicalPosition {
        &self.position
    }
}

#[derive(Deserialize, Debug)]
pub struct DragOverPayload {
    position: dpi::PhysicalPosition,
}

impl DragOverPayload {
    pub fn position(&self) -> &dpi::PhysicalPosition {
        &self.position
    }
}

pub struct Window {
    label: String,
    listeners: HashMap<String, Box<dyn SenderVec>>,
}

impl Window {
    /// Create a new Window.
    ///
    /// # Arguments
    /// + `label`: Unique window label. Must be alphanumberic: `a-zA-Z-/:_`.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            listeners: HashMap::new(),
        }
    }

    /// Gets the Window associated with the given label.
    pub fn get_by_label(label: impl AsRef<str>) -> Option<Self> {
        js_sys::try_iter(&inner::get_all())
            .unwrap()
            .unwrap()
            .into_iter()
            .find_map(|value| {
                let window_label = value.unwrap().as_string().unwrap();
                if window_label == label.as_ref() {
                    Some(Window::new(window_label))
                } else {
                    None
                }
            })
    }

    /// Get an instance of `Window` for the current window.
    pub fn get_current() -> Self {
        get_current()
    }

    /// Gets a list of instances of `Window` for all available windows.
    pub fn get_all() -> Vec<Self> {
        get_all()
    }
}

impl Window {
    pub fn label(&self) -> &String {
        &self.label
    }

    fn handle_tauri_event<T>(&mut self, event: String) -> Option<impl Stream<Item = Event<T>>>
    where
        T: DeserializeOwned + 'static,
    {
        if LOCAL_TAURI_EVENTS.contains(&event.as_str()) {
            let (tx, rx) = mpsc::unbounded::<Event<T>>();
            let entry = self
                .listeners
                .entry(event)
                .or_insert(Box::new(Vec::<mpsc::UnboundedSender<Event<T>>>::new()));

            let senders = entry
                .as_any_mut()
                .downcast_mut::<Vec<mpsc::UnboundedSender<Event<T>>>>()
                .unwrap();
            senders.push(tx);

            Some(Listen { rx })
        } else {
            None
        }
    }
}

impl Window {
    /// Listen to an emitted event on this window.
    ///
    /// # Arguments
    /// + `event`: Event name. Must include only alphanumeric characters, `-`, `/`, `:` and `_`.
    /// + `handler`: Event handler.
    ///
    /// # Returns
    /// A function to unlisten to the event.
    /// Note that removing the listener is required if your listener goes out of scope e.g. the component is unmounted.
    pub async fn listen<T>(
        &mut self,
        event: impl Into<String>,
    ) -> crate::Result<impl Stream<Item = Event<T>>>
    where
        T: DeserializeOwned + 'static,
    {
        use futures::future::Either;

        let event = event.into();
        if let Some(listener) = self.handle_tauri_event(event.clone()) {
            Ok(Either::Left(listener))
        } else {
            let listener =
                event::listen_to(&event, event::EventTarget::Window(self.label.clone())).await?;

            Ok(Either::Right(listener))
        }
    }

    /// Listen to an emitted event on this window only once.
    ///
    /// # Arguments
    /// + `event`: Event name. Must include only alphanumeric characters, `-`, `/`, `:` and `_`.
    /// + `handler`: Event handler.
    ///
    /// # Returns
    /// A promise resolving to a function to unlisten to the event.
    /// Note that removing the listener is required if your listener goes out of scope e.g. the component is unmounted.
    pub async fn once(&self, event: impl Into<String>, handler: Closure<dyn FnMut(JsValue)>) {
        todo!();
    }

    /// Emits an event to all {@link EventTarget|targets}.
    ///
    /// # Arguments
    /// + `event`: Event name. Must include only alphanumeric characters, `-`, `/`, `:` and `_`.
    /// + `payload`: Event payload.
    pub async fn emit<T: Serialize + Clone + 'static>(
        &self,
        event: impl Into<String>,
        payload: T,
    ) -> crate::Result<()> {
        let event: String = event.into();
        if LOCAL_TAURI_EVENTS.contains(&event.as_str()) {
            if let Some(listeners) = self.listeners.get(&event) {
                let listeners = listeners
                    .as_any()
                    .downcast_ref::<Vec<UnboundedSender<Event<T>>>>()
                    .unwrap();

                for listener in listeners {
                    listener
                        .unbounded_send(event::Event {
                            event: event.clone(),
                            id: -1,
                            payload: payload.clone(),
                        })
                        .unwrap();
                }
            }

            Ok(())
        } else {
            event::emit(event.as_str(), &payload).await
        }
    }

    /// Emits an event to all {@link EventTarget|targets} matching the given target.
    ///
    /// # Arguments
    /// + `target`: Label of the target Window/Webview/WebviewWindow or raw {@link EventTarget} object.
    /// + `event`: Event name. Must include only alphanumeric characters, `-`, `/`, `:` and `_`.
    /// + `payload`: Event payload.
    pub async fn emit_to<T: Serialize + Clone + 'static>(
        &self,
        target: &event::EventTarget,
        event: impl Into<String>,
        payload: T,
    ) -> crate::Result<()> {
        let event: String = event.into();
        if LOCAL_TAURI_EVENTS.contains(&event.as_str()) {
            if let Some(listeners) = self.listeners.get(&event) {
                let listeners = listeners
                    .as_any()
                    .downcast_ref::<Vec<UnboundedSender<Event<T>>>>()
                    .unwrap();

                for listener in listeners {
                    listener
                        .unbounded_send(event::Event {
                            event: event.clone(),
                            id: -1,
                            payload: payload.clone(),
                        })
                        .unwrap();
                }
            }

            Ok(())
        } else {
            event::emit_to(target, event.as_str(), &payload).await
        }
    }
}

impl Window {
    /// Listen to a file drop event.
    /// The listener is triggered when the user hovers the selected files on the webview,
    /// drops the files or cancels the operation.
    ///
    /// # Returns
    /// Function to unlisten to the event.
    /// Note that removing the listener is required if your listener goes out of scope e.g. the component is unmounted.
    pub async fn on_drag_drop_event(
        &self,
    ) -> crate::Result<impl Stream<Item = Event<DragDropEvent>>> {
        let (tx, rx) = mpsc::unbounded::<Event<DragDropEvent>>();

        let closure = {
            let tx = tx.clone();
            Closure::<dyn FnMut(JsValue)>::new(move |raw| {
                let Event { event, id, payload } =
                    serde_wasm_bindgen::from_value::<Event<DragDropPayload>>(raw).unwrap();
                let _ = tx.unbounded_send(Event {
                    event,
                    id,
                    payload: DragDropEvent::Enter(payload),
                });
            })
        };
        let unlisten = event::inner::listen(
            event::DRAG_ENTER,
            &closure,
            serde_wasm_bindgen::to_value(&event::Options {
                target: event::EventTarget::Window(self.label.clone()),
            })?,
        )
        .await?;
        closure.forget();

        let unlisten_enter = js_sys::Function::from(unlisten);

        let closure = {
            let tx = tx.clone();
            Closure::<dyn FnMut(JsValue)>::new(move |raw| {
                let Event { event, id, payload } =
                    serde_wasm_bindgen::from_value::<Event<DragDropPayload>>(raw).unwrap();
                let _ = tx.unbounded_send(Event {
                    event,
                    id,
                    payload: DragDropEvent::Drop(payload),
                });
            })
        };
        let unlisten = event::inner::listen(
            event::DRAG_DROP,
            &closure,
            serde_wasm_bindgen::to_value(&event::Options {
                target: event::EventTarget::Window(self.label.clone()),
            })?,
        )
        .await?;
        closure.forget();

        let unlisten_drop = js_sys::Function::from(unlisten);

        let closure = {
            let tx = tx.clone();
            Closure::<dyn FnMut(JsValue)>::new(move |raw| {
                let Event { event, id, payload } =
                    serde_wasm_bindgen::from_value::<Event<DragOverPayload>>(raw).unwrap();
                let _ = tx.unbounded_send(Event {
                    event,
                    id,
                    payload: DragDropEvent::Over(payload),
                });
            })
        };
        let unlisten = event::inner::listen(
            event::DRAG_OVER,
            &closure,
            serde_wasm_bindgen::to_value(&event::Options {
                target: event::EventTarget::Window(self.label.clone()),
            })?,
        )
        .await?;
        closure.forget();

        let unlisten_over = js_sys::Function::from(unlisten);

        let closure = {
            let tx = tx.clone();
            Closure::<dyn FnMut(JsValue)>::new(move |raw| {
                let Event { event, id, .. } =
                    serde_wasm_bindgen::from_value::<Event<()>>(raw).unwrap();
                let _ = tx.unbounded_send(Event {
                    event,
                    id,
                    payload: DragDropEvent::Leave,
                });
            })
        };
        let unlisten = event::inner::listen(
            event::DRAG_LEAVE,
            &closure,
            serde_wasm_bindgen::to_value(&event::Options {
                target: event::EventTarget::Window(self.label.clone()),
            })?,
        )
        .await?;
        closure.forget();

        let unlisten_leave = js_sys::Function::from(unlisten);

        Ok(DragDropListen {
            rx,
            unlisten_enter,
            unlisten_drop,
            unlisten_over,
            unlisten_leave,
        })
    }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Monitor {
    /// Human-readable name of the monitor.
    name: Option<String>,

    /// The monitor's resolution.
    size: dpi::PhysicalSize,

    /// the Top-left corner position of the monitor relative to the larger full screen area.
    position: dpi::PhysicalPosition,

    /// The scale factor that can be used to map physical pixels to logical pixels.
    scale_factor: f64,
}

impl Monitor {
    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn size(&self) -> &dpi::PhysicalSize {
        &self.size
    }

    pub fn position(&self) -> &dpi::PhysicalPosition {
        &self.position
    }

    pub fn scale_factor(&self) -> f64 {
        self.scale_factor
    }
}

pub fn get_current() -> Window {
    let WindowLabel { label } = serde_wasm_bindgen::from_value(inner::get_current()).unwrap();
    Window::new(label)
}

pub fn get_all() -> Vec<Window> {
    js_sys::try_iter(&inner::get_all())
        .unwrap()
        .unwrap()
        .into_iter()
        .map(|value| {
            let WindowLabel { label } = serde_wasm_bindgen::from_value(value.unwrap()).unwrap();
            Window::new(label)
        })
        .collect()
}

/// # Returns
/// Monitor on which the window currently resides.
pub async fn current_monitor() -> Option<Monitor> {
    let value = inner::current_monitor().await;
    if value.is_null() {
        None
    } else {
        Some(serde_wasm_bindgen::from_value(value).unwrap())
    }
}

/// # Returns
/// Primary monitor of the system.
pub async fn primary_monitor() -> Option<Monitor> {
    let value = inner::primary_monitor().await;
    if value.is_null() {
        None
    } else {
        Some(serde_wasm_bindgen::from_value(value).unwrap())
    }
}

/// # Returns
/// Monitor that contains the given point.
pub async fn monitor_from_point(x: isize, y: isize) -> Option<Monitor> {
    let value = inner::monitor_from_point(x, y).await;
    if value.is_null() {
        None
    } else {
        Some(serde_wasm_bindgen::from_value(value).unwrap())
    }
}

/// # Returns
/// All the monitors available on the system.
pub async fn available_monitors() -> Vec<Monitor> {
    let value = inner::available_monitors().await;
    js_sys::try_iter(&value)
        .unwrap()
        .unwrap()
        .into_iter()
        .map(|value| serde_wasm_bindgen::from_value(value.unwrap()).unwrap())
        .collect()
}

// TODO: Issue with cursorPosition in Tauri.
// See: https://github.com/tauri-apps/tauri/issues/10340
///// Get the cursor position relative to the top-left hand corner of the desktop.
/////
///// Note that the top-left hand corner of the desktop is not necessarily the same as the screen.
///// If the user uses a desktop with multiple monitors,
///// the top-left hand corner of the desktop is the top-left hand corner of the main monitor on Windows and macOS
///// or the top-left of the leftmost monitor on X11.
/////
///// The coordinates can be negative if the top-left hand corner of the window is outside of the visible screen region.
//pub async fn cursor_position() -> PhysicalPosition {
//    serde_wasm_bindgen::from_value(inner::cursor_position().await).unwrap()
//}

mod inner {
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

    #[wasm_bindgen(module = "/src/window.js")]
    extern "C" {
        #[wasm_bindgen(js_name = "getCurrent")]
        pub fn get_current() -> JsValue;
        #[wasm_bindgen(js_name = "getAll")]
        pub fn get_all() -> JsValue;
        #[wasm_bindgen(js_name = "currentMonitor")]
        pub async fn current_monitor() -> JsValue;
        #[wasm_bindgen(js_name = "primaryMonitor")]
        pub async fn primary_monitor() -> JsValue;
        #[wasm_bindgen(js_name = "monitorFromPoint")]
        pub async fn monitor_from_point(x: isize, y: isize) -> JsValue;
        #[wasm_bindgen(js_name = "availableMonitors")]
        pub async fn available_monitors() -> JsValue;
        #[wasm_bindgen(js_name = "cursorPosition")]
        pub async fn cursor_position() -> JsValue;
    }
}

// partial mocks
/*
pub enum Theme {
    Light,
    Dark,
}

/// Attention type to request on a window.
pub enum UserAttentionType {
    /// # Platform-specific
    /// - **macOS:** Bounces the dock icon until the application is in focus.
    /// - **Windows:** Flashes both the window and the taskbar button until the application is in focus.
    Critical = 1,

    /// # Platform-specific
    /// - **macOS:** Bounces the dock icon once.
    /// - **Windows:** Flashes the taskbar button until the application is in focus.
    Informational,
}

impl Window {
    /// The scale factor that can be used to map physical pixels to logical pixels.
    pub async fn scale_factor(&self) -> dpi::ScaleFactor {
        todo!();
    }

    /// The position of the top-left hand corner of the window's client area relative to the top-left hand corner of the desktop.
    pub async fn inner_position(&self) -> dpi::PhysicalPosition {
        todo!();
    }

    /// The position of the top-left hand corner of the window relative to the top-left hand corner of the desktop.
    pub async fn outer_position(&self) -> dpi::PhysicalPosition {
        todo!();
    }

    /// The physical size of the window's client area.
    /// The client area is the content of the window, excluding the title bar and borders.
    pub async fn inner_size(&self) -> dpi::PhysicalSize {
        todo!();
    }

    /// The physical size of the entire window.
    /// These dimensions include the title bar and borders. If you don't want that (and you usually don't), use inner_size instead.
    pub async fn outer_size(&self) -> dpi::PhysicalSize {
        todo!();
    }

    /// Gets the window's current fullscreen state.
    pub async fn is_fullscreen(&self) -> bool {
        todo!();
    }

    /// Gets the window's current minimized state.
    pub async fn is_minimized(&self) -> bool {
        todo!();
    }

    /// Gets the window's current maximized state.
    pub async fn is_maximized(&self) -> bool {
        todo!();
    }

    /// Gets the window's current focused state.
    pub async fn is_focused(&self) -> bool {
        todo!();
    }

    /// Gets the window's current decorated state.
    pub async fn is_decorated(&self) -> bool {
        todo!();
    }

    /// Gets the window's current resizable state.
    pub async fn is_resizable(&self) -> bool {
        todo!();
    }

    /// Gets the window's current visible state.
    pub async fn is_visible(&self) -> bool {
        todo!();
    }

    /// Gets the window's current title.
    pub async fn title(&self) -> String {
        todo!();
    }

    /// Gets the window's current theme.
    pub async fn theme(&self) -> Option<Theme> {
        todo!();
    }
}

/// # Platform-specific
/// - **Linux / iOS / Android:** Unsupported.
impl Window {
    /// Gets the window's native maximize button state.
    ///
    /// # Platform-specific
    /// - **Linux / iOS / Android:** Unsupported.
    pub async fn is_maximizable(&self) -> bool {
        todo!();
    }

    /// Gets the window's native minimize button state.
    ///
    /// # Platform-specific
    /// - **Linux / iOS / Android:** Unsupported.
    pub async fn is_minimizable(&self) -> bool {
        todo!();
    }

    /// Gets the window's native close button state.
    ///
    /// # Platform-specific
    /// - **Linux / iOS / Android:** Unsupported.
    pub async fn is_closable(&self) -> bool {
        todo!();
    }
}

impl Window {
    /// Centers the window.
    ///
    /// # Returns
    /// The success or failure of the operation.
    pub async fn center(&self) -> Result<(), ()> {
        todo!();
    }

    /// Requests user attention to the window, this has no effect if the application
    /// is already focused. How requesting for user attention manifests is platform dependent,
    /// see `UserAttentionType` for details.
    ///
    /// Providing `null` will unset the request for user attention. Unsetting the request for
    /// user attention might not be done automatically by the WM when the window receives input.
    ///
    /// # Platform-specific
    /// - **macOS:** `null` has no effect.
    /// - **Linux:** Urgency levels have the same effect.
    ///
    /// # Returns
    /// The success or failure of the operation.
    pub async fn request_user_attention() -> Result<(), ()> {
        todo!();
    }

    /// Requests user attention to the window, this has no effect if the application
    /// is already focused. How requesting for user attention manifests is platform dependent,
    /// see `UserAttentionType` for details.
    ///
    /// Providing `null` will unset the request for user attention. Unsetting the request for
    /// user attention might not be done automatically by the WM when the window receives input.
    ///
    /// # Platform-specific
    /// - **macOS:** `null` has no effect.
    /// - **Linux:** Urgency levels have the same effect.
    ///
    /// # Returns
    /// The success or failure of the operation.
    pub async fn request_user_attention_with_type(
        request_type: UserAttentionType,
    ) -> Result<(), ()> {
        todo!();
    }

    /// Updates the window resizable flag.
    ///
    /// # Returns
    /// The success or failure of the operation.
    pub async fn set_resizable(&self, resizable: bool) -> Result<(), ()> {
        todo!();
    }
}
*/
