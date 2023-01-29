use std::path::PathBuf;
use wasm_bindgen::JsValue;

#[derive(Clone, Eq, PartialEq, Debug, thiserror::Error)]
pub enum Error {
    #[error("TODO.")]
    Binding(String),
    #[error("TODO.")]
    Serde(String),
    #[cfg(any(feature = "event", feature = "window"))]
    #[error("TODO.")]
    OneshotCanceled(#[from] futures::channel::oneshot::Canceled),
    #[cfg(feature = "fs")]
    #[error("could not convert path to string")]
    Utf8(PathBuf),
}

impl From<serde_wasm_bindgen::Error> for Error {
    fn from(e: serde_wasm_bindgen::Error) -> Self {
        Self::Serde(e.to_string())
    }
}

impl From<JsValue> for Error {
    fn from(e: JsValue) -> Self {
        Self::Binding(format!("{:?}", e))
    }
}
