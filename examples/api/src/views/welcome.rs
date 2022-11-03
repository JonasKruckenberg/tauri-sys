use sycamore::prelude::*;

#[component]
pub fn Welcome<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        h1 {
            "Welcome"
        }
    }
}