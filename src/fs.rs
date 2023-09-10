//! Access the file system.
//!
//! # Security
//!
//! This module prevents path traversal, not allowing absolute paths or parent dir components
//! (i.e. "/usr/path/to/file" or "../path/to/file" paths are not allowed).
//! Paths accessed with this API must be relative to one of the {@link BaseDirectory | base directories}
//! so if you need access to arbitrary filesystem paths, you must write such logic on the core layer instead.
//!
//! The API has a scope configuration that forces you to restrict the paths that can be accessed using glob patterns.
//!
//! The scope configuration is an array of glob patterns describing folder paths that are allowed.
//! For instance, this scope configuration only allows accessing files on the
//! *databases* folder of the {@link path.appDataDir | $APPDATA directory}:
//! ```json
//! {
//!   "plugins": {
//!     "fs": {
//!       "scope": ["$APPDATA/databases/*"]
//!     }
//!   }
//! }
//! ```
//!
//! Notice the use of the `$APPDATA` variable. The value is injected at runtime, resolving to the {@link path.appDataDir | app data directory}.
//! The available variables are:
//! {@link path.appConfigDir | `$APPCONFIG`}, {@link path.appDataDir | `$APPDATA`}, {@link path.appLocalDataDir | `$APPLOCALDATA`},
//! {@link path.appCacheDir | `$APPCACHE`}, {@link path.appLogDir | `$APPLOG`},
//! {@link path.audioDir | `$AUDIO`}, {@link path.cacheDir | `$CACHE`}, {@link path.configDir | `$CONFIG`}, {@link path.dataDir | `$DATA`},
//! {@link path.localDataDir | `$LOCALDATA`}, {@link path.desktopDir | `$DESKTOP`}, {@link path.documentDir | `$DOCUMENT`},
//! {@link path.downloadDir | `$DOWNLOAD`}, {@link path.executableDir | `$EXE`}, {@link path.fontDir | `$FONT`}, {@link path.homeDir | `$HOME`},
//! {@link path.pictureDir | `$PICTURE`}, {@link path.publicDir | `$PUBLIC`}, {@link path.runtimeDir | `$RUNTIME`},
//! {@link path.templateDir | `$TEMPLATE`}, {@link path.videoDir | `$VIDEO`}, {@link path.resourceDir | `$RESOURCE`},
//! {@link os.tempdir | `$TEMP`}.
//!
//! Trying to execute any API with a URL not configured on the scope results in a promise rejection due to denied access.
//!
//! Note that this scope applies to **all** APIs on this module.
use crate::Error;
use js_sys::{ArrayBuffer, Uint8Array};
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
pub struct FileEntry {
    pub path: PathBuf,
    pub name: Option<String>,
    pub children: Option<Vec<FileEntry>>,
}

#[derive(Serialize, Clone, PartialEq, Debug)]
struct FsDirOptions {
    pub dir: Option<BaseDirectory>,
    pub recursive: Option<bool>,
}

#[derive(Serialize, Clone, PartialEq, Debug)]
struct FsOptions {
    pub dir: Option<BaseDirectory>,
}

