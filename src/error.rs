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

#[derive(Debug)]
pub enum Deserialize {
    /// Value could not be stringified
    Stringify(wasm_bindgen::JsValue),
    /// JSON could not be deserialized
    Deserialize(serde_json::Error),
}

impl From<wasm_bindgen::JsValue> for Deserialize {
    fn from(value: wasm_bindgen::JsValue) -> Self {
        Self::Stringify(value)
    }
}

impl From<serde_json::Error> for Deserialize {
    fn from(value: serde_json::Error) -> Self {
        Self::Deserialize(value)
    }
}
