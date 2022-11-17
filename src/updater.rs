use serde::Deserialize;
use wasm_bindgen::{prelude::Closure, JsValue};

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateManifest {
    pub body: String,
    pub date: String,
    pub version: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateResult {
    pub manifest: Option<UpdateManifest>,
    pub should_update: bool,
}

#[derive(Deserialize)]
struct UpdateStatusResult {
    error: Option<String>,
    status: UpdateStatus,
}

#[derive(Deserialize)]
pub enum UpdateStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "DONE")]
    Done,
    #[serde(rename = "UPTODATE")]
    UpToDate,
}

/// Checks if an update is available.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::updater::check_update;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let update = check_update().await?;
/// // now run installUpdate() if needed
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn check_update() -> crate::Result<UpdateResult> {
    let raw = inner::checkUpdate().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Install the update if there's one available.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::updater::{check_update, install_update};
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let update = check_update().await?;
///
/// if update.should_update {
///     log::info("Installing update {:?}", update.manifest);
///     install_update().await?;
/// }
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn install_update() -> crate::Result<()> {
    inner::installUpdate().await?;
    Ok(())
}

/// Listen to an updater event.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::updater::on_updater_event;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let unlisten = on_updater_event(|event| {
///     log::debug!("Updater event {:?}", event);
/// }).await?;
///
/// // you need to call unlisten if your handler goes out of scope e.g. the component is unmounted
/// unlisten();
/// # Ok(())
/// # }
/// ```
/// Note that removing the listener is required if your listener goes out of scope e.g. the component is unmounted.
#[inline(always)]
pub async fn on_updater_event<H>(mut handler: H) -> crate::Result<impl FnOnce()>
where
    H: FnMut(Result<UpdateStatus, String>) + 'static,
{
    let closure = Closure::<dyn FnMut(JsValue)>::new(move |raw| {
        let raw: UpdateStatusResult = serde_wasm_bindgen::from_value(raw).unwrap();

        let result = if let Some(error) = raw.error {
            Err(error)
        } else {
            Ok(raw.status)
        };

        (handler)(result)
    });

    let unlisten = inner::onUpdaterEvent(&closure).await?;

    closure.forget();

    let unlisten = js_sys::Function::from(unlisten);
    Ok(move || {
        unlisten.call0(&wasm_bindgen::JsValue::NULL).unwrap();
    })
}

mod inner {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/src/updater.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub async fn checkUpdate() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn installUpdate() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn onUpdaterEvent(
            handler: &Closure<dyn FnMut(JsValue)>,
        ) -> Result<JsValue, JsValue>;
    }
}
