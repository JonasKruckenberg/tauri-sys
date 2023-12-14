use std::path::PathBuf;
use wasm_bindgen::JsValue;

#[derive(Clone, Eq, PartialEq, Debug, thiserror::Error)]
pub enum Error {
    #[error("Command returned Error: {0}")]
    Command(String),
    #[error("Failed to parse JSON: {0}")]
    Serde(String),
    #[cfg(any(feature = "event", feature = "window"))]
    #[error("Oneshot cancelled: {0}")]
    OneshotCanceled(#[from] futures::channel::oneshot::Canceled),
    #[cfg(feature = "fs")]
    #[error("Could not convert path to string")]
    Utf8(PathBuf),
}

impl From<serde_wasm_bindgen::Error> for Error {
    fn from(e: serde_wasm_bindgen::Error) -> Self {
        Self::Serde(e.to_string())
    }
}

impl From<JsValue> for Error {
    fn from(e: JsValue) -> Self {
        Self::Command(format!("{:?}", e))
    }
}
