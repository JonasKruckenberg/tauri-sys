//! A plugin for Tauri that helps position your windows at well-known locations.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Position {
    #[serde(rename = "0")]
    TopLeft,
    #[serde(rename = "1")]
    TopRight,
    #[serde(rename = "2")]
    BottomLeft,
    #[serde(rename = "3")]
    BottomRight,
    #[serde(rename = "4")]
    TopCenter,
    #[serde(rename = "5")]
    BottomCenter,
    #[serde(rename = "6")]
    LeftCenter,
    #[serde(rename = "7")]
    RightCenter,
    #[serde(rename = "8")]
    Center,
    #[serde(rename = "9")]
    TrayLeft,
    #[serde(rename = "10")]
    TrayBottomLeft,
    #[serde(rename = "11")]
    TrayRight,
    #[serde(rename = "12")]
    TrayBottomRight,
    #[serde(rename = "13")]
    TrayCenter,
    #[serde(rename = "14")]
    TrayBottomCenter,
}

/// Moves the `Window` to the given position using `WindowExt.move_window()`
/// * All positions are relative to the **current** screen.
#[inline(always)]
pub async fn move_window(position: Position) -> crate::Result<()> {
    inner::move_window(serde_wasm_bindgen::to_value(&position)?).await?;

    Ok(())
}

mod inner {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/src/positioner.js")]
    extern "C" {
        #[wasm_bindgen(catch, js_name = "moveWindow")]
        pub async fn move_window(position: JsValue) -> Result<(), JsValue>;
    }
}
