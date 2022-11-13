use wasm_bindgen_futures::JsFuture;

use crate::Error;

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
pub async fn read_text() -> crate::Result<Option<String>> {
    JsFuture::from(inner::readText())
        .await
        .map(|v| v.as_string())
        .map_err(Error::Other)
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
pub async fn write_text(text: &str) -> crate::Result<()> {
    JsFuture::from(inner::writeText(text))
        .await
        .map_err(Error::Other)?;

    Ok(())
}

mod inner {
    use wasm_bindgen::{prelude::wasm_bindgen};

    #[wasm_bindgen(module = "/dist/clipboard.js")]
    extern "C" {
        pub fn readText() -> js_sys::Promise;
        pub fn writeText(text: &str) -> js_sys::Promise;
    }
}
