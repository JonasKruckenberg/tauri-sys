use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize)]
struct DialogFilter<'a> {
    extensions: &'a [&'a str],
    name: &'a str,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename = "camelCase")]
pub struct FileDialogBuilder<'a> {
    default_path: Option<&'a Path>,
    filters: Vec<DialogFilter<'a>>,
    title: Option<&'a str>,
    directory: bool,
    multiple: bool,
    recursive: bool,
}

impl<'a> FileDialogBuilder<'a> {
    /// Gets the default file dialog builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set starting file name or directory of the dialog.
    pub fn set_default_path(&mut self, default_path: &'a Path) {
        self.default_path = Some(default_path);
    }

    /// If directory is true, indicates that it will be read recursively later.
    /// Defines whether subdirectories will be allowed on the scope or not.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tauri_sys::dialog::FileDialogBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let _builder = FileDialogBuilder::new().set_recursive(true);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_recursive(&mut self, recursive: bool) {
        self.recursive = recursive;
    }

    /// Set the title of the dialog.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tauri_sys::dialog::FileDialogBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let _builder = FileDialogBuilder::new().set_title("Test Title");
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_title(&mut self, title: &'a str) {
        self.title = Some(title);
    }

    /// Add file extension filter. Takes in the name of the filter, and list of extensions
    ///
    /// # Example
    ///
    /// ```rust
    /// use tauri_sys::dialog::FileDialogBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let _builder = FileDialogBuilder::new().add_filter("Image", &["png", "jpeg"]);
    /// # Ok(())
    /// # }
    /// ```
    pub fn add_filter(&mut self, name: &'a str, extensions: &'a [&'a str]) {
        self.filters.push(DialogFilter { name, extensions });
    }

    /// Add many file extension filters.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tauri_sys::dialog::FileDialogBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let _builder = FileDialogBuilder::new().add_filters(&[("Image", &["png", "jpeg"]),("Video", &["mp4"])]);
    /// # Ok(())
    /// # }
    /// ```
    pub fn add_filters(&mut self, filters: impl IntoIterator<Item = (&'a str, &'a [&'a str])>) {
        for (name, extensions) in filters.into_iter() {
            self.filters.push(DialogFilter {
                name: name.as_ref(),
                extensions,
            });
        }
    }

    /// Shows the dialog to select a single file.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use tauri_sys::dialog::FileDialogBuilder;
    ///
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let file = FileDialogBuilder::new().pick_file().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn pick_file(self) -> crate::Result<Option<PathBuf>> {
        let raw = inner::open(serde_wasm_bindgen::to_value(&self)?).await?;

        Ok(serde_wasm_bindgen::from_value(raw)?)
    }

    /// Shows the dialog to select multiple files.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use tauri_sys::dialog::FileDialogBuilder;
    ///
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let files = FileDialogBuilder::new().pick_files().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn pick_files(mut self) -> crate::Result<Option<Vec<PathBuf>>> {
        self.multiple = true;

        let raw = inner::open(serde_wasm_bindgen::to_value(&self)?).await?;

        Ok(serde_wasm_bindgen::from_value(raw)?)
    }

    /// Shows the dialog to select a single folder.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use tauri_sys::dialog::FileDialogBuilder;
    ///
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let files = FileDialogBuilder::new().pick_folder().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn pick_folder(mut self) -> crate::Result<Option<PathBuf>> {
        self.directory = true;

        let raw = inner::open(serde_wasm_bindgen::to_value(&self)?).await?;

        Ok(serde_wasm_bindgen::from_value(raw)?)
    }

    /// Shows the dialog to select multiple folders.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use tauri_sys::dialog::FileDialogBuilder;
    ///
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let files = FileDialogBuilder::new().pick_folders().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn pick_folders(mut self) -> crate::Result<Option<Vec<PathBuf>>> {
        self.directory = true;
        self.multiple = true;

        let raw = inner::open(serde_wasm_bindgen::to_value(&self)?).await?;

        Ok(serde_wasm_bindgen::from_value(raw)?)
    }

    /// Open a file/directory save dialog.
    ///
    /// The selected path is added to the filesystem and asset protocol allowlist scopes.
    /// When security is more important than the easy of use of this API, prefer writing a dedicated command instead.
    ///
    /// Note that the allowlist scope change is not persisted, so the values are cleared when the application is restarted.
    /// You can save it to the filesystem using tauri-plugin-persisted-scope.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use tauri_sys::dialog::FileDialogBuilder;
    ///
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let file = FileDialogBuilder::new().save().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn save(self) -> crate::Result<Option<PathBuf>> {
        let raw = inner::save(serde_wasm_bindgen::to_value(&self)?).await?;

        Ok(serde_wasm_bindgen::from_value(raw)?)
    }
}

#[derive(Debug, Default)]
pub enum MessageDialogType {
    #[default]
    Info,
    Warning,
    Error,
}

impl Serialize for MessageDialogType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            MessageDialogType::Info => serializer.serialize_str("info"),
            MessageDialogType::Warning => serializer.serialize_str("warning"),
            MessageDialogType::Error => serializer.serialize_str("error"),
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct MessageDialogBuilder<'a> {
    title: Option<&'a str>,
    r#type: MessageDialogType,
}

impl<'a> MessageDialogBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the title of the dialog.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tauri_sys::dialog::MessageDialogBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let _builder = MessageDialogBuilder::new().set_title("Test Title");
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_title(&mut self, title: &'a str) {
        self.title = Some(title);
    }

    /// Set the type of the dialog.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tauri_sys::dialog::{MessageDialogBuilder,MessageDialogType};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let _builder = MessageDialogBuilder::new().set_type(MessageDialogType::Error);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_type(&mut self, r#type: MessageDialogType) {
        self.r#type = r#type;
    }

    /// Shows a message dialog with an `Ok` button.
    /// 
    /// # Example
    ///
    /// ```rust,no_run
    /// use tauri_sys::dialog::MessageDialogBuilder;
    ///
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let file = MessageDialogBuilder::new().message("Tauri is awesome").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn message(self, message: &str) -> crate::Result<()> {
        Ok(inner::message(message, serde_wasm_bindgen::to_value(&self)?).await?)
    }

    /// Shows a question dialog with `Yes` and `No` buttons.
    /// 
    /// # Example
    ///
    /// ```rust,no_run
    /// use tauri_sys::dialog::MessageDialogBuilder;
    ///
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let confirmation = MessageDialogBuilder::new().ask("Are you sure?").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn ask(self, message: &str) -> crate::Result<bool> {
        let raw = inner::ask(message, serde_wasm_bindgen::to_value(&self)?).await?;

        Ok(serde_wasm_bindgen::from_value(raw)?)
    }

    /// Shows a question dialog with `Ok` and `Cancel` buttons.
    /// 
    /// # Example
    ///
    /// ```rust,no_run
    /// use tauri_sys::dialog::MessageDialogBuilder;
    ///
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let confirmation = MessageDialogBuilder::new().confirm("Are you sure?").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn confirm(self, message: &str) -> crate::Result<bool> {
        let raw = inner::confirm(message, serde_wasm_bindgen::to_value(&self)?).await?;

        Ok(serde_wasm_bindgen::from_value(raw)?)
    }
}

