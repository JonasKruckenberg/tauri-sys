mod app;
mod clipboard;
mod dialog;
mod event;
mod notification;
mod os;
mod tauri_log;
mod window;
mod global_shortcut;

extern crate console_error_panic_hook;
use log::LevelFilter;
use std::future::Future;
use std::panic;
use sycamore::prelude::*;
use sycamore::suspense::Suspense;
use tauri_log::TauriLogger;

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
pub struct TestProps<F> {
    name: &'static str,
    test: F,
}

#[component]
pub async fn TestInner<G: Html, F>(props: TestProps<F>) -> View<G>
where
    F: Future<Output = anyhow::Result<()>> + 'static,
{
    let res = props.test.await;

    view! {
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

#[component]
pub fn Test<G: Html, F>(props: TestProps<F>) -> View<G>
where
    F: Future<Output = anyhow::Result<()>> + 'static,
{
    let fallback = view! {
        tr {
            td { code { (props.name.to_string()) } }
            td {
                span(class="loader") { "⏳" }
            }
        }
    };

    view! {
        Suspense(fallback=fallback) {
            TestInner(name= props.name, test=props.test)
        }
    }
}

#[cfg(not(feature = "ci"))]
#[component]
pub fn InteractiveTest<G: Html, F>(props: TestProps<F>) -> View<G>
where
    F: Future<Output = anyhow::Result<()>> + 'static,
{
    let mut test = Some(props.test);
    let render_test = create_signal(false);

    let run_test = |_| {
        render_test.set(true);
    };

    view! {
        (if render_test.get() {
            let test = test.take().unwrap();

            view! {
                Test(name=props.name, test=test)
            }
        } else {
            view! {
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
pub async fn InteractiveTest<'a, G: Html, F>(_props: TestProps<'a, F>) -> View<G>
where
    F: Future<Output = anyhow::Result<()>> + 'a,
{
    view! { "Interactive tests are not run in CI mode" }
}

#[component]
pub async fn Terminate<G: Html>() -> View<G> {
    #[cfg(feature = "ci")]
    sycamore::suspense::await_suspense(cx, async {
        tauri_sys::process::exit(0).await;
    })
    .await;

    view! {}
}

static LOGGER: TauriLogger = TauriLogger;

fn main() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Trace))
        .unwrap();

    panic::set_hook(Box::new(|info| {
        console_error_panic_hook::hook(info);

        #[cfg(feature = "ci")]
        wasm_bindgen_futures::spawn_local(exit_with_error(format!("{}", info)));
    }));

    sycamore::render(|| {
        view! {
            table {
                tbody {
                    // Suspense(fallback=view!{ cx, "Running Tests..." }) {
                        Test(name="app::get_name",test=app::get_name())
                        Test(name="app::get_version",test=app::get_version())
                        Test(name="app::get_tauri_version",test=app::get_tauri_version())
                        Test(name="clipboard::read_text | clipboard::write_text",test=clipboard::test())
                        Test(name="event::emit",test=event::emit())
                        Test(name="event::listen",test=event::listen())
                        Test(name="event::once",test=event::once())
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
                        Test(name="notification::is_permission_granted",test=notification::is_permission_granted())
                        Test(name="notification::request_permission",test=notification::request_permission())
                        InteractiveTest(name="notification::show_notification",test=notification::show_notification())
                        InteractiveTest(name="global_shortcut::register_all",test=global_shortcut::register_all())

                        Test(name="window::WebviewWindow::new",test=window::create_window())

                        Terminate
                    // }
                }
            }
        }
    });
}
