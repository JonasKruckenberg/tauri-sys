use anyhow::ensure;
use tauri_sys::{event, tauri};

pub async fn emit() -> anyhow::Result<()> {
    event::emit("foo", &"bar").await;

    ensure!(tauri::invoke::<_, bool>("verify_receive", &()).await.unwrap());

    Ok(())
}