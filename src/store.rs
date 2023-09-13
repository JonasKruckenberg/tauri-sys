//! Simple, persistent key-value store.

use crate::event::Listen;
use futures::{channel::mpsc, Stream};
use serde::{de::DeserializeOwned, Serialize};
use std::path::PathBuf;
use wasm_bindgen::{prelude::Closure, JsValue};

#[derive(Debug, Clone, PartialEq)]
pub struct KeyValuePair<T> {
    key: String,
    value: Option<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Store(inner::Store);

impl Store {
    pub fn new(path: &PathBuf) -> Self {
        Self(inner::Store::new(path.to_str().unwrap()))
    }

    /// Inserts a key-value pair into the store.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use tauri_api::store::Store;
    ///
    /// let store = Store::new("/home/user/.local/app/settings.dat");
    ///
    /// store.set("dark_mode", true).await;
    /// ```
    pub async fn set<T: Serialize>(self, key: &str, value: T) -> crate::Result<()> {
        let serialized_value = serde_wasm_bindgen::to_value(&value)?;

        self.0.set(key, serialized_value).await?;

        Ok(())
    }

    /// Returns the value for the given `key` or None if the key does not exist.
    ///
    /// ```rust,no_run
    /// use tauri_api::store::Store;
    ///
    /// let store = Store::new("/home/user/.local/app/settings.dat");
    ///
    /// let is_dark_mode = store.get("dark_mode").await?.unwrap_or_default();
    /// ```
    pub async fn get<T>(self, key: &str) -> crate::Result<Option<T>>
    where
        T: DeserializeOwned + 'static,
    {
        let value = self.0.get(key).await?;

        if value.is_null() {
            return Ok(None);
        }

        let deserialized_value = serde_wasm_bindgen::from_value(self.0.get(key).await?)?;

        Ok(Some(deserialized_value))
    }

    /// Returns `true` if the given `key` exists in the store.
    ///
    /// ```rust,no_run
    /// use tauri_api::store::Store;
    ///
    /// let store = Store::new("/home/user/.local/app/settings.dat");
    ///
    /// let exists = store.has("dark_mode").await.unwrap_or_default();
    /// ```
    pub async fn has(self, key: &str) -> crate::Result<bool> {
        Ok(serde_wasm_bindgen::from_value(self.0.has(key).await?)?)
    }

    /// Removes a key-value pair from the store.
    pub async fn delete(self, key: &str) -> crate::Result<bool> {
        Ok(serde_wasm_bindgen::from_value(self.0.delete(key).await?)?)
    }

    /// Clears the store, removing all key-value pairs.
    ///
    /// Note: To clear the storage and reset it to it's `default` value, use `reset` instead.
    pub async fn clear(self) -> crate::Result<()> {
        self.0.clear().await?;

        Ok(())
    }

    /// Resets the store to it's `default` value.
    ///
    /// If no default value has been set, this method behaves identical to `clear`.
    pub async fn reset(self) -> crate::Result<()> {
        self.0.reset().await?;

        Ok(())
    }

    /// Returns a list of all key in the store.
    pub async fn keys(self) -> crate::Result<Vec<String>> {
        Ok(serde_wasm_bindgen::from_value(self.0.keys().await?)?)
    }

    /// Returns a list of all values in the store.
    ///
    /// Note: This method returns a JsValue
    pub async fn values(self) -> crate::Result<JsValue> {
        Ok(self.0.values().await?)
    }

    /// Returns a list of all entries in the store.
    ///
    /// Note: This method returns a JsValue
    pub async fn entries(self) -> crate::Result<JsValue> {
        Ok(self.0.entries().await?)
    }

    /// Returns the number of key-value pairs in the store.
    pub async fn length(self) -> crate::Result<usize> {
        Ok(serde_wasm_bindgen::from_value(self.0.length().await?)?)
    }

    /// Attempts to load the on-disk state at the stores `path` into memory.
    ///
    /// This method is useful if the on-disk state was edited by the user and you want to synchronize the changes.
    ///
    /// Note: This method does not emit change events.
    pub async fn load(self) -> crate::Result<()> {
        self.0.load().await?;

        Ok(())
    }

    /// Saves the store to disk at the stores `path`.
    ///
    /// As the store is only persisted to disk before the apps exit, changes might be lost in a crash.
    /// This method lets you persist the store to disk whenever you deem necessary.
    pub async fn save(self) -> crate::Result<()> {
        self.0.save().await?;

        Ok(())
    }

