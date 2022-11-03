use anyhow::ensure;
use tauri_sys::clipboard;

pub async fn test() -> anyhow::Result<()> {
    clipboard::write_text("foobar").await;

    let text = clipboard::read_text().await;

    ensure!(text == Some("foobar".to_string()));

    Ok(())
}