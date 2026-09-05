//! Common functionality
use serde::{Serialize, de::DeserializeOwned};
use serde_wasm_bindgen as swb;

pub use channel::{Channel, Message};
pub use resource::Resource;

#[cfg_attr(feature = "nightly", track_caller)]
pub async fn invoke<T>(command: &str, args: impl Serialize) -> T
where
    T: DeserializeOwned,
{
    let value = inner::invoke(command, swb::to_value(&args).unwrap()).await;
    swb::from_value(value).unwrap()
}

#[cfg_attr(feature = "nightly", track_caller)]
pub async fn invoke_result<T, E>(command: &str, args: impl Serialize) -> Result<T, E>
where
    T: DeserializeOwned,
    E: DeserializeOwned,
{
    inner::invoke_result(command, swb::to_value(&args).unwrap())
        .await
        .map(|val| swb::from_value(val).unwrap())
        .map_err(|err| {
            // A torn-down IPC bridge (e.g. a WKWebView closing on macOS while
            // wasm microtasks still run) rejects an in-flight invoke with a
            // raw JS value that is not a serialized `E`, so deserializing it
            // directly can fail. Fall back to treating the rejection as its
            // string form (which succeeds for the common `E = String`) before
            // giving up, so teardown noise cannot panic.
            swb::from_value::<E>(err.clone()).unwrap_or_else(|_| {
                let text = err.as_string().unwrap_or_else(|| format!("{err:?}"));
                swb::from_value::<E>(wasm_bindgen::JsValue::from_str(&text)).unwrap_or_else(|_| {
                    panic!("invoke_result({command:?}) rejection is not a valid E: {text}")
                })
            })
        })
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

pub fn is_tauri() -> bool {
    inner::is_tauri()
}

mod resource {
    use super::invoke;
    use serde::Serialize;

    #[derive(Clone)]
    /// A Rust backed resource.
    pub struct Resource {
        rid: u64,
    }

    impl Resource {
        pub fn new(rid: u64) -> Self {
            Self { rid }
        }

        pub fn rid(&self) -> u64 {
            self.rid
        }

        /// Destroy the resource.
        pub async fn close(self) {
            #[derive(Serialize)]
            struct Args {
                rid: u64,
            }

            invoke::<()>("plugin:resources|close", Args { rid: self.rid }).await;
        }
    }
}

mod channel {
    use super::inner;
    use futures::{Stream, StreamExt, channel::mpsc};
    use send_wrapper::SendWrapper;
    use serde::{Deserialize, Serialize, de::DeserializeOwned};
    use std::{
        collections::BTreeMap,
        pin::Pin,
        task::{Context, Poll},
    };
    use wasm_bindgen::{JsValue, prelude::Closure};

    #[derive(derive_more::Deref, Deserialize, Debug)]
    pub struct Message<T> {
        index: usize,
        end: Option<bool>,

        #[deref]
        message: Option<T>,
    }

    impl<T> Message<T> {
        pub fn index(&self) -> usize {
            self.index
        }

        /// # Returns
        /// If the message's `end` property was set to `true`.
        /// i.e. Is `Some(true)`.
        pub fn end(&self) -> bool {
            match self.end {
                Some(true) => true,
                _ => false,
            }
        }
    }

    /// Restores the strict `index` order of incoming messages.
    ///
    /// Tauri delivers small channel messages synchronously through
    /// `webview.eval` but routes large payloads through an asynchronous fetch
    /// round-trip, so a large message is routinely overtaken by smaller
    /// messages (or the end-of-channel marker) sent after it. Each message
    /// carries a per-channel `index` starting at 0; out-of-order arrivals are
    /// buffered until the gaps fill, matching the `Channel` class in
    /// `@tauri-apps/api` (`core.ts`).
    #[derive(Debug)]
    struct Sequencer<T> {
        next_index: usize,
        pending: BTreeMap<usize, Message<T>>,
    }

    impl<T> Sequencer<T> {
        fn new() -> Self {
            Self {
                next_index: 0,
                pending: BTreeMap::new(),
            }
        }

        /// Polls `rx` until the message with the next expected `index` is
        /// available, buffering any messages that arrive out of order.
        fn poll_next(
            &mut self,
            rx: &mut (impl Stream<Item = Message<T>> + Unpin),
            cx: &mut Context<'_>,
        ) -> Poll<Option<T>> {
            loop {
                let item = match self.pending.remove(&self.next_index) {
                    Some(item) => item,
                    None => match rx.poll_next_unpin(cx) {
                        Poll::Ready(Some(item)) if item.index == self.next_index => item,
                        Poll::Ready(Some(item)) => {
                            self.pending.insert(item.index, item);
                            continue;
                        }
                        _ => return Poll::Pending,
                    },
                };

                self.next_index += 1;

                return if item.end() {
                    // TODO: Delete channel from `window`.
                    // See `core.ts > class Channel > private cleanupCallback`.
                    Poll::Ready(None)
                } else {
                    Poll::Ready(item.message)
                };
            }
        }
    }

    #[derive(Debug)]
    pub struct Channel<T> {
        id: usize,
        rx: mpsc::UnboundedReceiver<Message<T>>,
        sequencer: Sequencer<T>,
        _id_keep_alive: SendWrapper<Closure<dyn FnMut(JsValue)>>,
    }

    impl<T> Channel<T> {
        pub fn new() -> Self
        where
            T: DeserializeOwned + 'static,
        {
            let (tx, rx) = mpsc::unbounded::<Message<T>>();
            let closure = Closure::<dyn FnMut(JsValue)>::new(move |raw| {
                let _ = tx.unbounded_send(serde_wasm_bindgen::from_value(raw).unwrap());
            });

            let id = inner::transform_callback(&closure, false);

            Channel {
                id,
                rx,
                sequencer: Sequencer::new(),
                _id_keep_alive: SendWrapper::new(closure),
            }
        }

        pub fn id(&self) -> usize {
            self.id
        }
    }

    impl<T> Serialize for Channel<T> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(&format!("__CHANNEL__:{}", self.id))
        }
    }

    impl<T> Stream for Channel<T> {
        type Item = T;

        fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            let this = self.get_mut();
            this.sequencer.poll_next(&mut this.rx, cx)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use futures::FutureExt;

        fn message(index: usize, message: u32) -> Message<u32> {
            Message {
                index,
                end: None,
                message: Some(message),
            }
        }

        fn end(index: usize) -> Message<u32> {
            Message {
                index,
                end: Some(true),
                message: None,
            }
        }

        fn ordered(rx: mpsc::UnboundedReceiver<Message<u32>>) -> impl Stream<Item = u32> + Unpin {
            let mut sequencer = Sequencer::new();
            let mut rx = rx;
            futures::stream::poll_fn(move |cx| sequencer.poll_next(&mut rx, cx))
        }

        /// Polls the stream once without blocking.
        fn poll_once(stream: &mut (impl Stream<Item = u32> + Unpin)) -> Poll<Option<u32>> {
            match stream.next().now_or_never() {
                Some(item) => Poll::Ready(item),
                None => Poll::Pending,
            }
        }

        #[test]
        fn in_order_arrival_is_delivered_unchanged() {
            let (tx, rx) = mpsc::unbounded();
            let mut stream = ordered(rx);

            for (index, item) in [message(0, 10), message(1, 11), message(2, 12), end(3)]
                .into_iter()
                .enumerate()
            {
                assert_eq!(item.index(), index);
                tx.unbounded_send(item).unwrap();
            }

            assert_eq!(poll_once(&mut stream), Poll::Ready(Some(10)));
            assert_eq!(poll_once(&mut stream), Poll::Ready(Some(11)));
            assert_eq!(poll_once(&mut stream), Poll::Ready(Some(12)));
            assert_eq!(poll_once(&mut stream), Poll::Ready(None));
        }

        #[test]
        fn shuffled_arrival_is_delivered_in_index_order() {
            let (tx, rx) = mpsc::unbounded();
            let mut stream = ordered(rx);

            for item in [message(2, 12), message(0, 10), message(1, 11), end(3)] {
                tx.unbounded_send(item).unwrap();
            }

            assert_eq!(poll_once(&mut stream), Poll::Ready(Some(10)));
            assert_eq!(poll_once(&mut stream), Poll::Ready(Some(11)));
            assert_eq!(poll_once(&mut stream), Poll::Ready(Some(12)));
            assert_eq!(poll_once(&mut stream), Poll::Ready(None));
        }

        #[test]
        fn gap_blocks_delivery_until_filled() {
            let (tx, rx) = mpsc::unbounded();
            let mut stream = ordered(rx);

            tx.unbounded_send(message(5, 15)).unwrap();
            assert_eq!(poll_once(&mut stream), Poll::Pending);

            for index in 0..5 {
                tx.unbounded_send(message(index, 10 + index as u32))
                    .unwrap();
            }
            tx.unbounded_send(end(6)).unwrap();

            for item in 10..=15 {
                assert_eq!(poll_once(&mut stream), Poll::Ready(Some(item)));
            }
            assert_eq!(poll_once(&mut stream), Poll::Ready(None));
        }

        #[test]
        fn early_end_does_not_cut_off_earlier_messages() {
            let (tx, rx) = mpsc::unbounded();
            let mut stream = ordered(rx);

            tx.unbounded_send(end(3)).unwrap();
            assert_eq!(poll_once(&mut stream), Poll::Pending);

            for item in [message(2, 12), message(1, 11), message(0, 10)] {
                tx.unbounded_send(item).unwrap();
            }

            assert_eq!(poll_once(&mut stream), Poll::Ready(Some(10)));
            assert_eq!(poll_once(&mut stream), Poll::Ready(Some(11)));
            assert_eq!(poll_once(&mut stream), Poll::Ready(Some(12)));
            assert_eq!(poll_once(&mut stream), Poll::Ready(None));
        }
    }
}

mod inner {
    use wasm_bindgen::{
        JsValue,
        prelude::{Closure, wasm_bindgen},
    };

    #[wasm_bindgen(module = "/src/core.js")]
    extern "C" {
        pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
        #[wasm_bindgen(js_name = "invoke", catch)]
        pub async fn invoke_result(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(js_name = "convertFileSrc")]
        pub fn convert_file_src(filePath: &str, protocol: &str) -> JsValue;
        #[wasm_bindgen(js_name = "transformCallback")]
        pub fn transform_callback(callback: &Closure<dyn FnMut(JsValue)>, once: bool) -> usize;
        #[wasm_bindgen(js_name = "isTauri")]
        pub fn is_tauri() -> bool;
    }
}
