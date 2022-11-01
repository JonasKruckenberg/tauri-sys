use wasm_bindgen::JsValue;

#[cfg(feature = "app")]
pub mod app;
#[cfg(feature = "clipboard")]
pub mod clipboard;
#[cfg(feature = "event")]
pub mod event;
#[cfg(feature = "mocks")]
pub mod mocks;
#[cfg(feature = "tauri")]
pub mod tauri;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Serde(#[from] serde_wasm_bindgen::Error),
    #[error("{0:?}")]
    Other(JsValue)
}

pub(crate) type Result<T> = std::result::Result<T, Error>;