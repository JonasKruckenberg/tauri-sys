use anyhow::ensure;
use tauri_sys::dialog::{FileDialogBuilder, MessageDialogBuilder, MessageDialogKind};

pub async fn ask() -> anyhow::Result<()> {
    let works = MessageDialogBuilder::new()
        .set_title("Tauri")
        .set_kind(MessageDialogKind::Warning)
        .ask("Does this work? \n Click Yes to mark this test as passing")
        .await?;

    ensure!(works);

    Ok(())
}

pub async fn confirm() -> anyhow::Result<()> {
    let works = MessageDialogBuilder::new()
        .set_title("Tauri")
        .set_kind(MessageDialogKind::Warning)
        .confirm("Does this work? \n Click Ok to mark this test as passing")
        .await?;

    ensure!(works);

    Ok(())
}

pub async fn message() -> anyhow::Result<()> {
    MessageDialogBuilder::new()
        .set_title("Tauri")
        .set_kind(MessageDialogKind::Warning)
        .message("This is a message just for you!")
        .await?;

    Ok(())
}

pub async fn pick_file() -> anyhow::Result<()> {
    let file = FileDialogBuilder::new()
        .set_title("Select a file to mark this test as passing")
        .pick_file()
        .await?;

    ensure!(file.is_some());

    Ok(())
}

pub async fn pick_files() -> anyhow::Result<()> {
    let file = FileDialogBuilder::new()
        .set_title("Select a multiple files to mark this test as passing")
        .pick_files()
        .await?;

    ensure!(file.is_some());
    ensure!(file.unwrap().count() > 1);

    Ok(())
}

pub async fn pick_folder() -> anyhow::Result<()> {
    let file = FileDialogBuilder::new()
        .set_title("Select a folder to mark this test as passing")
        .pick_folder()
        .await?;

    ensure!(file.is_some());

    Ok(())
}

pub async fn pick_folders() -> anyhow::Result<()> {
    let file = FileDialogBuilder::new()
        .set_title("Select a multiple folders to mark this test as passing")
        .pick_folders()
        .await?;

    ensure!(file.is_some());
    ensure!(file.unwrap().count() > 1);

    Ok(())
}

pub async fn save() -> anyhow::Result<()> {
    let file = FileDialogBuilder::new()
        .set_title("Select a file to mark this test as passing")
        .save()
        .await?;

    ensure!(file.is_some());

    Ok(())
}
