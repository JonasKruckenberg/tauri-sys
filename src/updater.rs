use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UpdateManifest {
    pub body: String,
    pub date: String,
    pub version: String
}

#[derive(Deserialize, Debug)]
pub struct UpdateResult {
    pub manifest: Option<UpdateManifest>,
    pub should_update: bool
}

#[derive(Deserialize)]
pub enum UpdateStatus {
    PENDING,
    ERROR,
    DONE,
    UPTODATE
}

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
/// use tauri_api::updater::{check_update, UpdateResult};
///
/// let update: UpdateResult = check_update().await;
/// ```
#[inline(always)]
pub async fn check_update() -> UpdateResult {
    let update = inner::checkUpdate().await;
    serde_wasm_bindgen::from_value(update).unwrap()
}

mod inner {
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

    #[wasm_bindgen(module = "/dist/updater.js")]
    extern "C" {
        pub async fn installUpdate();
        pub async fn checkUpdate() -> JsValue;
    }
}
