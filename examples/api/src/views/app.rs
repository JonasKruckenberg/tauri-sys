use gloo_timers::callback::Timeout;
use sycamore::prelude::*;
use tauri_api::app;

#[component]
pub fn App<G: Html>(cx: Scope) -> View<G> {
    let show_app = |_| {
        sycamore::futures::spawn_local(async move {
            let res = app::hide().await;

            log::debug!("app hide res {:?}", res);

            let timeout = Timeout::new(2_000, move || {
                sycamore::futures::spawn_local(async move {
                    let res = app::show().await;

                    log::debug!("app show res {:?}", res);
                });
            });

            timeout.forget();
        });
    };

    let hide_app = |_| {
        sycamore::futures::spawn_local(async move {
            let res = app::hide().await;

            log::debug!("app hide res {:?}", res);
        });
    };

    let get_name = |_| {
        sycamore::futures::spawn_local(async move {
            let res = app::get_name().await;

            log::debug!("app name {:?}", res);
        });
    };

    let get_version = |_| {
        sycamore::futures::spawn_local(async move {
            let res = app::get_version().await;

            log::debug!("app version {:?}", res);
        });
    };

    let get_tauri_version = |_| {
        sycamore::futures::spawn_local(async move {
            let res = app::get_tauri_version().await;

            log::debug!("tauri version {:?}", res);
        });
    };

    view! { cx,
      div {
        button(class="btn",id="get_name",on:click=get_name) {
          "Get App Name"
        }
        button(class="btn",id="get_version",on:click=get_version) {
          "Get App Version"
        }
        button(class="btn",id="get_tauri_version",on:click=get_tauri_version) {
          "Get Tauri Version"
        }
      }
      div {
        button(class="btn",id="show",title="Hides and shows the app after 2 seconds",on:click=show_app) {
          "Show"
        }
        button(class="btn",id="hide",on:click=hide_app) {
          "Hide"
        }
      }
    }
}
