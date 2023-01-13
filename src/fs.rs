//! Access the file system.
//!
//! The APIs must be added to `tauri.allowlist.fs` in `tauri.conf.json`:
//! ```json
//! {
//!   "tauri": {
//!     "allowlist": {
//!       "fs": {
//!         "all": true, // enable all FS APIs
//!         "readFile": true,
//!         "writeFile": true,
//!         "readDir": true,
//!         "copyFile": true,
//!         "createDir": true,
//!         "removeDir": true,
//!         "removeFile": true,
//!         "renameFile": true,
//!         "exists": true
//!       }
//!     }
//!   }
//! }
//! ```
//! It is recommended to allowlist only the APIs you use for optimal bundle size and security.
use js_sys::ArrayBuffer;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::path::{Path, PathBuf};

#[derive(Serialize_repr)]
#[repr(u16)]
pub enum BaseDirectory {
    Audio = 1,
    Cache = 2,
    Config = 3,
    Data = 4,
    LocalData = 5,
    Desktop = 6,
    Document = 7,
    Download = 8,
    Executable = 9,
    Font = 10,
    Home = 11,
    Picture = 12,
    Public = 13,
    Runtime = 14,
    Template = 15,
    Video = 16,
    Resource = 17,
    App = 18,
    Log = 19,
    Temp = 20,
    AppConfig = 21,
    AppData = 22,
    AppLocalData = 23,
    AppCache = 24,
    AppLog = 25,
}

#[derive(Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
    pub name: Option<String>,
    pub children: Option<Vec<FileEntry>>,
}

// #[derive(Serialize)]
// pub struct FsBinaryFileOption {
//     pub path: PathBuf,
//     pub contents: BinaryFileContents,
// }

#[derive(Serialize)]
pub struct FsDirOptions {
    pub dir: Option<BaseDirectory>,
    pub recursive: Option<bool>,
}

#[derive(Serialize)]
pub struct FsOptions {
    pub dir: Option<BaseDirectory>,
}

#[derive(Serialize)]
pub struct FsTextFileOption {
    pub contents: String,
    path: PathBuf,
}

pub type BinaryFileContents = ArrayBuffer;

