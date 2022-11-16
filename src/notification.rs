use serde::{Deserialize, Serialize};

pub async fn is_permission_granted() -> crate::Result<bool> {
    let raw = inner::isPermissionGranted().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

pub async fn request_permission() -> crate::Result<Permission> {
    let raw = inner::requestPermission().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    #[default]
    Default,
    Granted,
    Denied,
}

impl<'de> Deserialize<'de> for Permission {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "default" => Ok(Permission::Default),
            "granted" => Ok(Permission::Granted),
            "denied" => Ok(Permission::Denied),
            _ => Err(serde::de::Error::custom(
                "expected one of default, granted, denied",
            )),
        }
    }
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
