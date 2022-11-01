use serde::Deserialize;
use serde::Serialize;
use tauri_sys::{mocks::mock_ipc, tauri};
use wasm_bindgen::JsError;
use wasm_bindgen_test::wasm_bindgen_test;
use wasm_bindgen_test::wasm_bindgen_test_configure;

macro_rules! bail {
    ($e:expr) => {
        return Err(JsError::new($e));
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err(JsError::new(&format!($fmt, $($arg)*)));
    };
}

macro_rules! ensure {
    ($cond:expr) => {
        if !($cond) {
            bail!("{}", _failure__stringify!($cond));
        }
    };
    ($cond:expr, $e:expr) => {
        if !($cond) {
            bail!($e);
        }
    };
    ($cond:expr, $fmt:expr, $($arg:tt)*) => {
        if !($cond) {
            bail!($fmt, $($arg)*);
        }
    };
}

macro_rules! _failure__stringify {
    ($($inner:tt)*) => {
        stringify! { $($inner)* }
    }
}

#[derive(Deserialize)]
struct ApiRequestInner {
    cmd: String,
}

#[derive(Deserialize)]
struct ApiRequest {
    #[serde(rename = "__tauriModule")]
    __tauri_module: String,
    message: ApiRequestInner,
}

fn main() {
    wasm_bindgen_test_configure!(run_in_browser);
}

/**
 * App module
 */

#[wasm_bindgen_test]
async fn test_get_version() {
    use tauri_sys::app::get_version;

    mock_ipc(|cmd, payload| {
        ensure!(cmd.as_str() == "tauri", "unknown command");

        let payload: ApiRequest = serde_wasm_bindgen::from_value(payload).unwrap();

        ensure!(payload.__tauri_module == "App");
        ensure!(payload.message.cmd == "getAppVersion");

        Ok("1.0.0")
    });

    let version = get_version().await;

    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0)
}

/**
 * Tauri module
 */

#[wasm_bindgen_test]
async fn test_invoke() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Serialize, Deserialize)]
    struct AddPayload {
        a: u32,
        b: u32,
    }

    mock_ipc(|cmd, payload| match cmd.as_str() {
        "add" => {
            let args: AddPayload = serde_wasm_bindgen::from_value(payload).unwrap();

            Ok(args.a + args.b)
        }
        _ => Err(JsError::new("Unknown command")),
    });

    let out = tauri::invoke::<_, u32>("add", &AddPayload { a: 12, b: 15 }).await?;

    assert_eq!(out, 27);

    Ok(())
}
