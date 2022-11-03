mod views;

use sycamore::prelude::*;
#[cfg(not(feature = "ssg"))]
use sycamore_router::{Router, HistoryIntegration};

#[component]
fn Header<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        header(style="display: flex; gap: 1em; margin-bottom: 1em;") {
            a(href="/") {
                "Welcome"
            }
            a(href="/app") {
                "App"
            }
            a(href="/clipboard") {
                "Clipboard"
            }
            a(href="/communication") {
                "Communication"
            }
        }
    }
}

#[cfg(all(not(debug_assertions), not(feature = "ssg")))]
fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    sycamore::hydrate(|cx| view! { cx,
        Header
        Router(
            integration=HistoryIntegration::new(),
            view=views::switch
        )
    });
}

#[cfg(all(debug_assertions, not(feature = "ssg")))]
fn main() {
    use sycamore::view;

    wasm_logger::init(wasm_logger::Config::default());

    sycamore::render(|cx| view! { cx,
        Header
        Router(
            integration=HistoryIntegration::new(),
            view=views::switch
        )
    });
}

#[cfg(feature = "ssg")]
fn main() {
    use sycamore_router::StaticRouter;

    let out_dir = std::env::args().nth(1).unwrap();

    println!("out_dir {}", out_dir);

    let template = std::fs::read_to_string(format!("{}/index.html", out_dir)).unwrap();

    let html = sycamore::render_to_string(|cx| view! { cx,
        Header
        StaticRouter(
            route=route.clone(),
            view=views::switch
        )
    });

    let html = template.replace("<!--app-html-->\n", &html);

    let path = format!("{}/index.html", out_dir);

    println!("Writing html to file \"{}\"", path);
    std::fs::write(path, html).unwrap();
}
