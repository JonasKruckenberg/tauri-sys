use crate::event::Event;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Display;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};

#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TitleBarStyle {
    Visible,
    Transparent,
    Overlay,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserAttentionType {
    Critical = 1,
    Informational,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Position {
    Physical(PhysicalPosition),
    Logical(LogicalPosition),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Size {
    Physical(PhysicalSize),
    Logical(LogicalSize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CursorIcon {
    Default,
    Crosshair,
    Hand,
    Arrow,
    Move,
    Text,
    Wait,
    Help,
    Progress,
    // something cannot be done
    NotAllowed,
    ContextMenu,
    Cell,
    VerticalText,
    Alias,
    Copy,
    NoDrop,
    // something can be grabbed
    Grab,
    /// something is grabbed
    Grabbing,
    AllScroll,
    ZoomIn,
    ZoomOut,
    // edge is to be moved
    EResize,
    NResize,
    NeResize,
    NwResize,
    SResize,
    SeResize,
    SwResize,
    WResize,
    EwResize,
    NsResize,
    NeswResize,
    NwseResize,
    ColResize,
    RowResize,
}

impl Display for CursorIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CursorIcon::Default => write!(f, "default"),
            CursorIcon::Crosshair => write!(f, "crosshair"),
            CursorIcon::Hand => write!(f, "hand"),
            CursorIcon::Arrow => write!(f, "arrow"),
            CursorIcon::Move => write!(f, "move"),
            CursorIcon::Text => write!(f, "text"),
            CursorIcon::Wait => write!(f, "wait"),
            CursorIcon::Help => write!(f, "help"),
            CursorIcon::Progress => write!(f, "progress"),
            CursorIcon::NotAllowed => write!(f, "notAllowed"),
            CursorIcon::ContextMenu => write!(f, "contextMenu"),
            CursorIcon::Cell => write!(f, "cell"),
            CursorIcon::VerticalText => write!(f, "verticalText"),
            CursorIcon::Alias => write!(f, "alias"),
            CursorIcon::Copy => write!(f, "copy"),
            CursorIcon::NoDrop => write!(f, "noDrop"),
            CursorIcon::Grab => write!(f, "grab"),
            CursorIcon::Grabbing => write!(f, "grabbing"),
            CursorIcon::AllScroll => write!(f, "allScroll"),
            CursorIcon::ZoomIn => write!(f, "zoomIn"),
            CursorIcon::ZoomOut => write!(f, "zoomOut"),
            CursorIcon::EResize => write!(f, "eResize"),
            CursorIcon::NResize => write!(f, "nResize"),
            CursorIcon::NeResize => write!(f, "neResize"),
            CursorIcon::NwResize => write!(f, "nwResize"),
            CursorIcon::SResize => write!(f, "sResize"),
            CursorIcon::SeResize => write!(f, "seResize"),
            CursorIcon::SwResize => write!(f, "swResize"),
            CursorIcon::WResize => write!(f, "wResize"),
            CursorIcon::EwResize => write!(f, "ewResize"),
            CursorIcon::NsResize => write!(f, "nsResize"),
            CursorIcon::NeswResize => write!(f, "neswResize"),
            CursorIcon::NwseResize => write!(f, "nwseResize"),
            CursorIcon::ColResize => write!(f, "colResize"),
            CursorIcon::RowResize => write!(f, "rowResize"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WebviewWindow(inner::WebviewWindow);

impl WebviewWindow {
    pub fn new(label: &str, options: ()) -> Self {
        Self(inner::WebviewWindow::new(label, options))
    }

    pub fn get_by_label(label: &str) -> Option<Self> {
        inner::WebviewWindow::getByLabel(label).map(Self)
    }

    pub fn label(&self) -> String {
        self.0.label()
    }

    pub async fn scale_factor(&self) -> f64 {
        self.0.scaleFactor().await.as_f64().unwrap()
    }

    pub async fn inner_position(&self) -> PhysicalPosition {
        PhysicalPosition(self.0.innerPosition().await.unchecked_into())
    }

    pub async fn outer_position(&self) -> PhysicalPosition {
        PhysicalPosition(self.0.outerPosition().await.unchecked_into())
    }

    pub async fn inner_size(&self) -> PhysicalSize {
        PhysicalSize(self.0.innerSize().await.unchecked_into())
    }

    pub async fn outer_size(&self) -> PhysicalSize {
        PhysicalSize(self.0.outerSize().await.unchecked_into())
    }

    pub async fn is_fullscreen(&self) -> bool {
        self.0.isFullscreen().await.as_bool().unwrap()
    }

    pub async fn is_maximized(&self) -> bool {
        self.0.isMaximized().await.as_bool().unwrap()
    }

    pub async fn is_decorated(&self) -> bool {
        self.0.isDecorated().await.as_bool().unwrap()
    }

    pub async fn is_resizable(&self) -> bool {
        self.0.isResizable().await.as_bool().unwrap()
    }

    pub async fn is_visible(&self) -> bool {
        self.0.isVisible().await.as_bool().unwrap()
    }

    pub async fn theme(&self) -> Theme {
        match self.0.theme().await.as_string().unwrap().as_str() {
            "light" => Theme::Light,
            "dark" => Theme::Dark,
            _ => panic!("Unknown theme"),
        }
    }

    pub async fn center(&self) {
        self.0.center().await;
    }

    pub async fn request_user_attention(&self, request_type: UserAttentionType) {
        self.0.requestUserAttention(request_type as u32).await;
    }

    pub async fn set_resizable(&self, resizable: bool) {
        self.0.setResizable(resizable).await;
    }

    pub async fn set_title(&self, title: impl AsRef<str>) {
        self.0.setTitle(title.as_ref()).await;
    }

    pub async fn maximize(&self) {
        self.0.maximize().await;
    }

    pub async fn unmaximize(&self) {
        self.0.unmaximize().await;
    }

    pub async fn toggle_maximize(&self) {
        self.0.toggleMaximize().await;
    }

    pub async fn minimize(&self) {
        self.0.minimize().await;
    }

    pub async fn unminimize(&self) {
        self.0.unminimize().await;
    }

    pub async fn show(&self) {
        self.0.show().await;
    }

    pub async fn hide(&self) {
        self.0.hide().await;
    }

    pub async fn close(&self) {
        self.0.close().await;
    }

    pub async fn set_decorations(&self, decorations: bool) {
        self.0.setDecorations(decorations).await;
    }

    pub async fn set_always_on_top(&self, always_on_top: bool) {
        self.0.setAlwaysOnTop(always_on_top).await;
    }

    pub async fn set_size(&self, size: Size) {
        match size {
            Size::Physical(size) => self.0.setSizePhysical(size.0).await,
            Size::Logical(size) => self.0.setSizeLogical(size.0).await,
        };
    }

    pub async fn set_min_size(&self, size: Option<Size>) {
        match size {
            None => self.0.setMinSizePhysical(None).await,
            Some(Size::Physical(size)) => self.0.setMinSizePhysical(Some(size.0)).await,
            Some(Size::Logical(size)) => self.0.setMinSizeLogical(Some(size.0)).await,
        }
    }

    pub async fn set_max_size(&self, size: Option<Size>) {
        match size {
            None => self.0.setMaxSizePhysical(None).await,
            Some(Size::Physical(size)) => self.0.setMaxSizePhysical(Some(size.0)).await,
            Some(Size::Logical(size)) => self.0.setMaxSizeLogical(Some(size.0)).await,
        }
    }

    pub async fn set_position(&self, position: Position) {
        match position {
            Position::Physical(pos) => self.0.setPositionPhysical(pos.0).await,
            Position::Logical(pos) => self.0.setPositionLogical(pos.0).await,
        }
    }

    pub async fn set_fullscreen(&self, fullscreen: bool) {
        self.0.setFullscreen(fullscreen).await;
    }

    pub async fn set_focus(&self) {
        self.0.setFocus().await;
    }

    pub async fn set_icon(&self, icon: &[u8]) {
        self.0.setIcon(icon).await;
    }

    pub async fn set_skip_taskbar(&self, skip: bool) {
        self.0.setSkipTaskbar(skip).await;
    }

    pub async fn set_cursor_grab(&self, grab: bool) {
        self.0.setCursorGrab(grab).await;
    }

    pub async fn set_cursor_visible(&self, visible: bool) {
        self.0.setCursorVisible(visible).await;
    }

    pub async fn set_cursor_icon(&self, icon: CursorIcon) {
        self.0.setCursorIcon(&icon.to_string()).await;
    }

    pub async fn set_cursor_position(&self, position: Position) {
        match position {
            Position::Physical(pos) => self.0.setCursorPositionPhysical(pos.0).await,
            Position::Logical(pos) => self.0.setCursorPositionLogical(pos.0).await,
        }
    }

    pub async fn set_ignore_cursor_events(&self, ignore: bool) {
        self.0.setIgnoreCursorEvents(ignore).await;
    }

    pub async fn start_dragging(&self) {
        self.0.startDragging().await;
    }

    #[inline(always)]
    pub async fn emit<T: Serialize>(&self, event: &str, payload: &T) {
        self.0
            .emit(event, serde_wasm_bindgen::to_value(payload).unwrap())
            .await;
    }

    #[inline(always)]
    pub async fn listen<T, H>(&self, event: &str, mut handler: H) -> impl FnOnce()
    where
        T: DeserializeOwned,
        H: FnMut(Event<T>) + 'static,
    {
        let closure = Closure::<dyn FnMut(JsValue)>::new(move |raw| {
            (handler)(serde_wasm_bindgen::from_value(raw).unwrap())
        });

        let unlisten = self.0.listen(event, &closure).await;

        closure.forget();

        let unlisten = js_sys::Function::from(unlisten);
        move || {
            unlisten.call0(&wasm_bindgen::JsValue::NULL).unwrap();
        }
    }

    #[inline(always)]
    pub async fn once<T, H>(&self, event: &str, mut handler: H) -> impl FnOnce()
    where
        T: DeserializeOwned,
        H: FnMut(Event<T>) + 'static,
    {
        let closure = Closure::<dyn FnMut(JsValue)>::new(move |raw| {
            (handler)(serde_wasm_bindgen::from_value(raw).unwrap())
        });

        let unlisten = self.0.once(event, &closure).await;

        closure.forget();

        let unlisten = js_sys::Function::from(unlisten);
        move || {
            unlisten.call0(&wasm_bindgen::JsValue::NULL).unwrap();
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LogicalPosition(inner::LogicalPosition);

impl LogicalPosition {
    pub fn new(x: u32, y: u32) -> Self {
        Self(inner::LogicalPosition::new(x, y))
    }

    pub fn x(&self) -> u32 {
        self.0.x()
    }
    pub fn set_x(&self, x: u32) {
        self.0.set_x(x)
    }
    pub fn y(&self) -> u32 {
        self.0.y()
    }
    pub fn set_y(&self, y: u32) {
        self.0.set_y(y)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalPosition(inner::PhysicalPosition);

impl PhysicalPosition {
    pub fn new(x: u32, y: u32) -> Self {
        Self(inner::PhysicalPosition::new(x, y))
    }

    pub fn to_logical(self, scale_factor: u32) -> LogicalPosition {
        LogicalPosition(self.0.toLogical(scale_factor))
    }

    pub fn x(&self) -> u32 {
        self.0.x()
    }
    pub fn set_x(&self, x: u32) {
        self.0.set_x(x)
    }
    pub fn y(&self) -> u32 {
        self.0.y()
    }
    pub fn set_y(&self, y: u32) {
        self.0.set_y(y)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LogicalSize(inner::LogicalSize);

impl LogicalSize {
    pub fn new(x: u32, y: u32) -> Self {
        Self(inner::LogicalSize::new(x, y))
    }

    pub fn width(&self) -> u32 {
        self.0.width()
    }
    pub fn set_width(&self, x: u32) {
        self.0.set_width(x)
    }
    pub fn height(&self) -> u32 {
        self.0.height()
    }
    pub fn set_height(&self, y: u32) {
        self.0.set_height(y)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalSize(inner::PhysicalSize);

impl PhysicalSize {
    pub fn new(x: u32, y: u32) -> Self {
        Self(inner::PhysicalSize::new(x, y))
    }

    pub fn to_logical(self, scale_factor: u32) -> LogicalSize {
        LogicalSize(self.0.toLogical(scale_factor))
    }

    pub fn width(&self) -> u32 {
        self.0.width()
    }
    pub fn set_width(&self, x: u32) {
        self.0.set_width(x)
    }
    pub fn height(&self) -> u32 {
        self.0.height()
    }
    pub fn set_height(&self, y: u32) {
        self.0.set_height(y)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Monitor(JsValue);

impl Monitor {
    pub fn name(&self) -> Option<String> {
        let raw = js_sys::Reflect::get(&self.0, &JsValue::from_str("name")).unwrap();

        raw.as_string()
    }

    pub fn size(&self) -> PhysicalSize {
        let raw = js_sys::Reflect::get(&self.0, &JsValue::from_str("size")).unwrap();

        PhysicalSize(raw.unchecked_into())
    }

    pub fn position(&self) -> PhysicalPosition {
        let raw = js_sys::Reflect::get(&self.0, &JsValue::from_str("position")).unwrap();

        PhysicalPosition(raw.unchecked_into())
    }

    pub fn scale_factor(&self) -> u32 {
        let raw = js_sys::Reflect::get(&self.0, &JsValue::from_str("size"))
            .unwrap()
            .as_f64()
            .unwrap();

        raw as u32
    }
}

pub fn current_window() -> WebviewWindow {
    WebviewWindow(inner::getCurrent())
}

pub fn all_windows() -> Vec<WebviewWindow> {
    inner::getAll().into_iter().map(WebviewWindow).collect()
}

pub async fn current_monitor() -> Monitor {
    Monitor(inner::currentMonitor().await)
}
pub async fn primary_monitor() -> Monitor {
    Monitor(inner::primaryMonitor().await)
}

#[derive(Debug, Clone)]
pub struct AvailableMonitors {
    idx: u32,
    array: js_sys::Array,
}

impl Iterator for AvailableMonitors {
    type Item = Monitor;

    fn next(&mut self) -> Option<Self::Item> {
        let raw = self.array.get(self.idx);

        if raw.is_undefined() {
            None
        } else {
            let monitor = Monitor(raw);
            self.idx += 1;

            Some(monitor)
        }
    }
}

pub async fn available_monitors() -> AvailableMonitors {
    AvailableMonitors {
        idx: 0,
        array: inner::availableMonitors().await.unchecked_into()
    }
}

mod inner {
    use wasm_bindgen::{
        prelude::{wasm_bindgen, Closure},
        JsValue,
    };

    #[wasm_bindgen(module = "/dist/window.js")]
    extern "C" {
        #[derive(Debug, Clone, PartialEq)]
        pub type LogicalPosition;
        #[wasm_bindgen(constructor)]
        pub fn new(x: u32, y: u32) -> LogicalPosition;
        #[wasm_bindgen(method, getter)]
        pub fn x(this: &LogicalPosition) -> u32;
        #[wasm_bindgen(method, setter)]
        pub fn set_x(this: &LogicalPosition, x: u32);
        #[wasm_bindgen(method, getter)]
        pub fn y(this: &LogicalPosition) -> u32;
        #[wasm_bindgen(method, setter)]
        pub fn set_y(this: &LogicalPosition, y: u32);
    }

    #[wasm_bindgen(module = "/dist/window.js")]
    extern "C" {
        #[derive(Debug, Clone, PartialEq)]
        pub type PhysicalPosition;
        #[wasm_bindgen(constructor)]
        pub fn new(x: u32, y: u32) -> PhysicalPosition;
        #[wasm_bindgen(method)]
        pub fn toLogical(this: &PhysicalPosition, scaleFactor: u32) -> LogicalPosition;
        #[wasm_bindgen(method, getter)]
        pub fn x(this: &PhysicalPosition) -> u32;
        #[wasm_bindgen(method, setter)]
        pub fn set_x(this: &PhysicalPosition, x: u32);
        #[wasm_bindgen(method, getter)]
        pub fn y(this: &PhysicalPosition) -> u32;
        #[wasm_bindgen(method, setter)]
        pub fn set_y(this: &PhysicalPosition, y: u32);
    }

    #[wasm_bindgen(module = "/dist/window.js")]
    extern "C" {
        #[derive(Debug, Clone, PartialEq)]
        pub type LogicalSize;
        #[wasm_bindgen(constructor)]
        pub fn new(width: u32, height: u32) -> LogicalSize;
        #[wasm_bindgen(method, getter)]
        pub fn width(this: &LogicalSize) -> u32;
        #[wasm_bindgen(method, setter)]
        pub fn set_width(this: &LogicalSize, width: u32);
        #[wasm_bindgen(method, getter)]
        pub fn height(this: &LogicalSize) -> u32;
        #[wasm_bindgen(method, setter)]
        pub fn set_height(this: &LogicalSize, height: u32);
    }

    #[wasm_bindgen(module = "/dist/window.js")]
    extern "C" {
        #[derive(Debug, Clone, PartialEq)]
        pub type PhysicalSize;
        #[wasm_bindgen(constructor)]
        pub fn new(width: u32, height: u32) -> PhysicalSize;
        #[wasm_bindgen(method)]
        pub fn toLogical(this: &PhysicalSize, scaleFactor: u32) -> LogicalSize;
        #[wasm_bindgen(method, getter)]
        pub fn width(this: &PhysicalSize) -> u32;
        #[wasm_bindgen(method, setter)]
        pub fn set_width(this: &PhysicalSize, width: u32);
        #[wasm_bindgen(method, getter)]
        pub fn height(this: &PhysicalSize) -> u32;
        #[wasm_bindgen(method, setter)]
        pub fn set_height(this: &PhysicalSize, height: u32);
    }

    #[wasm_bindgen(module = "/dist/window.js")]
    extern "C" {
        #[derive(Debug, Clone, PartialEq)]
        pub type WebviewWindowHandle;
        #[wasm_bindgen(constructor)]
        pub fn new(label: &str) -> WebviewWindowHandle;
        #[wasm_bindgen(method)]
        pub async fn listen(
            this: &WebviewWindowHandle,
            event: &str,
            handler: &Closure<dyn FnMut(JsValue)>,
        ) -> JsValue;
        #[wasm_bindgen(method)]
        pub async fn once(
            this: &WebviewWindowHandle,
            event: &str,
            handler: &Closure<dyn FnMut(JsValue)>,
        ) -> JsValue;
        #[wasm_bindgen(method)]
        pub async fn emit(this: &WebviewWindowHandle, event: &str, payload: JsValue);
    }

    #[wasm_bindgen(module = "/dist/window.js")]
    extern "C" {
        #[wasm_bindgen(extends = WebviewWindowHandle)]
        #[derive(Debug, Clone, PartialEq)]
        pub type WindowManager;
        #[wasm_bindgen(constructor)]
        pub fn new(label: &str) -> WindowManager;
        #[wasm_bindgen(method, getter)]
        pub fn label(this: &WindowManager) -> String;
        #[wasm_bindgen(method)]
        pub async fn scaleFactor(this: &WindowManager) -> JsValue;
        #[wasm_bindgen(method)]
        pub async fn innerPosition(this: &WindowManager) -> JsValue;
        #[wasm_bindgen(method)]
        pub async fn outerPosition(this: &WindowManager) -> JsValue;
        #[wasm_bindgen(method)]
        pub async fn innerSize(this: &WindowManager) -> JsValue;
        #[wasm_bindgen(method)]
        pub async fn outerSize(this: &WindowManager) -> JsValue;
        #[wasm_bindgen(method)]
        pub async fn isFullscreen(this: &WindowManager) -> JsValue;
        #[wasm_bindgen(method)]
        pub async fn isMaximized(this: &WindowManager) -> JsValue;
        #[wasm_bindgen(method)]
        pub async fn isDecorated(this: &WindowManager) -> JsValue;
        #[wasm_bindgen(method)]
        pub async fn isResizable(this: &WindowManager) -> JsValue;
        #[wasm_bindgen(method)]
        pub async fn isVisible(this: &WindowManager) -> JsValue;
        #[wasm_bindgen(method)]
        pub async fn theme(this: &WindowManager) -> JsValue;
        #[wasm_bindgen(method)]
        pub async fn center(this: &WindowManager);
        #[wasm_bindgen(method)]
        pub async fn requestUserAttention(this: &WindowManager, requestType: u32);
        #[wasm_bindgen(method)]
        pub async fn setResizable(this: &WindowManager, resizable: bool);
        #[wasm_bindgen(method)]
        pub async fn setTitle(this: &WindowManager, title: &str);
        #[wasm_bindgen(method)]
        pub async fn maximize(this: &WindowManager);
        #[wasm_bindgen(method)]
        pub async fn unmaximize(this: &WindowManager);
        #[wasm_bindgen(method)]
        pub async fn toggleMaximize(this: &WindowManager);
        #[wasm_bindgen(method)]
        pub async fn minimize(this: &WindowManager);
        #[wasm_bindgen(method)]
        pub async fn unminimize(this: &WindowManager);
        #[wasm_bindgen(method)]
        pub async fn show(this: &WindowManager);
        #[wasm_bindgen(method)]
        pub async fn hide(this: &WindowManager);
        #[wasm_bindgen(method)]
        pub async fn close(this: &WindowManager);
        #[wasm_bindgen(method)]
        pub async fn setDecorations(this: &WindowManager, decorations: bool);
        #[wasm_bindgen(method)]
        pub async fn setAlwaysOnTop(this: &WindowManager, alwaysOnTop: bool);
        #[wasm_bindgen(method, js_name = setSize)]
        pub async fn setSizePhysical(this: &WindowManager, size: PhysicalSize);
        #[wasm_bindgen(method, js_name = setSize)]
        pub async fn setSizeLogical(this: &WindowManager, size: LogicalSize);
        #[wasm_bindgen(method, js_name = setMinSize)]
        pub async fn setMinSizePhysical(this: &WindowManager, size: Option<PhysicalSize>);
        #[wasm_bindgen(method, js_name = setMinSize)]
        pub async fn setMinSizeLogical(this: &WindowManager, size: Option<LogicalSize>);
        #[wasm_bindgen(method, js_name = setMaxSize)]
        pub async fn setMaxSizePhysical(this: &WindowManager, size: Option<PhysicalSize>);
        #[wasm_bindgen(method, js_name = setMinSize)]
        pub async fn setMaxSizeLogical(this: &WindowManager, size: Option<LogicalSize>);
        #[wasm_bindgen(method, js_name = setPosition)]
        pub async fn setPositionPhysical(this: &WindowManager, position: PhysicalPosition);
        #[wasm_bindgen(method, js_name = setPosition)]
        pub async fn setPositionLogical(this: &WindowManager, position: LogicalPosition);
        #[wasm_bindgen(method)]
        pub async fn setFullscreen(this: &WindowManager, fullscreen: bool);
        #[wasm_bindgen(method)]
        pub async fn setFocus(this: &WindowManager);
        #[wasm_bindgen(method)]
        pub async fn setIcon(this: &WindowManager, icon: &[u8]);
        #[wasm_bindgen(method)]
        pub async fn setSkipTaskbar(this: &WindowManager, skip: bool);
        #[wasm_bindgen(method)]
        pub async fn setCursorGrab(this: &WindowManager, grab: bool);
        #[wasm_bindgen(method)]
        pub async fn setCursorVisible(this: &WindowManager, visible: bool);
        #[wasm_bindgen(method)]
        pub async fn setCursorIcon(this: &WindowManager, icon: &str);
        #[wasm_bindgen(method, js_name = setCursorPosition)]
        pub async fn setCursorPositionPhysical(this: &WindowManager, position: PhysicalPosition);
        #[wasm_bindgen(method, js_name = setCursorPosition)]
        pub async fn setCursorPositionLogical(this: &WindowManager, position: LogicalPosition);
        #[wasm_bindgen(method)]
        pub async fn setIgnoreCursorEvents(this: &WindowManager, ignore: bool);
        #[wasm_bindgen(method)]
        pub async fn startDragging(this: &WindowManager);
    }

    #[wasm_bindgen(module = "/dist/window.js")]
    extern "C" {
        #[wasm_bindgen(extends = WindowManager)]
        #[derive(Debug, Clone, PartialEq)]
        pub type WebviewWindow;
        #[wasm_bindgen(constructor)]
        pub fn new(label: &str, options: ()) -> WebviewWindow;
        #[wasm_bindgen(static_method_of = WebviewWindow)]
        pub fn getByLabel(label: &str) -> Option<WebviewWindow>;
    }

    #[wasm_bindgen(module = "/dist/window.js")]
    extern "C" {
        pub fn getCurrent() -> WebviewWindow;
        pub fn getAll() -> Vec<WebviewWindow>;
        pub async fn currentMonitor() -> JsValue;
        pub async fn primaryMonitor() -> JsValue;
        pub async fn availableMonitors() -> JsValue;
    }
}
