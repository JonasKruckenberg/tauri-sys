use sycamore::prelude::*;
use tauri_sys::updater::check_update;

#[component]
pub fn Updater<G: Html>(cx: Scope) -> View<G> {
    let text = create_signal(cx, "...".to_string());

    let check = move |_| {
        sycamore::futures::spawn_local_scoped(cx, async move {
            log::info!("Test");

            let text = format!("{:?}", check_update().await);

            log::info!("Update info {:?}", text);
            //   .then(() => {
            //     onMessage('Wrote to the clipboard')
            //   })
            //   .catch(onMessage)
        });
    };

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

    view! { cx,
        div(class="flex gap-1") {
            p(class="grow input",bind:value=text)
            button(class="btn",type="button",on:click=check) {
                "Check"
            }
        }
    }
}
