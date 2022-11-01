#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use shared::{Reply, RequestBody};

#[tauri::command]
fn log_operation(event: String, payload: Option<String>) {
    println!("{} {:?}", event, payload);
}

#[tauri::command]
fn perform_request(endpoint: String, body: RequestBody) -> String {
    println!("{} {:?}", endpoint, body);
    "message response".into()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![log_operation, perform_request])
        .on_page_load(|window, _| {
            let window_ = window.clone();
            window.listen("js-event", move |event| {
                println!("got js-event with message '{:?}'", event.payload());
                let reply = Reply {
                    data: "something else",
                };

                window_
                    .emit("rust-event", Some(reply))
                    .expect("failed to emit");
            });
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
