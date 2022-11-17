use serde::{Deserialize, Serialize};

#[inline(always)]
pub async fn is_permission_granted() -> crate::Result<bool> {
    let raw = inner::isPermissionGranted().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

#[inline(always)]
pub async fn request_permission() -> crate::Result<Permission> {
    let raw = inner::requestPermission().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

#[derive(Debug, Deserialize, Default, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    #[default]
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "granted")]
    Granted,
    #[serde(rename = "denied")]
    Denied,
}

#[derive(Debug, Default, Serialize)]
pub struct Notification<'a> {
    body: Option<&'a str>,
    title: Option<&'a str>,
    icon: Option<&'a str>
} 

impl<'a> Notification<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_title(&mut self, title: &'a str) {
        self.title = Some(title);
    }

    pub fn set_body(&mut self, body: &'a str) {
        self.body = Some(body);
    }

    pub fn set_icon(&mut self, icon: &'a str) {
        self.icon = Some(icon);
    }

    #[inline(always)]
    pub fn show(&self) -> crate::Result<()> {
        inner::sendNotification(serde_wasm_bindgen::to_value(&self)?)?;
        
        Ok(())
    }
}

mod inner {
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

    #[wasm_bindgen(module = "/src/notification.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub async fn isPermissionGranted() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn requestPermission() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub fn sendNotification(notification: JsValue) -> Result<(), JsValue>;
    }
}
