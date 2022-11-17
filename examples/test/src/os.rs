use tauri_sys::os;

pub async fn arch() -> anyhow::Result<()> {
    let arch = os::arch().await?;

    log::debug!("{:?}", arch);

    Ok(())
}

pub async fn platform() -> anyhow::Result<()> {
    let platform = os::platform().await?;

    log::debug!("{:?}", platform);

    Ok(())
}

pub async fn tempdir() -> anyhow::Result<()> {
    let tempdir = os::tempdir().await?;

    log::info!("{:?}", tempdir);

    Ok(())
}

pub async fn kind() -> anyhow::Result<()> {
    let kind = os::kind().await?;

    log::debug!("{:?}", kind);

    Ok(())
}

pub async fn version() -> anyhow::Result<()> {
    let version = os::version().await?;

    log::debug!("{:?}", version);

    Ok(())
}
