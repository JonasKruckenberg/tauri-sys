use wasm_bindgen::JsValue;

#[cfg(feature = "app")]
pub mod app;
#[cfg(feature = "clipboard")]
pub mod clipboard;
#[cfg(feature = "event")]
pub mod event;
#[cfg(feature = "mocks")]
pub mod mocks;
#[cfg(feature = "process")]
pub mod process;
#[cfg(feature = "tauri")]
pub mod tauri;
#[cfg(feature = "window")]
pub mod window;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Serde(#[from] serde_wasm_bindgen::Error),
    #[error("Unknown Theme \"{0}\". Expected one of \"light\",\"dark\"")]
    UnknownTheme(String),
    #[error("Invalid Url {0}")]
    InvalidUrl(#[from] url::ParseError),
    #[error("Invalid Version {0}")]
    InvalidVersion(#[from] semver::Error),
    #[error("{0:?}")]
    Other(JsValue),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