/// Copies a file to a destination.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::fs;
///
/// fs::copy_file(source, destination, Some(BaseDirectory::Download)).expect("could not copy file");
/// ```
///
/// Requires [`allowlist > fs > copyFile`](https://tauri.app/v1/api/js/fs) to be enabled.
pub async fn copy_file(
    source: &Path,
    destination: &Path,
    dir: Option<BaseDirectory>,
) -> crate::Result<()> {
    let raw = inner::copyFile(
        source.to_str().expect("could not convert path to str"),
        destination.to_str().expect("could not convert path to str"),
        serde_wasm_bindgen::to_value(&FsOptions { dir })?,
    )
    .await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Creates a directory.
/// If one of the path's parent components doesn't exist and the `recursive` option isn't set to true, the promise will be rejected.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::fs;
///
/// fs::create_dir(dir, Some(BaseDirectory::Download), Some(true)).expect("could not create directory");
/// ```
///
/// Requires [`allowlist > fs > createDir`](https://tauri.app/v1/api/js/fs) to be enabled.
pub async fn create_dir(
    dir: &Path,
    base_dir: Option<BaseDirectory>,
    recursive: Option<bool>,
) -> crate::Result<()> {
    Ok(inner::createDir(
        dir.to_str().expect("could not convert path to str"),
        serde_wasm_bindgen::to_value(&FsDirOptions {
            dir: base_dir,
            recursive,
        })?,
    )
    .await?)
}

/// Checks if a path exists.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::fs;
///
/// let file_exists = fs::exists(path, Some(BaseDirectory::Download)).expect("could not check if path exists");
/// ```
///
/// Requires [`allowlist > fs > exists`](https://tauri.app/v1/api/js/fs) to be enabled.
pub async fn exists(path: &Path, dir: Option<BaseDirectory>) -> crate::Result<bool> {
    let raw = inner::exists(
        path.to_str().expect("could not convert path to str"),
        serde_wasm_bindgen::to_value(&FsOptions { dir })?,
    )
    .await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Reads a file as a byte array.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::fs;
///
/// let contents = fs::read_binary_file(filePath, Some(BaseDirectory::Download)).expect("could not read file contents");
/// ```
///
/// Requires [`allowlist > fs > readBinaryFile`](https://tauri.app/v1/api/js/fs) to be enabled.
pub async fn read_binary_file(path: &Path, dir: Option<BaseDirectory>) -> crate::Result<Vec<u8>> {
    let raw = inner::readBinaryFile(
        path.to_str().expect("could not convert path to str"),
        serde_wasm_bindgen::to_value(&FsOptions { dir })?,
    )
    .await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// List directory files.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::fs;
///
/// let files = fs::read_dir(path, Some(BaseDirectory::Download), Some(true)).expect("could not read directory");
/// ```
///
/// Requires [`allowlist > fs > readDir`](https://tauri.app/v1/api/js/fs) to be enabled.
pub async fn read_dir(
    path: &Path,
    dir: Option<BaseDirectory>,
    recursive: Option<bool>,
) -> crate::Result<Vec<FileEntry>> {
    let raw = inner::readDir(
        path.to_str().expect("could not convert path to str"),
        serde_wasm_bindgen::to_value(&FsDirOptions { dir, recursive })?,
    )
    .await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Read a file as an UTF-8 encoded string.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::fs;
///
/// let contents = fs::readTextFile(path, Some(BaseDirectory::Download)).expect("could not read file as text");
/// ```
///
/// Requires [`allowlist > fs > readTextFile`](https://tauri.app/v1/api/js/fs) to be enabled.
pub async fn read_text_file(path: &Path, dir: Option<BaseDirectory>) -> crate::Result<String> {
    let raw = inner::readTextFile(
        path.to_str().expect("could not convert path to str"),
        serde_wasm_bindgen::to_value(&FsOptions { dir })?,
    )
    .await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Removes a directory.
/// If the directory is not empty and the `recursive` option isn't set to true, the promise will be rejected.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::fs;
///
/// fs::remove_dir(path, Some(BaseDirectory::Download), Some(true)).expect("could not remove directory");
/// ```
///
/// Requires [`allowlist > fs > removeDir`](https://tauri.app/v1/api/js/fs) to be enabled.
pub async fn remove_dir(
    dir: &Path,
    base_dir: Option<BaseDirectory>,
    recursive: Option<bool>,
) -> crate::Result<()> {
    Ok(inner::removeDir(
        dir.to_str().expect("could not convert path to str"),
        serde_wasm_bindgen::to_value(&FsDirOptions {
            dir: base_dir,
            recursive,
        })?,
    )
    .await?)
}

/// Removes a file.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::fs;
///
/// fs::remove_file(path, Some(BaseDirectory::Download)).expect("could not remove file");
/// ```
///
/// Requires [`allowlist > fs > removeFile`](https://tauri.app/v1/api/js/fs) to be enabled.
pub async fn remove_file(file: &Path, dir: Option<BaseDirectory>) -> crate::Result<()> {
    Ok(inner::removeFile(
        file.to_str().expect("could not convert path to str"),
        serde_wasm_bindgen::to_value(&FsOptions { dir })?,
    )
    .await?)
}

/// Renames a file.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::fs;
///
/// fs::rename_file(old_path, new_path, Some(BaseDirectory::Download)).expect("could not rename file");
/// ```
///
/// Requires [`allowlist > fs > renameFile`](https://tauri.app/v1/api/js/fs) to be enabled.
pub async fn rename_file(
    old_path: &Path,
    new_path: &Path,
    dir: Option<BaseDirectory>,
) -> crate::Result<()> {
    Ok(inner::renameFile(
        old_path.to_str().expect("could not convert path to str"),
        new_path.to_str().expect("could not convert path to str"),
        serde_wasm_bindgen::to_value(&FsOptions { dir })?,
    )
    .await?)
}

/// Writes a byte array content to a file.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::fs;
///
/// fs::write_binary_file(path, contents, Some(BaseDirectory::Download)).expect("could not writet binary file");
/// ```
///
/// Requires [`allowlist > fs > writeBinaryFile`](https://tauri.app/v1/api/js/fs) to be enabled.
// pub async fn write_binary_file(
//     path: &Path,
//     contents: BinaryFileContents,
//     dir: Option<BaseDirectory>,
// ) -> crate::Result<()> {
//     Ok(inner::writeBinaryFile(
//         path.to_str().expect("could not convert path to str"),
//         contents,
//         serde_wasm_bindgen::to_value(&FsOptions { dir })?,
//     )
//     .await?)
// }

/// Writes a UTF-8 text file.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::fs;
///
/// fs::write_text_file(path, contents, Some(BaseDirectory::Download)).expect("could not writet binary file");
/// ```
///
/// Requires [`allowlist > fs > writeTextFile`](https://tauri.app/v1/api/js/fs) to be enabled.
pub async fn write_text_file(
    path: &Path,
    contents: &str,
    dir: Option<BaseDirectory>,
) -> crate::Result<()> {
    Ok(inner::writeTextFile(
        path.to_str().expect("could not convert path to str"),
        &contents,
        serde_wasm_bindgen::to_value(&FsOptions { dir })?,
    )
    .await?)
}

mod inner {
    use super::BinaryFileContents;
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

    #[wasm_bindgen(module = "/src/fs.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub async fn copyFile(
            source: &str,
            destination: &str,
            options: JsValue,
        ) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn createDir(dir: &str, options: JsValue) -> Result<(), JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn exists(path: &str, options: JsValue) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn readBinaryFile(filePath: &str, options: JsValue) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn readTextFile(filePath: &str, options: JsValue) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn readDir(dir: &str, options: JsValue) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn removeDir(dir: &str, options: JsValue) -> Result<(), JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn removeFile(source: &str, options: JsValue) -> Result<(), JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn renameFile(
            oldPath: &str,
            newPath: &str,
            options: JsValue,
        ) -> Result<(), JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn writeBinaryFile(
            filePath: &str,
            contents: BinaryFileContents,
            options: JsValue,
        ) -> Result<(), JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn writeTextFile(
            filePath: &str,
            contents: &str,
            options: JsValue,
        ) -> Result<(), JsValue>;
    }
}
