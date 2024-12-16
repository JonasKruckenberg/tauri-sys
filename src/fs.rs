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
use crate::Error;
use js_sys::ArrayBuffer;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::path::{Path, PathBuf};
use std::str;

#[derive(Serialize_repr, Clone, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum BaseDirectory {
    Audio = 1,
    Cache = 2,
    Config = 3,
    Data = 4,
    LocalData = 5,
    Document = 6,
    Download = 7,
    Picture = 8,
    Public = 9,
    Video = 10,
    Resource = 11,
    Temp = 12,
    AppConfig = 13,
    AppData = 14,
    AppLocalData = 15,
    AppCache = 16,
    AppLog = 17,
    Desktop = 18,
    Executable = 19,
    Font = 20,
    Home = 21,
    Runtime = 22,
    Template = 23,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct DirEntry {
    pub name: PathBuf,
    pub is_directory: bool,
    pub is_file: bool,
    pub is_symlink: bool,
}

#[derive(Serialize, Clone, PartialEq, Debug)]
struct FsDirOptions {
    pub dir: Option<BaseDirectory>,
    pub recursive: Option<bool>,
}

#[derive(Serialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct ReadDirOptions {
    pub base_dir: Option<BaseDirectory>,
}

#[derive(Serialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct FsOptions {
    pub base_dir: Option<BaseDirectory>,
}

#[derive(Serialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct RemoveOptions {
    pub base_dir: Option<BaseDirectory>,
    pub recursive: Option<bool>,
}

#[derive(Serialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct RenameOptions {
    pub old_path_base_dir: Option<BaseDirectory>,
    pub new_path_base_dir: Option<BaseDirectory>,
}

#[derive(Serialize, Clone, PartialEq, Debug)]
struct FsTextFileOption {
    pub contents: String,
    path: PathBuf,
}

