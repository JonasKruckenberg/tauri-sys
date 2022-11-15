use anyhow::ensure;
use tauri_sys::notification::{self, Permission};

pub async fn is_permission_granted() -> anyhow::Result<()> {
    let granted = notification::is_permission_granted().await?;

    ensure!(granted);

    Ok(())
}

pub async fn request_permission() -> anyhow::Result<()> {
    let permission = notification::request_permission().await?;

    ensure!(permission == Permission::Granted);

    Ok(())
}

pub async fn show_notification() -> anyhow::Result<()> {
    let mut n = notification::Notification::default();
        n.set_title("TAURI");
        n.set_body("Tauri is awesome!");
    
    n.show()?;

    Ok(())
}