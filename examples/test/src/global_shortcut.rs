use std::time::Duration;

use futures::StreamExt;
use tauri_sys::global_shortcut;

pub async fn register_all() -> anyhow::Result<()> {
    let task = async {
        let shortcuts = ["CommandOrControl+Shift+C", "Ctrl+Alt+F12"];

        let streams = futures::future::try_join_all(shortcuts.map(|s| async move {
            let stream = global_shortcut::register(s).await?;

            anyhow::Ok(stream.map(move |_| s))
        }))
        .await?;

        let mut events = futures::stream::select_all(streams);

        while let Some(shortcut) = events.next().await {
            log::debug!("Shortcut {} triggered", shortcut)
        }

        anyhow::Ok(())
    };

    let timeout = gloo_timers::future::sleep(Duration::from_secs(20));

    futures::future::select(Box::pin(task), timeout).await;

    Ok(())
}