/// Copies a file to a destination.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::fs;
///
/// fs::copy_file(source, destination, BaseDirectory::Download).expect("could not copy file");
/// ```
///
/// Requires [`allowlist > fs > copyFile`](https://tauri.app/v1/api/js/fs) to be enabled.
pub async fn copy_file(source: &Path, destination: &Path, dir: BaseDirectory) -> crate::Result<()> {
    let Some(source) = source.to_str() else {
        return Err(Error::Utf8(source.to_path_buf()));
    };

    let Some(destination) = destination.to_str() else {
        return Err(Error::Utf8(destination.to_path_buf()));
    };

    let raw = inner::copyFile(
        source,
        destination,
        serde_wasm_bindgen::to_value(&FsOptions { base_dir: Some(dir) })?,
    )
    .await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Creates a directory.
/// If one of the path's parent components doesn't exist the promise will be rejected.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::fs;
///
/// fs::create_dir(dir, BaseDirectory::Download).expect("could not create directory");
/// ```
///
/// Requires [`allowlist > fs > createDir`](https://tauri.app/v1/api/js/fs) to be enabled.
pub async fn create_dir(dir: &Path, base_dir: BaseDirectory) -> crate::Result<()> {
    let recursive = Some(false);

    let Some(dir) = dir.to_str() else {
        return Err(Error::Utf8(dir.to_path_buf()));
    };

    Ok(inner::createDir(
        dir,
        serde_wasm_bindgen::to_value(&FsDirOptions {
            dir: Some(base_dir),
            recursive,
        })?,
    )
    .await?)
}

/// Creates a directory recursively.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::fs;
///
/// fs::create_dir_all(dir, BaseDirectory::Download).expect("could not create directory");
/// ```
///
/// Requires [`allowlist > fs > createDir`](https://tauri.app/v1/api/js/fs) to be enabled.
pub async fn create_dir_all(dir: &Path, base_dir: BaseDirectory) -> crate::Result<()> {
    let recursive = Some(true);

    let Some(dir) = dir.to_str() else {
        return Err(Error::Utf8(dir.to_path_buf()));
    };

    Ok(inner::createDir(
        dir,
        serde_wasm_bindgen::to_value(&FsDirOptions {
            dir: Some(base_dir),
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
/// let file_exists = fs::exists(path, BaseDirectory::Download).expect("could not check if path exists");
/// ```
///
/// Requires [`allowlist > fs > exists`](https://tauri.app/v1/api/js/fs) to be enabled.
pub async fn exists(path: &Path, dir: BaseDirectory) -> crate::Result<bool> {
    let Some(path) = path.to_str() else {
        return Err(Error::Utf8(path.to_path_buf()));
    };

    let raw = inner::exists(
        path,
        serde_wasm_bindgen::to_value(&FsOptions { base_dir: Some(dir) })?,
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
/// let contents = fs::read_binary_file(filePath, BaseDirectory::Download).expect("could not read file contents");
/// ```
///
/// Requires [`allowlist > fs > readBinaryFile`](https://tauri.app/v1/api/js/fs) to be enabled.
pub async fn read_binary_file(path: &Path, dir: BaseDirectory) -> crate::Result<Vec<u8>> {
    let Some(path) = path.to_str() else {
        return Err(Error::Utf8(path.to_path_buf()));
    };

    let raw = inner::readBinaryFile(
        path,
        serde_wasm_bindgen::to_value(&FsOptions { base_dir: Some(dir) })?,
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
/// let files = fs::read_dir(path, BaseDirectory::Download).expect("could not read directory");
/// ```
///
/// Requires [`allowlist > fs > readDir`](https://tauri.app/v1/api/js/fs) to be enabled.
pub async fn read_dir(path: &Path, dir: BaseDirectory) -> crate::Result<Vec<DirEntry>> {
    let Some(path) = path.to_str() else {
        return Err(Error::Utf8(path.to_path_buf()));
    };

    let raw = inner::readDir(
        path,
        serde_wasm_bindgen::to_value(&ReadDirOptions {
            base_dir: Some(dir),
        })?,
    )
    .await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// List directory files recursively.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::fs;
///
/// let files = fs::read_dir_all(path, BaseDirectory::Download).expect("could not read directory");
/// ```
///
/// Requires [`allowlist > fs > readDir`](https://tauri.app/v1/api/js/fs) to be enabled.
pub async fn read_dir_all(path: &Path, dir: BaseDirectory) -> crate::Result<Vec<DirEntry>> {
    let Some(path) = path.to_str() else {
        return Err(Error::Utf8(path.to_path_buf()));
    };

    let raw = inner::readDir(
        path,
        serde_wasm_bindgen::to_value(&ReadDirOptions {
            base_dir: Some(dir),
        })?,
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
/// let contents = fs::readTextFile(path, BaseDirectory::Download).expect("could not read file as text");
/// ```
///
/// Requires [`allowlist > fs > readTextFile`](https://tauri.app/v1/api/js/fs) to be enabled.
pub async fn read_text_file(path: &Path, dir: BaseDirectory) -> crate::Result<String> {
    let Some(path) = path.to_str() else {
        return Err(Error::Utf8(path.to_path_buf()));
    };

    let raw = inner::readTextFile(
        path,
        serde_wasm_bindgen::to_value(&FsOptions { base_dir: Some(dir) })?,
    )
    .await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Removes the named file or directory. If the directory is not empty and
/// the recursive option isn’t set to true, the promise will be rejected.
pub async fn remove(path: &Path, options: RemoveOptions) -> crate::Result<()> {
    let Some(path) = path.to_str() else {
        return Err(Error::Utf8(path.to_path_buf()));
    };

    Ok(inner::remove(path, serde_wasm_bindgen::to_value(&options)?).await?)
}

/// Renames (moves) oldpath to newpath. Paths may be files or directories.
/// If newpath already exists and is not a directory, rename() replaces it.
/// OS-specific restrictions may apply when oldpath and newpath are in
/// different directories.
///
/// On Unix, this operation does not follow symlinks at either path.
pub async fn rename(
    old_path: &Path,
    new_path: &Path,
    options: RenameOptions,
) -> crate::Result<()> {
    let Some(old_path) = old_path.to_str() else {
        return Err(Error::Utf8(old_path.to_path_buf()));
    };

    let Some(new_path) = new_path.to_str() else {
        return Err(Error::Utf8(new_path.to_path_buf()));
    };

    Ok(inner::rename(
        old_path,
        new_path,
        serde_wasm_bindgen::to_value(&options)?,
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
/// fs::write_binary_file(path, contents, BaseDirectory::Download).expect("could not writes binary file");
/// ```
///
/// Requires [`allowlist > fs > writeBinaryFile`](https://tauri.app/v1/api/js/fs) to be enabled.
pub async fn write_binary_file(
    path: &Path,
    contents: ArrayBuffer,
    dir: BaseDirectory,
) -> crate::Result<()> {
    let Some(path) = path.to_str() else {
        return Err(Error::Utf8(path.to_path_buf()));
    };

    Ok(inner::writeFile(
        path,
        contents,
        serde_wasm_bindgen::to_value(&FsOptions { base_dir: Some(dir) })?,
    )
    .await?)
}

/// Writes a UTF-8 text file.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::fs;
///
/// fs::write_text_file(path, contents, BaseDirectory::Download).expect("could not writes binary file");
/// ```
///
/// Requires [`allowlist > fs > writeTextFile`](https://tauri.app/v1/api/js/fs) to be enabled.
pub async fn write_text_file(path: &Path, contents: &str, dir: BaseDirectory) -> crate::Result<()> {
    let Some(path) = path.to_str() else {
        return Err(Error::Utf8(path.to_path_buf()));
    };

    Ok(inner::writeTextFile(
        path,
        &contents,
        serde_wasm_bindgen::to_value(&FsOptions { base_dir: Some(dir) })?,
    )
    .await?)
}

mod inner {
    use super::ArrayBuffer;
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
        pub async fn remove(dir: &str, options: JsValue) -> Result<(), JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn rename(
            oldPath: &str,
            newPath: &str,
            options: JsValue,
        ) -> Result<(), JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn writeFile(
            filePath: &str,
            contents: ArrayBuffer,
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
