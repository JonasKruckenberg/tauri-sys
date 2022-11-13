use semver::Version;

use crate::Error;

/// Gets the application name.
///
/// # Example
///
/// ```typescript
/// import { getName } from '@tauri-apps/api/app';
/// const appName = await getName();
/// ```
#[inline(always)]
pub async fn get_name() -> crate::Result<String> {
    let js_val = inner::getName().await.map_err(Error::Other)?;

    Ok(serde_wasm_bindgen::from_value(js_val)?)
}

/// Gets the application version.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_api::app::get_version;
///     
/// let version = get_version().await;
/// ```
#[inline(always)]
pub async fn get_version() -> crate::Result<Version> {
    let js_val = inner::getVersion().await.map_err(Error::Other)?;

    Ok(serde_wasm_bindgen::from_value(js_val)?)
}

/// Gets the Tauri version.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_app::app:get_tauri_version;
///
/// let version = get_tauri_version().await;
/// ```
#[inline(always)]
pub async fn get_tauri_version() -> crate::Result<Version> {
    let js_val = inner::getTauriVersion().await.map_err(Error::Other)?;

    Ok(serde_wasm_bindgen::from_value(js_val)?)
}

/// Shows the application on macOS. This function does not automatically focuses any app window.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_api::app::show;
///
/// show().await;
/// ```
#[inline(always)]
pub async fn show() -> crate::Result<()> {
    inner::show().await.map_err(Error::Other)
}

/// Hides the application on macOS.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_api::app::hide;
///
/// hide().await;
/// ```
#[inline(always)]
pub async fn hide() -> crate::Result<()> {
    inner::hide().await.map_err(Error::Other)
}

mod inner {
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

    #[wasm_bindgen(module = "/dist/app.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub async fn getName() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn getTauriVersion() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn getVersion() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn hide() -> Result<(), JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn show() -> Result<(), JsValue>;
    }
}
