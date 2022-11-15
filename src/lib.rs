use wasm_bindgen::JsValue;

#[cfg(feature = "app")]
pub mod app;
#[cfg(feature = "clipboard")]
pub mod clipboard;
#[cfg(feature = "dialog")]
pub mod dialog;
#[cfg(feature = "event")]
pub mod event;
#[cfg(feature = "mocks")]
pub mod mocks;
#[cfg(feature = "process")]
pub mod process;
#[cfg(feature = "tauri")]
pub mod tauri;
#[cfg(feature = "updater")]
pub mod updater;
#[cfg(feature = "window")]
pub mod window;
#[cfg(feature = "notification")]
pub mod notification;
#[cfg(feature = "os")]
pub mod os;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Serde(String),
    #[error("Unknown Theme \"{0}\". Expected one of \"light\",\"dark\"")]
    UnknownTheme(String),
    #[error("Invalid Url {0}")]
    InvalidUrl(#[from] url::ParseError),
    #[error("Invalid Version {0}")]
    InvalidVersion(#[from] semver::Error),
    #[error("{0}")]
    Other(String),
}

impl From<serde_wasm_bindgen::Error> for Error {
    fn from(e: serde_wasm_bindgen::Error) -> Self {
        Self::Serde(format!("{:?}", e))
    }
}

impl From<JsValue> for Error {
    fn from(e: JsValue) -> Self {
        Self::Serde(format!("{:?}", e))
    }
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
