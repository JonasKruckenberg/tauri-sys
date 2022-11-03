mod app;
mod clipboard;
mod communication;
mod welcome;
mod window;

use sycamore::view::View;
use sycamore_router::Route;
use sycamore::prelude::*;

#[derive(Debug, Clone, Route)]
pub enum Page {
    #[to("/app")]
    App,
    #[to("/clipboard")]
    Clipboard,
    #[to("/communication")]
    Communication,
    #[to("/window")]
    Window,
    #[not_found]
    NotFound
}

pub fn switch<G: Html>(cx: Scope, route: &ReadSignal<Page>) -> View<G> {
    match route.get().as_ref() {
        Page::App => app::App(cx),
        Page::Clipboard => clipboard::Clipboard(cx),
        Page::Communication => communication::Communication(cx),
        Page::Window => window::Window(cx),
        Page::NotFound => welcome::Welcome(cx)
    }
}