// //! User interaction with the file system using dialog boxes.
// //!
// //! # Example
// //!
// //! ```rust,no_run
// //! use tauri_api::dialog::open;
// //!
// //! let path = open(None).await;
// //! ```
// use serde::Serialize;
// use std::path::PathBuf;

// /// Extension filter for the file dialog.
// ///
// /// # Example
// ///
// /// ```rust,no_run
// /// let filter = DialogFilter {
// ///   extension: vec![".jpg", ".jpeg", ".png", ".bmp"],
// ///   name: "images",
// /// };
// /// ```
// #[derive(Serialize)]
// pub struct DialogFilter {
//     /// Extensions to filter, without a `.` prefix.
//     pub extensions: Vec<String>,

//     /// Filter name
//     pub name: String,
// }

// /// Types of a [`message`] dialog.
// #[derive(Serialize)]
// pub enum MessageDialogType {
//     Error,
//     Info,
//     Warning,
// }

// /// Options for the [`message`] dialog.
// #[derive(Serialize)]
// pub struct MessageDialogOptions {
//     /// The title of the dialog. Defaults to the app name.
//     pub title: Option<String>,

//     /// The type of the dialog. Defaults to MessageDialogType::Info.
//     #[serde(rename(serialize = "type"))]
//     pub kind: MessageDialogType,
// }

