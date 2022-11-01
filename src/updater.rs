/// Install the update if there's one available.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_api::updater::install_update;
///
/// install_update().await;
/// ```
#[inline(always)]
pub async fn install_update() {
    inner::installUpdate().await
}

/// Checks if an update is available.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_api::clipboard::{write_text, read_text};
///
/// write_text("Tauri is awesome!").await;
/// assert_eq!(read_text().await, "Tauri is awesome!");
/// ```
///
/// @returns A promise indicating the success or failure of the operation.
#[inline(always)]
pub async fn check_update() {
    inner::checkUpdate().await
}

mod inner {
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

    #[wasm_bindgen(module = "/dist/updater.js")]
    extern "C" {
        pub async fn installUpdate();
        pub async fn checkUpdate();
    }
}
