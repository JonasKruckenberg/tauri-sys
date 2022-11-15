use anyhow::ensure;
use tauri_sys::dialog::{FileDialogBuilder, MessageDialogBuilder, MessageDialogType};

pub async fn ask() -> anyhow::Result<()> {
    let mut builder = MessageDialogBuilder::new();
    builder.set_title("Tauri");
    builder.set_type(MessageDialogType::Warning);

    let works = builder
        .ask("Does this work? \n Click Yes to mark this test as passing")
        .await?;

    ensure!(works);

    Ok(())
}

pub async fn confirm() -> anyhow::Result<()> {
    let mut builder = MessageDialogBuilder::new();
    builder.set_title("Tauri");
    builder.set_type(MessageDialogType::Warning);

    let works = builder
        .confirm("Does this work? \n Click Ok to mark this test as passing")
        .await?;

    ensure!(works);

    Ok(())
}

pub async fn message() -> anyhow::Result<()> {
    let mut builder = MessageDialogBuilder::new();
    builder.set_title("Tauri");
    builder.set_type(MessageDialogType::Warning);

    builder.message("This is a message just for you!").await?;

    Ok(())
}

pub async fn pick_file() -> anyhow::Result<()> {
    let mut builder = FileDialogBuilder::new();
    builder.set_title("Select a file to mark this test as passing");

    let file = builder.pick_file().await?;

    ensure!(file.is_some());

    Ok(())
}

pub async fn pick_files() -> anyhow::Result<()> {
    let mut builder = FileDialogBuilder::new();
    builder.set_title("Select a multiple files to mark this test as passing");

    let file = builder.pick_files().await?;

    ensure!(file.is_some());
    ensure!(file.unwrap().len() > 1);

    Ok(())
}

pub async fn pick_folder() -> anyhow::Result<()> {
    let mut builder = FileDialogBuilder::new();
    builder.set_title("Select a folder to mark this test as passing");

    let file = builder.pick_folder().await?;

    ensure!(file.is_some());

    Ok(())
}

pub async fn pick_folders() -> anyhow::Result<()> {
    let mut builder = FileDialogBuilder::new();
    builder.set_title("Select a multiple folders to mark this test as passing");

    let file = builder.pick_folders().await?;

    ensure!(file.is_some());
    ensure!(file.unwrap().len() > 1);

    Ok(())
}

pub async fn save() -> anyhow::Result<()> {
    let mut builder = FileDialogBuilder::new();
    builder.set_title("Select a file to mark this test as passing");

    let file = builder.save().await?;

    ensure!(file.is_some());

    Ok(())
}