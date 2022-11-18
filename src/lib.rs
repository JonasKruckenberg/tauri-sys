use wasm_bindgen::JsValue;

#[cfg(feature = "app")]
pub mod app;
#[cfg(feature = "clipboard")]
pub mod clipboard;
#[cfg(feature = "dialog")]
pub mod dialog;
#[cfg(feature = "event")]
pub mod event;
#[cfg(feature = "global_shortcut")]
pub mod global_shortcut;
#[cfg(feature = "mocks")]
pub mod mocks;
#[cfg(feature = "notification")]
pub mod notification;
#[cfg(feature = "os")]
pub mod os;
#[cfg(feature = "path")]
pub mod path;
#[cfg(feature = "process")]
pub mod process;
#[cfg(feature = "tauri")]
pub mod tauri;
#[cfg(feature = "updater")]
pub mod updater;
#[cfg(feature = "window")]
pub mod window;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Serde(String),
    #[error("Unknown Theme \"{0}\". Expected one of \"light\",\"dark\"")]
    UnknownTheme(String),
    #[error("{0}")]
    Other(String),
    #[cfg(feature = "tauri")]
    #[error("Invalid Url {0}")]
    InvalidUrl(#[from] url::ParseError),
    #[cfg(feature = "app")]
    #[error("Invalid Version {0}")]
    InvalidVersion(#[from] semver::Error),
    #[cfg(any(feature = "event", feature = "updater", feature = "window"))]
    #[error(transparent)]
    Recv(#[from] futures::channel::oneshot::Canceled)
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