#[derive(Serialize, Clone, PartialEq, Debug)]
struct FsTextFileOption {
    pub contents: String,
    path: PathBuf,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Permissions {
    /// Indicates if these permissions describe a readonly (unwritable) file
    readonly: bool,
    /// The underlying raw `st_mode` bits that contain the standard Unix permissions for this file.
    mode: Option<usize>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    #[serde(rename = "accessedAtMs")]
    /// The last access time of this metadata.
    pub accessed_at: i64,
    #[serde(rename = "createdAtMs")]
    /// The creation time listed in this metadata.    
    pub created_at: i64,
    #[serde(rename = "modifiedAtMs")]
    /// The last modification time listed in this metadata.
    pub modified_at: i64,
    /// Indicates if this metadata is for a directory.
    pub is_dir: bool,
    /// Indicates if this metadata is for a regular file.
    pub is_file: bool,
    /// Indicates if this metadata is for a symbolic link.
    pub is_symlink: bool,
    /// The size of the file, in bytes, this metadata is for.
    pub size: u64,
    /// The permissions of the file this metadata is for.
    pub permissions: Permissions,
    /// The ID of the device containing the file. Only available on Unix.
    pub dev: Option<u64>,
    /// The inode number. Only available on Unix.
    pub ino: Option<u64>,
    /// The rights applied to this file. Only available on Unix.
    pub mode: Option<u32>,
    /// The number of hard links pointing to this file. Only available on Unix.
    pub nlink: Option<u64>,
    /// The user ID of the owner of this file. Only available on Unix.
    pub uid: Option<u32>,
    /// The group ID of the owner of this file. Only available on Unix.
    pub gid: Option<u32>,
    /// The device ID of this file (if it is a special one). Only available on Unix.
    pub rdev: Option<u64>,
    /// The block size for filesystem I/O. Only available on Unix.
    pub blksize: Option<u64>,
    /// The number of blocks allocated to the file, in 512-byte units. Only available on Unix.
    pub blocks: Option<u64>,
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
        serde_wasm_bindgen::to_value(&FsOptions { dir: Some(dir) })?,
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
pub async fn exists(path: &Path, dir: BaseDirectory) -> crate::Result<bool> {
    let Some(path) = path.to_str() else {
        return Err(Error::Utf8(path.to_path_buf()));
    };

    let raw = inner::exists(
        path,
        serde_wasm_bindgen::to_value(&FsOptions { dir: Some(dir) })?,
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
pub async fn read_binary_file(path: &Path, dir: BaseDirectory) -> crate::Result<Vec<u8>> {
    let Some(path) = path.to_str() else {
        return Err(Error::Utf8(path.to_path_buf()));
    };

    let raw = inner::readBinaryFile(
        path,
        serde_wasm_bindgen::to_value(&FsOptions { dir: Some(dir) })?,
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
pub async fn read_dir(path: &Path, dir: BaseDirectory) -> crate::Result<Vec<FileEntry>> {
    let recursive = Some(false);
    let Some(path) = path.to_str() else {
        return Err(Error::Utf8(path.to_path_buf()));
    };

    let raw = inner::readDir(
        path,
        serde_wasm_bindgen::to_value(&FsDirOptions {
            dir: Some(dir),
            recursive,
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
pub async fn read_dir_all(path: &Path, dir: BaseDirectory) -> crate::Result<Vec<FileEntry>> {
    let recursive = Some(true);
    let Some(path) = path.to_str() else {
        return Err(Error::Utf8(path.to_path_buf()));
    };

    let raw = inner::readDir(
        path,
        serde_wasm_bindgen::to_value(&FsDirOptions {
            dir: Some(dir),
            recursive,
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
pub async fn read_text_file(path: &Path, dir: BaseDirectory) -> crate::Result<String> {
    let Some(path) = path.to_str() else {
        return Err(Error::Utf8(path.to_path_buf()));
    };

    let raw = inner::readTextFile(
        path,
        serde_wasm_bindgen::to_value(&FsOptions { dir: Some(dir) })?,
    )
    .await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Removes a directory.
/// If the directory is not empty the promise will be rejected.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::fs;
///
/// fs::remove_dir(path, BaseDirectory::Download).expect("could not remove directory");
/// ```
pub async fn remove_dir(dir: &Path, base_dir: BaseDirectory) -> crate::Result<()> {
    let recursive = Some(false);
    let Some(dir) = dir.to_str() else {
        return Err(Error::Utf8(dir.to_path_buf()));
    };

    Ok(inner::removeDir(
        dir,
        serde_wasm_bindgen::to_value(&FsDirOptions {
            dir: Some(base_dir),
            recursive,
        })?,
    )
    .await?)
}

/// Removes a directory and its contents.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::fs;
///
/// fs::remove_dir_all(path, BaseDirectory::Download).expect("could not remove directory");
/// ```
pub async fn remove_dir_all(dir: &Path, base_dir: BaseDirectory) -> crate::Result<()> {
    let recursive = Some(true);
    let Some(dir) = dir.to_str() else {
        return Err(Error::Utf8(dir.to_path_buf()));
    };

    Ok(inner::removeDir(
        dir,
        serde_wasm_bindgen::to_value(&FsDirOptions {
            dir: Some(base_dir),
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
/// fs::remove_file(path, BaseDirectory::Download).expect("could not remove file");
/// ```
pub async fn remove_file(file: &Path, dir: BaseDirectory) -> crate::Result<()> {
    let Some(file) = file.to_str() else {
        return Err(Error::Utf8(file.to_path_buf()));
    };

    Ok(inner::removeFile(
        file,
        serde_wasm_bindgen::to_value(&FsOptions { dir: Some(dir) })?,
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
/// fs::rename_file(old_path, new_path, BaseDirectory::Download).expect("could not rename file");
/// ```
pub async fn rename_file(
    old_path: &Path,
    new_path: &Path,
    dir: BaseDirectory,
) -> crate::Result<()> {
    let Some(old_path) = old_path.to_str() else {
        return Err(Error::Utf8(old_path.to_path_buf()));
    };

    let Some(new_path) = new_path.to_str() else {
        return Err(Error::Utf8(new_path.to_path_buf()));
    };

    Ok(inner::renameFile(
        old_path,
        new_path,
        serde_wasm_bindgen::to_value(&FsOptions { dir: Some(dir) })?,
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
/// fs::write_binary_file(path, contents, BaseDirectory::Download).expect("could not writet binary file");
/// ```
pub async fn write_binary_file(
    path: &Path,
    contents: &Vec<u8>,
    dir: BaseDirectory,
) -> crate::Result<()> {
    let Some(path) = path.to_str() else {
        return Err(Error::Utf8(path.to_path_buf()));
    };

    let array = Uint8Array::from(contents.as_slice());

    Ok(inner::writeBinaryFile(
        path,
        array.buffer(),
        serde_wasm_bindgen::to_value(&FsOptions { dir: Some(dir) })?,
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
/// fs::write_text_file(path, contents, BaseDirectory::Download).expect("could not writet binary file");
/// ```
pub async fn write_text_file(path: &Path, contents: &str, dir: BaseDirectory) -> crate::Result<()> {
    let Some(path) = path.to_str() else {
        return Err(Error::Utf8(path.to_path_buf()));
    };

    Ok(inner::writeTextFile(
        path,
        &contents,
        serde_wasm_bindgen::to_value(&FsOptions { dir: Some(dir) })?,
    )
    .await?)
}

/// Returns the metadata for the given path.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::fs;
///
/// fs::metadata(path).expect("failed to get metadata");
/// ```
pub async fn metadata(path: &Path) -> crate::Result<Metadata> {
    let Some(path) = path.to_str() else {
        return Err(Error::Utf8(path.to_path_buf()));
    };

    let raw = inner::metadata(path).await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
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
            contents: ArrayBuffer,
            options: JsValue,
        ) -> Result<(), JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn writeTextFile(
            filePath: &str,
            contents: &str,
            options: JsValue,
        ) -> Result<(), JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn metadata(path: &str) -> Result<JsValue, JsValue>;
    }
}
