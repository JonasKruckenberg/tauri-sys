/// Gets the clipboard content as plain text.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_api::clipboard::read_text;
///
/// let clipboard_text = read_text().await;
/// ```
#[inline(always)]
pub async fn read_text() -> crate::Result<String> {
    let js_val = inner::readText().await?;

    Ok(serde_wasm_bindgen::from_value(js_val)?)
}

/// Writes plain text to the clipboard.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_api::clipboard::{write_text, read_text};
///
/// write_text("Tauri is awesome!").await;
/// assert_eq!(read_text().await, "Tauri is awesome!");
/// ```
#[inline(always)]
pub async fn write_text(text: &str) -> crate::Result<()> {
    Ok(inner::writeText(text).await?)
}

mod inner {
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

    #[wasm_bindgen(module = "/src/clipboard.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub async fn readText() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn writeText(text: &str) -> Result<(), JsValue>;
    }
}
