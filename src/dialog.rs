//! User interaction with the file system using dialog boxes.
//!
//! # Example
//!
//! ```rust,no_run
//! use tauri_api::dialog::open;
//!
//! let path = open.await;
//! ```
use serde::Serialize;
use std::path::PathBuf;

/// Extension filter for the file dialog.
///
/// # Example
///
/// ```rust,no_run
/// let filter = DialogFilter {
///   extension: vec![".jpg", ".jpeg", ".png", ".bmp"],
///   name: "images",
/// };
/// ```
#[derive(Serialize)]
pub struct DialogFilter {
    /// Extensions to filter, without a `.` prefix.
    pub extensions: Vec<String>,

    /// Filter name
    pub name: String,
}

/// Types of a [`message`] dialog.
#[derive(Serialize)]
pub enum MessageDialogType {
    Error,
    Info,
    Warning,
}

/// Options for the [`message`] dialog.
#[derive(Serialize)]
pub struct MessageDialogOptions {
    /// The title of the dialog. Defaults to the app name.
    pub title: Option<String>,

    /// The type of the dialog. Defaults to MessageDialogType::Info.
    pub kind: MessageDialogType,
}

impl MessageDialogOptions {
    /// Creates a new `MessageDialogOptions` with sensible default values.
    pub fn new() -> Self {
        Self {
            title: None,
            kind: MessageDialogType::Info,
        }
    }
}

/// Options for an [`open`] dialog.
#[derive(Serialize)]
pub struct OpenDialogOptions {
    /// Initial directory or file path.
    pub default_path: Option<PathBuf>,

    /// Whether the dialog is a directory selection or not.
    pub directory: bool,

    /// The filters of the dialog.
    pub filters: Vec<DialogFilter>,

    /// Whether the dialgo allows multiple selection or not.
    pub multiple: bool,

    /// If `directory` is `true`, indicatees that it will be read recursivley later.
    /// Defines whether subdirectories will be allowed on the scope or not.
    pub recursive: bool,

    /// The title of the dialog window.
    pub title: Option<String>,
}

impl OpenDialogOptions {
    /// Creates a new `OpenDialogOptions` with sensible default values.
    pub fn new() -> Self {
        Self {
            default_path: None,
            directory: false,
            filters: Vec::new(),
            multiple: false,
            recursive: false,
            title: None,
        }
    }
}

/// Options for the save dialog.
#[derive(Serialize)]
pub struct SaveDialogOptions {
    /// Initial directory of the file path.
    /// If it's not a directory path, the dialog interface will change to that folder.
    /// If it's not an existing directory, the file name will be set to the dialog's
    /// file name input and the dialog will be set to the parent folder.
    pub default_path: Option<PathBuf>,

    /// The filters of the dialog.
    pub filters: Vec<DialogFilter>,

    /// The title of the dialog window.
    pub title: Option<String>,
}

impl SaveDialogOptions {
    /// Creates a new `SaveDialogOptions` with sensible default values.
    pub fn new() -> Self {
        Self {
            default_path: None,
            filters: Vec::new(),
            title: None,
        }
    }
}

/// Show a question dialog with `Yes` and `No` buttons.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_api::dialog::{ask, MessageDialogOptions};
///
/// let yes = ask("Are you sure?", None).await;
/// ```
/// @param message Message to display.
/// @param options Dialog options.
/// @returns Whether the user selected `Yes` or `No`.
#[inline(always)]
pub async fn ask(message: &str, options: Option<MessageDialogOptions>) -> Option<bool> {
    inner::ask(message, serde_wasm_bindgen::to_value(&options).unwrap())
        .await
        .as_bool()
}

/// Shows a question dialog with `Ok` and `Cancel` buttons.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_api::dialog::{confirm, MessageDialogOptions};
///
/// let confirmed = confirm("Are you sure?", None).await;
/// ```
/// @returns Whether the user selelced `Ok` or `Cancel`.
pub async fn confirm(message: &str, options: Option<MessageDialogOptions>) -> Option<bool> {
    inner::confirm(message, serde_wasm_bindgen::to_value(&options).unwrap())
        .await
        .as_bool()
}

