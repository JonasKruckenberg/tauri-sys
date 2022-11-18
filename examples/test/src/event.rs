use std::time::Duration;

use anyhow::ensure;
use futures::StreamExt;
use tauri_sys::{event, tauri};

pub async fn emit() -> anyhow::Result<()> {
    event::emit("javascript-event", &"bar").await?;

    ensure!(tauri::invoke::<_, bool>("verify_receive", &()).await?);

    Ok(())
}

pub async fn listen() -> anyhow::Result<()> {
    let events = event::listen::<u32>("rust-event-listen").await?;
    tauri::invoke::<_, ()>("emit_event_5_times", &()).await?;

    let events: Vec<u32> = events
        .take(5)
        .map(|e| e.payload)
        .collect()
        .await;

    ensure!(events == vec![0, 1, 2, 3, 4]);

    Ok(())
}

pub async fn once() -> anyhow::Result<()> {
    // this causes enough delay for `once` to register it's event listener before the event gets triggered
    wasm_bindgen_futures::spawn_local(async {
        tauri::invoke::<_, ()>("emit_event", &()).await.unwrap();
    });
    let event = event::once::<String>("rust-event-once").await?;

    ensure!(event.payload == "Hello World from Rust!");

    Ok(())
}