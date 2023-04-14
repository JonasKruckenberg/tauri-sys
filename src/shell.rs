//! Access the system shell. Allows you to spawn child processes and manage files and URLs using their default application.

/// Opens a path or URL with the system's default app.
#[inline(always)]
pub async fn open(path: impl AsRef<str>) -> crate::Result<()> {
    inner::open(path.as_ref(), None).await?;
    Ok(())
}

/// Opens a path or URL with the system's default app, or the one specified with `openWith`.
#[inline(always)]
pub async fn open_with(path: impl AsRef<str>, with: impl Into<Option<&str>>) -> crate::Result<()> {
    inner::open(path.as_ref(), with.into()).await?;
    Ok(())
}

mod inner {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/src/shell.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub async fn open(path: &str, with: Option<&str>) -> Result<(), JsValue>;
    }
}
