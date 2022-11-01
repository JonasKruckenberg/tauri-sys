use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use tauri_sys::event::{emit, listen};
use tauri_sys::tauri::invoke;
use shared::RequestBody;

#[component]
pub fn Communication<'a, G: Html>(cx: Scope<'a>) -> View<G> {
    let unlisten = create_signal::<Option<Box<&dyn FnOnce()>>>(cx, None);

    // on_mount(cx, move || {

    //     sycamore::futures::spawn_local_scoped(cx, async move {
    //         let unlisten_raw = listen::<Reply>("rust-event", &|reply| log::debug!("got reply {:?}", reply)).await;

    //         unlisten.set(Some(Box::new(&unlisten_raw)));
    //     });
    // });

    // on_cleanup(cx, || {
    //     if let Some(unlisten) = unlisten .take().as_deref() {
    //         (unlisten)()
    //     }
    // });

    let log = |_| {
        #[derive(Serialize)]
        struct Payload<'a> {
            event: &'a str,
            payload: &'a str,
        }

        sycamore::futures::spawn_local(async move {
            let res = invoke::<_, ()>(
                "log_operation",
                &Payload {
                    event: "tauri-click",
                    payload: "this payload is optional because we used Option in Rust",
                },
            )
            .await;

            log::debug!("Emitted event, response {:?}", res);
        });
    };

    let perform_request = |_| {
        sycamore::futures::spawn_local(async move {
            #[derive(Serialize)]
            struct Payload<'a> {
                endpoint: &'a str, 
                body: RequestBody<'a>
            }

            let res = invoke::<_, String>(
                "perform_request",
                &Payload {
                    endpoint: "dummy endpoint arg",
                    body: RequestBody {
                        id: 5,
                        name: "test",
                    },
                },
            )
            .await;

            log::debug!("Got reply {:?}", res);
        });
    };

    let emit_event = |_| {
        sycamore::futures::spawn_local(async move {
            emit("js-event", &"this is the payload string").await;
        });
    };

    view! { cx,
        div {
            button(class="btn",id="log",on:click=log) {
                "Call Log API"
            }
            button(class="btn",mid="request",on:click=perform_request) {
                "Call Request (async) API"
            }
            button(class="btn",id="event",on:click=emit_event) {
                "Send event to Rust"
            }
        }
    }
}