/// Shows a message dialog with an `Ok` button.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_api::dialog::{message, MessageDialogOptions};
///
/// message("Tauri is awesome", None).await;
/// ```
/// @param message Message to display.
/// @param options Dialog options.
/// @returns Promise resolved when user closes the dialog.
pub async fn message(message: &str, options: Option<MessageDialogOptions>) {
    inner::message(message, serde_wasm_bindgen::to_value(&options).unwrap()).await
}

/// Opens a file/directory selection dialog for a single file.
/// `multiple` field of [`options`](OpenDialogOptions) must be `false`, if provided.
///
/// The selected paths are added to the filesystem and asset protocol allowlist scopes.
/// When security is mroe important than the ease of use of this API,
/// prefer writing a dedicated command instead.
///
/// Note that the allowlist scope change is not persisited,
/// so the values are cleared when the applicaiton is restarted.
/// You can save it to the filessytem using the [tauri-plugin-persisted-scope](https://github.com/tauri-apps/tauri-plugin-persisted-scope).
///
/// # Example
///
/// ```rust,no_run
/// use tauri_api::dialog::{open, OpenDialogOptions};
///
/// let file = open(None).await;
/// ```
/// @param options Dialog options.
/// @returns List of file paths, or `None` if user cancelled the dialog.
pub async fn open(options: Option<OpenDialogOptions>) -> Option<PathBuf> {
    let file = inner::open(serde_wasm_bindgen::to_value(&options).unwrap()).await;
    serde_wasm_bindgen::from_value(file).unwrap()
}

/// Opens a file/directory selection dialog for multiple files.
/// `multiple` field of [`options`](OpenDialogOptions) must be `true`, if provided.
///
/// The selected paths are added to the filesystem and asset protocol allowlist scopes.
/// When security is mroe important than the ease of use of this API,
/// prefer writing a dedicated command instead.
///
/// Note that the allowlist scope change is not persisited,
/// so the values are cleared when the applicaiton is restarted.
/// You can save it to the filessytem using the [tauri-plugin-persisted-scope](https://github.com/tauri-apps/tauri-plugin-persisted-scope).
///
/// # Example
///
/// ```rust,no_run
/// use tauri_api::dialog::{open, OpenDialogOptions};
///
/// let files = open_multiple(None).await;
/// ```
/// @param options Dialog options.
/// @returns List of file paths, or `None` if user cancelled the dialog.
pub async fn open_multiple(options: Option<OpenDialogOptions>) -> Option<Vec<PathBuf>> {
    let files = inner::open_multiple(serde_wasm_bindgen::to_value(&options).unwrap()).await;
    serde_wasm_bindgen::from_value(files).unwrap()
}

/// Opens a file/directory save dialog.
///
/// The selected paths are added to the filesystem and asset protocol allowlist scopes.
/// When security is mroe important than the ease of use of this API,
/// prefer writing a dedicated command instead.
///
/// Note that the allowlist scope change is not persisited,
/// so the values are cleared when the applicaiton is restarted.
/// You can save it to the filessytem using the [tauri-plugin-persisted-scope](https://github.com/tauri-apps/tauri-plugin-persisted-scope).
///
/// # Example
///
/// ```rust,no_run
/// use tauri_api::dialog::{save, SaveDialogOptions};
///
/// let file = save(None).await;
/// ```
/// @param options Dialog options.
/// @returns File path, or `None` if user cancelled the dialog.
pub async fn save(options: Option<SaveDialogOptions>) -> Option<PathBuf> {
    let path = inner::save(serde_wasm_bindgen::to_value(&options).unwrap()).await;
    serde_wasm_bindgen::from_value(path).unwrap()
}

mod inner {
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

    #[wasm_bindgen(module = "/dist/dialog.js")]
    extern "C" {
        pub async fn ask(message: &str, options: JsValue) -> JsValue;
        pub async fn confirm(message: &str, options: JsValue) -> JsValue;
        pub async fn open(options: JsValue) -> JsValue;
        pub async fn open_multiple(options: JsValue) -> JsValue;
        pub async fn message(message: &str, option: JsValue);
        pub async fn save(options: JsValue) -> JsValue;
    }
}
