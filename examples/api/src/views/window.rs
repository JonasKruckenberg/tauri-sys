use sycamore::prelude::*;
use tauri_sys::window;

#[component]
pub fn Window<G: Html>(cx: Scope) -> View<G> {
    let get_current = |_| {
        let win = window::current_window();

        log::debug!("{:?}", win);
    };

    let get_all = |_| {
        let windows = window::all_windows();

        log::debug!("{:?}", windows);
    };

    let get_current_monitor = |_| {
        sycamore::futures::spawn_local(async move {
            let monitor = window::current_monitor().await;

            log::debug!("{:?}", monitor);
        });
    };

    let get_primary_monitor = |_| {
        sycamore::futures::spawn_local(async move {
            let monitor = window::primary_monitor().await;

            log::debug!("{:?}", monitor);
        });
    };

    let get_all_monitors = |_| {
        sycamore::futures::spawn_local(async move {
            let monitors = window::available_monitors().await.collect::<Vec<_>>();

            log::debug!("{:?}", monitors);
        });
    };

    view! { cx,
        div {
            button(class="btn",id="get_name",on:click=get_current) {
                "Get Current Window"
            }
            button(class="btn",id="get_version",on:click=get_all) {
                "Get All Windows"
            }
        }
        div {
            button(class="btn",id="get_tauri_version",on:click=get_current_monitor) {
                "Get Current Monitor"
            }
            button(class="btn",id="get_tauri_version",on:click=get_primary_monitor) {
                "Get Primary Monitor"
            }
            button(class="btn",id="get_tauri_version",on:click=get_all_monitors) {
                "Get All Monitors"
            }
        }
    }
}
