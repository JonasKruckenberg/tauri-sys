//! Common functionality
use std::path::PathBuf;

use serde::{de::DeserializeOwned, Serialize};
use serde_wasm_bindgen as swb;

pub async fn invoke<T>(command: &str, args: impl Serialize) -> T
where
    T: DeserializeOwned,
{
    let value = inner::invoke(command, swb::to_value(&args).unwrap()).await;
    swb::from_value(value).unwrap()
}

pub async fn invoke_result<T, E>(command: &str, args: impl Serialize) -> Result<T, E>
where
    T: DeserializeOwned,
    E: DeserializeOwned,
{
    inner::invoke_result(command, swb::to_value(&args).unwrap())
        .await
        .map(|val| swb::from_value(val).unwrap())
        .map_err(|err| swb::from_value(err).unwrap())
}

pub fn convert_file_src(file_path: impl AsRef<str>) -> String {
    inner::convert_file_src(file_path.as_ref(), "asset")
        .as_string()
        .unwrap()
}

pub fn convert_file_src_with_protocol(
    file_path: impl AsRef<str>,
    protocol: impl AsRef<str>,
) -> String {
    inner::convert_file_src(file_path.as_ref(), protocol.as_ref())
        .as_string()
        .unwrap()
}

mod inner {
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

    #[wasm_bindgen(module = "/src/core.js")]
    extern "C" {
        #[wasm_bindgen]
        pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
        #[wasm_bindgen(js_name = "invoke", catch)]
        pub async fn invoke_result(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(js_name = "convertFileSrc")]
        pub fn convert_file_src(filePath: &str, protocol: &str) -> JsValue;
    }
}
