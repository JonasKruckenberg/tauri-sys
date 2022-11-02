use sycamore::prelude::*;
use tauri_sys::clipboard::{read_text, write_text};

#[component]
pub fn Updater<G: Html>(cx: Scope) -> View<G> {
    // let text = create_signal(cx, "clipboard message".to_string());

    // let write = move |_| {
    //     sycamore::futures::spawn_local_scoped(cx, async move {
    //         write_text(&text.get()).await
    //         //   .then(() => {
    //         //     onMessage('Wrote to the clipboard')
    //         //   })
    //         //   .catch(onMessage)
    //     });
    // };

    // let read = |_| {
    //     sycamore::futures::spawn_local(async move {
    //         let text = read_text().await;

    //         log::info!("Read text from clipboard {:?}", text);
    //         // readText()
    //         //   .then((contents) => {
    //         //     onMessage(`Clipboard contents: ${contents}`)
    //         //   })
    //         //   .catch(onMessage)
    //     });
    // };

    // view! { cx,
    //     div(class="flex gap-1") {
    //         input(class="grow input",placeholder="Text to write to the clipboard",bind:value=text)
    //         button(class="btn",type="button",on:click=write) {
    //             "Write"
    //         }
    //         button(class="btn",type="button",on:click=read) {
    //             "Read"
    //         }
    //     }
    // }
    view! { cx,
      div(class="updater") { }
    }
}
