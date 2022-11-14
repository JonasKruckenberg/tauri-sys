mod app;
mod clipboard;
mod event;
mod window;

extern crate console_error_panic_hook;
use std::future::Future;
use std::panic;
use sycamore::prelude::*;
use sycamore::suspense::Suspense;

#[cfg(feature = "ci")]
async fn exit_with_error(e: String) {
    use serde::Serialize;

    #[derive(Serialize)]
    struct Args {
        e: String,
    }

    tauri_sys::tauri::invoke::<_, ()>("exit_with_error", &Args { e })
        .await
        .unwrap();
}

#[derive(Props)]
pub struct TestProps<'a, F>
where
    F: Future<Output = anyhow::Result<()>> + 'a,
{
    name: &'a str,
    test: F,
}

#[component]
pub async fn Test<'a, G: Html, F>(cx: Scope<'a>, props: TestProps<'a, F>) -> View<G>
where
    F: Future<Output = anyhow::Result<()>> + 'a,
{
    let res = props.test.await;

    view! { cx,
        tr {
            td { code { (props.name.to_string()) } }
            td { (if let Err(e) = &res {
                    #[cfg(feature = "ci")]
                    {
                        wasm_bindgen_futures::spawn_local(exit_with_error(e.to_string()));
                        unreachable!()
                    }
                    #[cfg(not(feature = "ci"))]
                    format!("❌ {:?}", e)
                } else {
                    format!("✅")
                })
            }
        }
    }
}

fn main() {
    panic::set_hook(Box::new(|info| {
        console_error_panic_hook::hook(info);

        #[cfg(feature = "ci")]
        wasm_bindgen_futures::spawn_local(exit_with_error(format!("{}", info)));
    }));

    sycamore::render(|cx| {
        view! { cx,
            table {
                tbody {
                    Suspense(fallback=view!{ cx, "Running Tests..." }) {
                        Test(name="app::get_name",test=app::get_name())
                        Test(name="app::get_version",test=app::get_version())
                        Test(name="app::get_tauri_version",test=app::get_tauri_version())
                        Test(name="clipboard::read_text | clipboard::write_text",test=clipboard::test())
                        Test(name="event::emit",test=event::emit())
                        // Test(name="window::WebviewWindow::new",test=window::create_window())
                    }
                }
            }
        }
    });

    #[cfg(feature = "ci")]
    wasm_bindgen_futures::spawn_local(async { tauri_sys::process::exit(0).await; });
}
