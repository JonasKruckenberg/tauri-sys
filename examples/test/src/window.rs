use anyhow::ensure;
use tauri_sys::window;

pub async fn create_window() -> anyhow::Result<()> {
    let win = window::WebviewWindowBuilder::new("foo").build()?;

    ensure!(win.is_visible().await?);
    // ensure!(win.label() == "foo".to_string());

    win.close().await?;

    Ok(())
}
