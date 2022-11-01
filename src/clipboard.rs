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
pub async fn read_text() -> Option<String> {
    inner::readText().await.as_string()
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
///
/// @returns A promise indicating the success or failure of the operation.
#[inline(always)]
pub async fn write_text(text: &str) {
    inner::writeText(text).await
}

mod inner {
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

    #[wasm_bindgen(module = "/dist/clipboard.js")]
    extern "C" {
        pub async fn readText() -> JsValue;
        pub async fn writeText(text: &str);
    }
}