// impl MessageDialogOptions {
//     /// Creates a new `MessageDialogOptions` with sensible default values.
//     pub fn new() -> Self {
//         Self {
//             title: None,
//             kind: MessageDialogType::Info,
//         }
//     }
// }

// /// Options for an [`open`] dialog.
// #[derive(Serialize)]
// pub struct OpenDialogOptions {
//     /// Initial directory or file path.
//     #[serde(rename(serialize = "defaultPath"))]
//     pub default_path: Option<PathBuf>,

//     /// Whether the dialog is a directory selection or not.
//     pub directory: bool,

//     /// The filters of the dialog.
//     pub filters: Vec<DialogFilter>,

//     /// Whether the dialog allows multiple selection or not.
//     pub multiple: bool,

//     /// If `directory` is `true`, indicatees that it will be read recursivley later.
//     /// Defines whether subdirectories will be allowed on the scope or not.
//     pub recursive: bool,

//     /// The title of the dialog window.
//     pub title: Option<String>,
// }

// impl OpenDialogOptions {
//     /// Creates a new `OpenDialogOptions` with sensible default values.
//     pub fn new() -> Self {
//         Self {
//             default_path: None,
//             directory: false,
//             filters: Vec::new(),
//             multiple: false,
//             recursive: false,
//             title: None,
//         }
//     }
// }

// /// Options for the save dialog.
// #[derive(Serialize)]
// pub struct SaveDialogOptions {
//     /// Initial directory of the file path.
//     /// If it's not a directory path, the dialog interface will change to that folder.
//     /// If it's not an existing directory, the file name will be set to the dialog's
//     /// file name input and the dialog will be set to the parent folder.
//     #[serde(rename(serialize = "defaultPath"))]
//     pub default_path: Option<PathBuf>,

//     /// The filters of the dialog.
//     pub filters: Vec<DialogFilter>,

//     /// The title of the dialog window.
//     pub title: Option<String>,
// }

// impl SaveDialogOptions {
//     /// Creates a new `SaveDialogOptions` with sensible default values.
//     pub fn new() -> Self {
//         Self {
//             default_path: None,
//             filters: Vec::new(),
//             title: None,
//         }
//     }
// }

// /// Show a question dialog with `Yes` and `No` buttons.
// ///
// /// # Example
// ///
// /// ```rust,no_run
// /// use tauri_api::dialog::{ask, MessageDialogOptions};
// ///
// /// let yes = ask("Are you sure?", None).await;
// /// ```
// /// @param message Message to display.
// /// @param options Dialog options.
// /// @returns Whether the user selected `Yes` or `No`.
// #[inline(always)]
// pub async fn ask(message: &str, options: Option<MessageDialogOptions>) -> crate::Result<bool> {
//     let js_val = inner::ask(message, serde_wasm_bindgen::to_value(&options)?).await?;

//     Ok(serde_wasm_bindgen::from_value(js_val)?)
// }

// /// Shows a question dialog with `Ok` and `Cancel` buttons.
// ///
// /// # Example
// ///
// /// ```rust,no_run
// /// use tauri_api::dialog::{confirm, MessageDialogOptions};
// ///
// /// let confirmed = confirm("Are you sure?", None).await;
// /// ```
// /// @returns Whether the user selelced `Ok` or `Cancel`.
// pub async fn confirm(message: &str, options: Option<MessageDialogOptions>) -> crate::Result<bool> {
//     let js_val = inner::confirm(message, serde_wasm_bindgen::to_value(&options)?).await?;

//     Ok(serde_wasm_bindgen::from_value(js_val)?)
// }

// /// Shows a message dialog with an `Ok` button.
// ///
// /// # Example
// ///
// /// ```rust,no_run
// /// use tauri_api::dialog::{message, MessageDialogOptions};
// ///
// /// message("Tauri is awesome", None).await;
// /// ```
// /// @param message Message to display.
// /// @param options Dialog options.
// /// @returns Promise resolved when user closes the dialog.
// pub async fn message(message: &str, options: Option<MessageDialogOptions>) -> crate::Result<()> {
//     Ok(inner::message(message, serde_wasm_bindgen::to_value(&options)?).await?)
// }

