use wasm_bindgen::JsValue;


#[derive(Copy, Clone, Eq, PartialEq, Debug, thiserror::Error)]
pub enum Error {
    #[error("TODO.")]
    Binding,
    #[error("TODO.")]
    Serde,
    #[error("TODO.")]
    OneshotCanceled(#[from] futures::channel::oneshot::Canceled)
}

impl From<serde_wasm_bindgen::Error> for Error {
    fn from(e: serde_wasm_bindgen::Error) -> Self {
        Self::Serde
    }
}

impl From<JsValue> for Error {
    fn from(e: JsValue) -> Self {
        Self::Serde
    }
}