use std::fmt::Debug;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Event<T> {
    /// Event name
    pub event: String,
    /// Event identifier used to unlisten
    pub id: u32,
    /// Event payload
    pub payload: T,
    /// The label of the window that emitted this event
    pub window_label: String,
}

impl<T: Debug> Debug for Event<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Event")
            .field("event", &self.event)
            .field("id", &self.id)
            .field("payload", &self.payload)
            .field("window_label", &self.window_label)
            .finish()
    }
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
pub async fn emit<T: Serialize>(event: &str, payload: &T) {
    inner::emit(event, serde_wasm_bindgen::to_value(payload).unwrap()).await
}

/// Listen to an event from the backend.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_api::event::{emit, listen};
///
/// const unlisten = listen::<String>("error", |event| {
///   println!("Got error in window {}, payload: {}", event.window_label, event.payload);
/// }).await;
///
/// // you need to call unlisten if your handler goes out of scope e.g. the component is unmounted
/// unlisten();
/// ```
///
/// @param event Event name. Must include only alphanumeric characters, `-`, `/`, `:` and `_`.
/// @param handler Event handler callback.
/// @returns A promise resolving to a function to unlisten to the event.
///
/// Note that removing the listener is required if your listener goes out of scope e.g. the component is unmounted.
#[inline(always)]
pub async fn listen<T: DeserializeOwned>(event: &str, handler: &dyn Fn(Event<T>)) -> impl FnOnce() {
    let unlisten = inner::listen(event, &|raw| {
        handler(serde_wasm_bindgen::from_value(raw).unwrap())
    })
    .await;

    let unlisten = js_sys::Function::from(unlisten);
    move || {
        unlisten.call0(&wasm_bindgen::JsValue::NULL).unwrap();
    }
}

/// Listen to an one-off event from the backend.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_api::event::once;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// interface LoadedPayload {
///   logged_in: bool,
///   token: String
/// }
/// const unlisten = once::<LoadedPayload>("loaded", |event| {
///     println!("App is loaded, loggedIn: {}, token: {}", event.payload.logged_in, event.payload.token);
/// }).await;
///
/// // you need to call unlisten if your handler goes out of scope e.g. the component is unmounted
/// unlisten();
/// ```
///
/// @param event Event name. Must include only alphanumeric characters, `-`, `/`, `:` and `_`.
/// @returns A promise resolving to a function to unlisten to the event.
///
/// Note that removing the listener is required if your listener goes out of scope e.g. the component is unmounted.
#[inline(always)]
pub async fn once<T: DeserializeOwned>(
    event: &str,
    handler: &mut dyn FnMut(Event<T>),
) -> impl FnOnce() {
    let unlisten = inner::once(event, &mut |raw| {
        handler(serde_wasm_bindgen::from_value(raw).unwrap())
    })
    .await;

    let unlisten = js_sys::Function::from(unlisten);
    move || {
        unlisten.call0(&wasm_bindgen::JsValue::NULL).unwrap();
    }
}

mod inner {
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

    #[wasm_bindgen(module = "/dist/event.js")]
    extern "C" {
        pub async fn emit(event: &str, payload: JsValue);
        pub async fn listen(event: &str, handler: &dyn Fn(JsValue)) -> JsValue;
        pub async fn once(event: &str, handler: &mut dyn FnMut(JsValue)) -> JsValue;
    }
}
