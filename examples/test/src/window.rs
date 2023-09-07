use anyhow::ensure;
use tauri_sys::window;

pub async fn create_window() -> anyhow::Result<()> {
    let win = window::WindowBuilder::new("foo-win")
        .set_url("/")
        .build()
        .await?;

    ensure!(win.is_visible().await?);

    win.close().await?;

    Ok(())
}
