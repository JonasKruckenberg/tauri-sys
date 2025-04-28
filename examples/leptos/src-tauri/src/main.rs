// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Emitter;

fn main() {
    logging::enable();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![trigger_listen_events,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn trigger_listen_events(app: tauri::AppHandle) {
    tracing::debug!("trigger_listen_event");
    std::thread::spawn({
        move || {
            for i in 1..=20 {
                app.emit("event::listen", i).unwrap();
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }
    });
}

mod logging {
    use tracing_subscriber::{Layer, Registry, filter::LevelFilter, fmt, prelude::*};

    const MAX_LOG_LEVEL: LevelFilter = LevelFilter::DEBUG;

    pub fn enable() {
        let console_logger = fmt::layer()
            .with_writer(std::io::stdout)
            .pretty()
            .with_filter(MAX_LOG_LEVEL);

        let subscriber = Registry::default().with(console_logger);
        tracing::subscriber::set_global_default(subscriber).unwrap();
    }
}
