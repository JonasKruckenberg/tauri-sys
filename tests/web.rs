use serde::Deserialize;
use serde::Serialize;
use tauri_sys::{mocks::mock_ipc, tauri};
use wasm_bindgen::JsError;
use wasm_bindgen_test::wasm_bindgen_test;
use wasm_bindgen_test::wasm_bindgen_test_configure;

fn main() {
    wasm_bindgen_test_configure!(run_in_browser);
}

#[wasm_bindgen_test]
async fn testinvoke() -> Result<(), Box<dyn std::error::Error>> {
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