// /// Opens a file/directory selection dialog for a single file.
// /// `multiple` field of [`options`](OpenDialogOptions) must be `false`, if provided.
// ///
// /// The selected paths are added to the filesystem and asset protocol allowlist scopes.
// /// When security is mroe important than the ease of use of this API,
// /// prefer writing a dedicated command instead.
// ///
// /// Note that the allowlist scope change is not persisited,
// /// so the values are cleared when the applicaiton is restarted.
// /// You can save it to the filessytem using the [tauri-plugin-persisted-scope](https://github.com/tauri-apps/tauri-plugin-persisted-scope).
// ///
// /// # Example
// ///
// /// ```rust,no_run
// /// use tauri_api::dialog::{open, OpenDialogOptions};
// ///
// /// let file = open(None).await;
// ///
// /// let mut opts = OpenDialogOptions::new();
// /// opts.directory = true;
// /// let dir = open(Some(opts)).await;
// /// ```
// /// @param options Dialog options.
// /// @returns List of file paths, or `None` if user cancelled the dialog.
// pub async fn open(options: Option<OpenDialogOptions>) -> crate::Result<Option<PathBuf>> {
//     let file = inner::open(serde_wasm_bindgen::to_value(&options)?).await?;

//     Ok(serde_wasm_bindgen::from_value(file)?)
// }

// /// Opens a file/directory selection dialog for multiple files.
// /// `multiple` field of [`options`](OpenDialogOptions) must be `true`, if provided.
// ///
// /// The selected paths are added to the filesystem and asset protocol allowlist scopes.
// /// When security is mroe important than the ease of use of this API,
// /// prefer writing a dedicated command instead.
// ///
// /// Note that the allowlist scope change is not persisited,
// /// so the values are cleared when the applicaiton is restarted.
// /// You can save it to the filessytem using the [tauri-plugin-persisted-scope](https://github.com/tauri-apps/tauri-plugin-persisted-scope).
// ///
// /// # Example
// ///
// /// ```rust,no_run
// /// use tauri_api::dialog::{open, OpenDialogOptions};
// ///
// /// let files = open_multiple(None).await;
// ///
// /// let mut opts = OpenDialogOptions::new();
// /// opts.multiple = true;
// /// opts.directory = true;
// /// let dirs = open(Some(opts)).await;
// /// ```
// /// @param options Dialog options.
// /// @returns List of file paths, or `None` if user cancelled the dialog.
// pub async fn open_multiple(
//     options: Option<OpenDialogOptions>,
// ) -> crate::Result<Option<Vec<PathBuf>>> {
//     let files = inner::open_multiple(serde_wasm_bindgen::to_value(&options)?).await?;

//     Ok(serde_wasm_bindgen::from_value(files)?)
// }

// /// Opens a file/directory save dialog.
// ///
// /// The selected paths are added to the filesystem and asset protocol allowlist scopes.
// /// When security is mroe important than the ease of use of this API,
// /// prefer writing a dedicated command instead.
// ///
// /// Note that the allowlist scope change is not persisited,
// /// so the values are cleared when the applicaiton is restarted.
// /// You can save it to the filessytem using the [tauri-plugin-persisted-scope](https://github.com/tauri-apps/tauri-plugin-persisted-scope).
// ///
// /// # Example
// ///
// /// ```rust,no_run
// /// use tauri_api::dialog::{save, SaveDialogOptions};
// ///
// /// let file = save(None).await;
// /// ```
// /// @param options Dialog options.
// /// @returns File path, or `None` if user cancelled the dialog.
// pub async fn save(options: Option<SaveDialogOptions>) -> crate::Result<Option<PathBuf>> {
//     let path = inner::save(serde_wasm_bindgen::to_value(&options)?).await?;

//     Ok(serde_wasm_bindgen::from_value(path)?)
// }

mod inner {
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

    #[wasm_bindgen(module = "/src/dialog.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub async fn ask(message: &str, options: JsValue) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn confirm(message: &str, options: JsValue) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn open(options: JsValue) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn message(message: &str, option: JsValue) -> Result<(), JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn save(options: JsValue) -> Result<JsValue, JsValue>;
    }
}
