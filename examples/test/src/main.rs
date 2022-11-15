mod app;
mod clipboard;
mod event;
mod window;
mod dialog;
mod os;

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

#[cfg(not(feature = "ci"))]
#[component]
pub async fn InteractiveTest<'a, G: Html, F>(cx: Scope<'a>, props: TestProps<'a, F>) -> View<G>
where
    F: Future<Output = anyhow::Result<()>> + 'a,
{
    let mut test = Some(props.test);
    let render_test = create_signal(cx, false);

    let run_test = |_| {
        render_test.set(true);
    };

    view! { cx,
        (if *render_test.get() {
            let test = test.take().unwrap();

            let fallback = view! { cx,
                tr {
                    td { code { (props.name.to_string()) } }
                    td {
                        "Running Test..."
                    }
                }
            };

            view! { cx,
                Suspense(fallback=fallback) {
                    Test(name=props.name, test=test)
                }
            }
        } else {
            view! { cx,
                tr {
                    td { code { (props.name.to_string()) } }
                    td {
                        button(on:click=run_test) { "Run Interactive Test"}
                    }
                }
            }
        })
    }
}

#[cfg(feature = "ci")]
#[component]
pub async fn InteractiveTest<'a, G: Html, F>(cx: Scope<'a>, _props: TestProps<'a, F>) -> View<G>
where
    F: Future<Output = anyhow::Result<()>> + 'a,
{
    view! { cx, "Interactive tests are not run in CI mode" }
}

#[component]
pub async fn Terminate<'a, G: Html>(cx: Scope<'a>) -> View<G> {
    #[cfg(feature = "ci")]
    sycamore::suspense::await_suspense(cx, async {
        tauri_sys::process::exit(0).await;
    })
    .await;

    view! {
        cx,
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

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
                        InteractiveTest(name="dialog::message",test=dialog::message())
                        InteractiveTest(name="dialog::ask",test=dialog::ask())
                        InteractiveTest(name="dialog::confirm",test=dialog::confirm())
                        InteractiveTest(name="dialog::pick_file",test=dialog::pick_file())
                        InteractiveTest(name="dialog::pick_files",test=dialog::pick_files())
                        InteractiveTest(name="dialog::pick_folder",test=dialog::pick_folder())
                        InteractiveTest(name="dialog::pick_folders",test=dialog::pick_folders())
                        InteractiveTest(name="dialog::save",test=dialog::save())
                        Test(name="os::arch",test=os::arch())
                        Test(name="os::platform",test=os::platform())
                        Test(name="os::tempdir",test=os::tempdir())
                        Test(name="os::kind",test=os::kind())
                        Test(name="os::version",test=os::version())

                        // Test(name="window::WebviewWindow::new",test=window::create_window())

                        Terminate
                    }
                }
            }
        }
    });
}
