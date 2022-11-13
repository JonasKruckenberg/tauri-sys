use anyhow::ensure;
use tauri_sys::app;

pub async fn get_name() -> anyhow::Result<()> {
    let name = app::get_name().await;

    ensure!(name == "tauri-sys-test");

    Ok(())
}

pub async fn get_version() -> anyhow::Result<()> {
    let version = app::get_version().await?;

    ensure!(version.major == 0);
    ensure!(version.minor == 0);
    ensure!(version.patch == 0);
    ensure!(version.build.is_empty());
    ensure!(version.pre.is_empty());

    Ok(())
}

pub async fn get_tauri_version() -> anyhow::Result<()> {
    let version = app::get_tauri_version().await;

    ensure!(version.major == 1);
    ensure!(version.minor == 1);
    ensure!(version.patch == 1);
    ensure!(version.build.is_empty());
    ensure!(version.pre.is_empty());

    Ok(())
}
