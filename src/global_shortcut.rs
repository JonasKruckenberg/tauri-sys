//! Register global shortcuts.
//! 
//! The APIs must be added to tauri.allowlist.globalShortcut in tauri.conf.json:
//! 
//! ```json
//! {
//!     "tauri": {
//!         "allowlist": {
//!             "globalShortcut": {
//!                 "all": true // enable all global shortcut APIs
//!             }
//!         }
//!     }
//! }
//! ```
//! It is recommended to allowlist only the APIs you use for optimal bundle size and security.

use wasm_bindgen::{prelude::Closure, JsValue};

/// Determines whether the given shortcut is registered by this application or not.
/// 
/// # Example
/// 
/// ```rust,no_run
/// use tauri_sys::global_shortcut::is_registered;
/// 
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let registered = is_registered("CommandOrControl+P").await?;
/// # Ok(())
/// # }
/// ```
pub async fn is_registered(shortcut: &str) -> crate::Result<bool> {
    let raw = inner::isRegistered(shortcut).await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Register a global shortcut.
/// 
/// # Example
/// 
/// ```rust,no_run
/// use tauri_sys::global_shortcut::register;
/// use web_sys::console;
/// 
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// register("CommandOrControl+Shift+C", |_| {
///     console::log_1(&"Shortcut triggered".into());
/// }).await?;
/// # Ok(())
/// # }
/// ```
pub async fn register<H>(shortcut: &str, mut handler: H) -> crate::Result<()>
where
    H: FnMut(&str) + 'static,
{
    let closure = Closure::<dyn FnMut(JsValue)>::new(move |raw: JsValue| {
        let raw = raw.as_string().unwrap();
        (handler)(raw.as_str())
    });

    inner::register(shortcut, &closure).await?;

    closure.forget();

    Ok(())
}

/// Register a collection of global shortcuts.
/// 
/// # Example
/// 
/// ```rust,no_run
/// use tauri_sys::global_shortcut::register_all;
/// 
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let registered = register_all(["CommandOrControl+Shift+C", "Ctrl+Alt+F12"], |shortcut| {
///     console::log_1(&format!("Shortcut {} triggered", shortcut).into());
/// }).await?;
/// # Ok(())
/// # }
/// ```
pub async fn register_all<'a, I, H>(shortcuts: I, mut handler: H) -> crate::Result<()>
where
    I: IntoIterator<Item = &'a str>,
    H: FnMut(&str) + 'static,
{
    let shortcuts = shortcuts.into_iter().map(JsValue::from_str).collect();

    let closure = Closure::<dyn FnMut(JsValue)>::new(move |raw: JsValue| {
        let raw = raw.as_string().unwrap();
        (handler)(raw.as_str())
    });

    inner::registerAll(shortcuts, &closure).await?;

    closure.forget();

    Ok(())
}

/// Unregister a global shortcut.
/// 
/// # Example
/// 
/// ```rust,no_run
/// use tauri_sys::global_shortcut::unregister;
/// 
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// unregister("CmdOrControl+Space").await?;
/// # Ok(())
/// # }
/// ```
pub async fn unregister(shortcut: &str) -> crate::Result<()> {
    inner::unregister(shortcut).await?;

    Ok(())
}

/// Unregisters all shortcuts registered by the application.
/// 
/// # Example
/// 
/// ```rust,no_run
/// use tauri_sys::global_shortcut::unregister_all;
/// 
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// unregister_all().await?;
/// # Ok(())
/// # }
/// ```
pub async fn unregister_all() -> crate::Result<()> {
    inner::unregisterAll().await?;

    Ok(())
}

mod inner {
    use js_sys::Array;
    use wasm_bindgen::{
        prelude::{wasm_bindgen, Closure},
        JsValue,
    };

    #[wasm_bindgen(module = "/src/globalShortcut.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub async fn isRegistered(shortcut: &str) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn register(
            shortcut: &str,
            handler: &Closure<dyn FnMut(JsValue)>,
        ) -> Result<(), JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn registerAll(
            shortcuts: Array,
            handler: &Closure<dyn FnMut(JsValue)>,
        ) -> Result<(), JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn unregister(shortcut: &str) -> Result<(), JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn unregisterAll() -> Result<(), JsValue>;
    }
}
