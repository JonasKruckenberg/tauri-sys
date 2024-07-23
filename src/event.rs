//! The event system allows you to emit events to the backend and listen to events from it.
use futures::{
    channel::{mpsc, oneshot},
    Future, FutureExt, Stream, StreamExt,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;
use wasm_bindgen::{prelude::Closure, JsValue};

pub const WINDOW_RESIZED: &str = "tauri://resize";
pub const WINDOW_MOVED: &str = "tauri://move";
pub const WINDOW_CLOSE_REQUESTED: &str = "tauri://close-requested";
pub const WINDOW_DESTROYED: &str = "tauri://destroyed";
pub const WINDOW_FOCUS: &str = "tauri://focus";
pub const WINDOW_BLUR: &str = "tauri://blur";
pub const WINDOW_SCALE_FACTOR_CHANGED: &str = "tauri://scale-change";
pub const WINDOW_THEME_CHANGED: &str = "tauri://theme-changed";
pub const WINDOW_CREATED: &str = "tauri://window-created";
pub const WEBVIEW_CREATED: &str = "tauri://webview-created";
pub const DRAG: &str = "tauri://drag";
pub const DROP: &str = "tauri://drop";
pub const DROP_OVER: &str = "tauri://drop-over";
pub const DROP_CANCELLED: &str = "tauri://drag-cancelled";

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event<T> {
    /// Event name
    pub event: String,
    /// Event identifier used to unlisten
    pub id: isize,
    /// Event payload
    pub payload: T,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind", content = "label")]
pub enum EventTarget {
    Any,
    AnyLabel(String),
    App,
    Window(String),
    Webview(String),
    WebviewWindow(String),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct Options {
    pub target: EventTarget,
}

/// Emits an event to the backend.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_api::event::emit;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Payload {
///     logged_in: bool,
///     token: String
/// }
///
/// emit("frontend-loaded", &Payload { logged_in: true, token: "authToken" }).await;
/// ```
///
/// @param event Event name. Must include only alphanumeric characters, `-`, `/`, `:` and `_`.
#[inline(always)]
pub async fn emit<T: Serialize>(event: &str, payload: &T) -> crate::Result<()> {
    inner::emit(event, serde_wasm_bindgen::to_value(payload)?).await?;

    Ok(())
}

/// Emits an event to the backend.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_api::event::{EventTarget, emit_to};
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Payload {
///     logged_in: bool,
///     token: String
/// }
///
/// emit_to(EventTarget::Any, "frontend-loaded", &Payload { logged_in: true, token: "authToken" }).await;
/// ```
///
/// @param event Event name. Must include only alphanumeric characters, `-`, `/`, `:` and `_`.
#[inline(always)]
pub async fn emit_to<T: Serialize>(
    target: &EventTarget,
    event: &str,
    payload: &T,
) -> crate::Result<()> {
    inner::emitTo(
        serde_wasm_bindgen::to_value(target)?,
        event,
        serde_wasm_bindgen::to_value(payload)?,
    )
    .await?;

    Ok(())
}

/// Listen to an event from the backend.
///
/// The returned Future will automatically clean up it's underlying event listener when dropped, so no manual unlisten function needs to be called.
/// See [Differences to the JavaScript API](../index.html#differences-to-the-javascript-api) for details.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_api::event::listen;
/// use web_sys::console;
/// use futures::StreamExt;
///
/// let events = listen::<String>("error", EventTarget::Any);
///
/// while let Some(event) = events.next().await {
///     console::log_1(&format!("Got error in window {}, payload: {}", event.window_label, event.payload).into());
/// }
/// ```
#[inline(always)]
pub async fn listen<T>(event: &str) -> crate::Result<impl Stream<Item = Event<T>>>
where
    T: DeserializeOwned + 'static,
{
    let (tx, rx) = mpsc::unbounded::<Event<T>>();

    let closure = Closure::<dyn FnMut(JsValue)>::new(move |raw| {
        let _ = tx.unbounded_send(serde_wasm_bindgen::from_value(raw).unwrap());
    });
    let unlisten = inner::listen(
        event,
        &closure,
        serde_wasm_bindgen::to_value(&Options {
            target: EventTarget::Any,
        })?,
    )
    .await?;
    closure.forget();

    Ok(Listen {
        rx,
        unlisten: js_sys::Function::from(unlisten),
    })
}

/// Listen to an event from the backend.
///
/// The returned Future will automatically clean up it's underlying event listener when dropped, so no manual unlisten function needs to be called.
/// See [Differences to the JavaScript API](../index.html#differences-to-the-javascript-api) for details.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_api::event::{EventTarget, listen_to};
/// use web_sys::console;
/// use futures::StreamExt;
///
/// let events = listen_to::<String>("error", EventTarget::Any);
///
/// while let Some(event) = events.next().await {
///     console::log_1(&format!("Got error in window {}, payload: {}", event.window_label, event.payload).into());
/// }
/// ```
#[inline(always)]
pub async fn listen_to<T>(
    event: &str,
    target: EventTarget,
) -> crate::Result<impl Stream<Item = Event<T>>>
where
    T: DeserializeOwned + 'static,
{
    let (tx, rx) = mpsc::unbounded::<Event<T>>();

    let closure = Closure::<dyn FnMut(JsValue)>::new(move |raw| {
        let _ = tx.unbounded_send(serde_wasm_bindgen::from_value(raw).unwrap());
    });
    let unlisten = inner::listen(
        event,
        &closure,
        serde_wasm_bindgen::to_value(&Options { target })?,
    )
    .await?;
    closure.forget();

    Ok(Listen {
        rx,
        unlisten: js_sys::Function::from(unlisten),
    })
}

pub(crate) struct Listen<T> {
    pub rx: mpsc::UnboundedReceiver<T>,
    pub unlisten: js_sys::Function,
}

impl<T> Drop for Listen<T> {
    fn drop(&mut self) {
        log::debug!("Calling unlisten for listen callback");
        self.unlisten.call0(&wasm_bindgen::JsValue::NULL).unwrap();
    }
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

/// Listen to an one-off event from the backend.
///
/// The returned Future will automatically clean up it's underlying event listener when dropped, so no manual unlisten function needs to be called.
/// See [Differences to the JavaScript API](../index.html#differences-to-the-javascript-api) for details.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_api::event::once;
/// use serde::Deserialize;
/// use web_sys::console;
///
/// #[derive(Deserialize)]
/// interface LoadedPayload {
///   logged_in: bool,
///   token: String
/// }
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// const event = once::<LoadedPayload>("loaded").await?;
///
/// console::log_1!(&format!("App is loaded, loggedIn: {}, token: {}", event.payload.logged_in, event.payload.token).into());
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn once<T>(event: &str) -> crate::Result<Event<T>>
where
    T: DeserializeOwned + 'static,
{
    let (tx, rx) = oneshot::channel::<Event<T>>();

    let closure: Closure<dyn FnMut(JsValue)> = Closure::once(move |raw| {
        let _ = tx.send(serde_wasm_bindgen::from_value(raw).unwrap());
    });
    let unlisten = inner::once(
        event,
        &closure,
        serde_wasm_bindgen::to_value(&Options {
            target: EventTarget::Any,
        })?,
    )
    .await?;
    closure.forget();

    let fut = Once {
        rx,
        unlisten: js_sys::Function::from(unlisten),
    };

    fut.await
}

/// Listen to an one-off event from the backend.
///
/// The returned Future will automatically clean up it's underlying event listener when dropped, so no manual unlisten function needs to be called.
/// See [Differences to the JavaScript API](../index.html#differences-to-the-javascript-api) for details.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_api::event::{EventTarget, once_to};
/// use serde::Deserialize;
/// use web_sys::console;
///
/// #[derive(Deserialize)]
/// interface LoadedPayload {
///   logged_in: bool,
///   token: String
/// }
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// const event = once_to::<LoadedPayload>("loaded", EventTarget::Any).await?;
///
/// console::log_1!(&format!("App is loaded, loggedIn: {}, token: {}", event.payload.logged_in, event.payload.token).into());
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn once_to<T>(event: &str, target: EventTarget) -> crate::Result<Event<T>>
where
    T: DeserializeOwned + 'static,
{
    let (tx, rx) = oneshot::channel::<Event<T>>();

    let closure: Closure<dyn FnMut(JsValue)> = Closure::once(move |raw| {
        let _ = tx.send(serde_wasm_bindgen::from_value(raw).unwrap());
    });
    let unlisten = inner::once(
        event,
        &closure,
        serde_wasm_bindgen::to_value(&Options { target })?,
    )
    .await?;
    closure.forget();

    let fut = Once {
        rx,
        unlisten: js_sys::Function::from(unlisten),
    };

    fut.await
}

pub(crate) struct Once<T> {
    pub rx: oneshot::Receiver<Event<T>>,
    pub unlisten: js_sys::Function,
}

impl<T> Drop for Once<T> {
    fn drop(&mut self) {
        self.rx.close();
        log::debug!("Calling unlisten for once callback");
        self.unlisten.call0(&wasm_bindgen::JsValue::NULL).unwrap();
    }
}

impl<T> Future for Once<T> {
    type Output = crate::Result<Event<T>>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.rx.poll_unpin(cx).map_err(Into::into)
    }
}

pub(crate) mod inner {
    use wasm_bindgen::{
        prelude::{wasm_bindgen, Closure},
        JsValue,
    };

    #[wasm_bindgen(module = "/src/event.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub async fn emit(event: &str, payload: JsValue) -> Result<(), JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn emitTo(target: JsValue, event: &str, payload: JsValue) -> Result<(), JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn listen(
            event: &str,
            handler: &Closure<dyn FnMut(JsValue)>,
            options: JsValue,
        ) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn once(
            event: &str,
            handler: &Closure<dyn FnMut(JsValue)>,
            options: JsValue,
        ) -> Result<JsValue, JsValue>;
    }
}
