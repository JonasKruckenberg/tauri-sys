use std::{cell::RefCell, rc::Rc};

use futures::{channel::mpsc, Stream, StreamExt};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::Closure, JsValue};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateManifest {
    pub available: bool,
    pub body: String,
    pub current_version: String,
    pub date: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Update(inner::Update);

impl Update {
    #[inline(always)]
    pub async fn download_and_install(&self) -> crate::Result<impl Stream<Item = ()>> {
        let (tx, rx) = mpsc::unbounded();

        let closure = Closure::<dyn FnMut(JsValue)>::new(move |_| {
            let _ = tx.unbounded_send(());
        });

        self.0.downloadAndInstall(&closure).await?;

        closure.forget();

        Ok(Listen { rx })
    }
}

#[inline(always)]
pub async fn check() -> crate::Result<Update> {
    let raw = inner::check().await?;

    Ok(Update(inner::Update::new(raw)))
}

struct Listen<T> {
    pub rx: mpsc::UnboundedReceiver<T>,
}

impl<T> Stream for Listen<T> {
    type Item = T;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.rx.poll_next_unpin(cx)
    }
}

mod inner {
    use wasm_bindgen::{
        prelude::{wasm_bindgen, Closure},
        JsValue,
    };

    #[wasm_bindgen(module = "/src/updater.js")]
    extern "C" {
        #[derive(Debug, Clone, PartialEq)]
        pub type Update;
        #[wasm_bindgen(constructor)]
        pub fn new(metadata: JsValue) -> Update;
        #[wasm_bindgen(method, catch)]
        pub async fn downloadAndInstall(
            this: &Update,
            handler: &Closure<dyn FnMut(JsValue)>,
        ) -> Result<JsValue, JsValue>;
    }

    #[wasm_bindgen(module = "/src/updater.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub async fn check() -> Result<JsValue, JsValue>;
    }
}
