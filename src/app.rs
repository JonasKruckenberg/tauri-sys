use semver::Version;

/// Gets the application name.
/// 
/// # Example
/// 
/// ```typescript
/// import { getName } from '@tauri-apps/api/app';
/// const appName = await getName();
/// ```
#[inline(always)]
pub async fn get_name() -> String {
    inner::getName().await.as_string().unwrap()
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
pub async fn get_version() -> Version {
    Version::parse(&inner::getVersion().await.as_string().unwrap()).unwrap()
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
pub async fn get_tauri_version() -> Version {
    Version::parse(&inner::getTauriVersion().await.as_string().unwrap()).unwrap()
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
pub async fn show() {
    inner::show().await;
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
pub async fn hide() {
    inner::hide().await;
}

mod inner {
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
    
    #[wasm_bindgen(module = "/dist/app.js")]
    extern "C" {
        pub async fn getName() -> JsValue;
        pub async fn getTauriVersion() -> JsValue;
        pub async fn getVersion() -> JsValue;
        pub async fn hide();
        pub async fn show();
    }
}