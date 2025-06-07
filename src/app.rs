use crate::core::invoke;

pub use image::{Image, ImageSize};

#[derive(Clone, Copy)]
pub enum Theme {
    Light,
    Dark,
    System,
}

/// Get the app's name as defined in `src-tauri/tauri.conf.json`.
pub async fn get_name() -> String {
    serde_wasm_bindgen::from_value(inner::get_name().await).unwrap()
}

pub async fn get_tauri_version() -> String {
    serde_wasm_bindgen::from_value(inner::get_tauri_version().await).unwrap()
}

/// Get the app's version.
pub async fn get_version() -> String {
    serde_wasm_bindgen::from_value(inner::get_version().await).unwrap()
}

pub async fn default_window_icon() -> Option<Image> {
    invoke::<Option<u64>>("plugin:app|default_window_icon", ())
        .await
        .map(|rid| Image::from_rid(rid))
}

/// Set the apps theme.
///
/// # Note
/// + Requires the `core:app:allow-set-theme` permission.
pub async fn set_theme(theme: Theme) {
    let theme = match theme {
        Theme::Light => Some("light"),
        Theme::Dark => Some("dark"),
        Theme::System => None,
    };
    inner::set_theme(theme).await;
}

/// Hide the app.
///
/// # Note
/// + Requires the `core:app:allow-app-hide` permission.
#[cfg(target_os = "macos")]
pub async fn hide() -> Result<(), JsValue> {
    inner::hide().await
}

/// Show the app. Does not automatically focus any specific app window.
///
/// # Note
/// + Requires the `core:app:allow-app-show` permission.
#[cfg(target_os = "macos")]
pub async fn show() -> Result<(), JsValue> {
    inner::show().await
}

mod image {
    use crate::core::{Resource, invoke};
    use serde::Serialize;

    #[derive(serde::Deserialize, Clone)]
    pub struct ImageSize {
        width: u64,
        height: u64,
    }

    impl ImageSize {
        pub fn width(&self) -> u64 {
            self.width
        }

        pub fn height(&self) -> u64 {
            self.height
        }
    }

    #[derive(derive_more::Deref, Clone)]
    pub struct Image {
        #[deref]
        resource: Resource,
    }

    impl Image {
        pub fn from_rid(rid: u64) -> Self {
            Self {
                resource: Resource::new(rid),
            }
        }

        /// Returns the RGBA data for this image, in row-major order from top to bottom.
        pub async fn rgba(&self) -> Vec<u8> {
            #[derive(Serialize)]
            struct Args {
                rid: u64,
            }

            invoke("plugin:image|rgba", Args { rid: self.rid() }).await
        }

        pub async fn size(&self) -> ImageSize {
            #[derive(Serialize)]
            struct Args {
                rid: u64,
            }

            invoke("plugin:image|size", Args { rid: self.rid() }).await
        }
    }
}

mod inner {
    use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

    #[wasm_bindgen(module = "/src/app.js")]
    extern "C" {
        #[wasm_bindgen(js_name = "getName")]
        pub async fn get_name() -> JsValue;
        #[wasm_bindgen(js_name = "getTauriVersion")]
        pub async fn get_tauri_version() -> JsValue;
        #[wasm_bindgen(js_name = "getVersion")]
        pub async fn get_version() -> JsValue;
        #[wasm_bindgen(js_name = "setTheme")]
        pub async fn set_theme(theme: Option<&str>) -> JsValue;
        #[cfg(target_os = "macos")]
        #[wasm_bindgen(catch)]
        pub async fn hide() -> Result<(), JsValue>;
        #[cfg(target_os = "macos")]
        #[wasm_bindgen(catch)]
        pub async fn show() -> Result<(), JsValue>;
    }
}