    /// Listen to changes on a store key.
    ///
    /// The returned Future will automatically clean up it's underlying event listener when dropped, so no manual unlisten function needs to be called.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use tauri_api::store::Store;
    /// use web_sys::console;
    ///
    /// let store = Store::new("/home/user/.local/app/settings.dat");
    ///
    /// let mut events = Store.on_key_change::<String>("my_value")?;
    ///
    /// while let Some(event) = events.next().await {
    ///     console::log_1(&format!("New value: {}", event).into());
    /// }
    /// ```
    pub async fn on_key_change<T>(self, key: &str) -> crate::Result<impl Stream<Item = Option<T>>>
    where
        T: DeserializeOwned + 'static,
    {
        let (tx, rx) = mpsc::unbounded::<Option<T>>();

        let closure = Closure::<dyn FnMut(JsValue)>::new(move |raw| {
            let _ = tx.unbounded_send(serde_wasm_bindgen::from_value(raw).unwrap_or(None));
        });
        let unlisten = self.0.onKeyChange(key, &closure).await?;
        closure.forget();

        Ok(Listen {
            rx,
            unlisten: js_sys::Function::from(unlisten),
        })
    }

    /// Listen to changes on the store.
    ///
    /// The returned Future will automatically clean up it's underlying event listener when dropped, so no manual unlisten function needs to be called.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use tauri_api::store::Store;
    /// use web_sys::console;
    ///
    /// let store = Store::new("/home/user/.local/app/settings.dat");
    ///
    /// let mut events = store.on_change::<String>()?;
    ///
    /// while let Some(event) = events.next().await {
    ///     console::log_1(&format!("Store changed - Key: {} - Value: {}", event.key, event.value).into());
    /// }
    /// ```
    pub async fn on_change<T>(self) -> crate::Result<impl Stream<Item = KeyValuePair<T>>>
    where
        T: DeserializeOwned + 'static,
    {
        let (tx, rx) = mpsc::unbounded::<KeyValuePair<T>>();

        let closure = Closure::<dyn FnMut(JsValue, JsValue)>::new(move |key, value| {
            let _ = tx.unbounded_send(KeyValuePair {
                key: serde_wasm_bindgen::from_value(key).unwrap(),
                value: serde_wasm_bindgen::from_value(value).unwrap_or(None),
            });
        });
        let unlisten = self.0.onChange(&closure).await?;
        closure.forget();

        Ok(Listen {
            rx,
            unlisten: js_sys::Function::from(unlisten),
        })
    }
}

mod inner {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/src/store.js")]
    extern "C" {
        #[derive(Debug, Clone, PartialEq)]
        pub type Store;

        #[wasm_bindgen(constructor)]
        pub fn new(path: &str) -> Store;

        #[wasm_bindgen(method, catch)]
        pub async fn set(this: &Store, key: &str, value: JsValue) -> Result<(), JsValue>;

        #[wasm_bindgen(method, catch)]
        pub async fn get(this: &Store, key: &str) -> Result<JsValue, JsValue>;

        #[wasm_bindgen(method, catch)]
        pub async fn has(this: &Store, key: &str) -> Result<JsValue, JsValue>;

        #[wasm_bindgen(method, catch)]
        pub async fn delete(this: &Store, key: &str) -> Result<JsValue, JsValue>;

        #[wasm_bindgen(method, catch)]
        pub async fn clear(this: &Store) -> Result<(), JsValue>;

        #[wasm_bindgen(method, catch)]
        pub async fn reset(this: &Store) -> Result<(), JsValue>;

        #[wasm_bindgen(method, catch)]
        pub async fn keys(this: &Store) -> Result<JsValue, JsValue>;

        #[wasm_bindgen(method, catch)]
        pub async fn values(this: &Store) -> Result<JsValue, JsValue>;

        #[wasm_bindgen(method, catch)]
        pub async fn entries(this: &Store) -> Result<JsValue, JsValue>;

        #[wasm_bindgen(method, catch)]
        pub async fn length(this: &Store) -> Result<JsValue, JsValue>;

        #[wasm_bindgen(method, catch)]
        pub async fn load(this: &Store) -> Result<JsValue, JsValue>;

        #[wasm_bindgen(method, catch)]
        pub async fn save(this: &Store) -> Result<JsValue, JsValue>;

        #[wasm_bindgen(method, catch)]
        pub async fn onKeyChange(
            this: &Store,
            key: &str,
            handler: &Closure<dyn FnMut(JsValue)>,
        ) -> Result<JsValue, JsValue>;

        #[wasm_bindgen(method, catch)]
        pub async fn onChange(
            this: &Store,
            handler: &Closure<dyn FnMut(JsValue, JsValue)>,
        ) -> Result<JsValue, JsValue>;

    }
}
