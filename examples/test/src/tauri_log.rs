use log::{Metadata, Record};
use serde::Serialize;
use tauri_sys::tauri;

#[derive(Debug, Serialize)]
struct LogArgs {
    level: Level,
    message: String,
    location: String,
    file: Option<String>,
    line: Option<u32>,
}

#[derive(Debug)]
enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl From<log::Level> for Level {
    fn from(l: log::Level) -> Self {
        match l {
            log::Level::Error => Level::Error,
            log::Level::Warn => Level::Warn,
            log::Level::Info => Level::Info,
            log::Level::Debug => Level::Debug,
            log::Level::Trace => Level::Trace,
        }
    }
}

impl Serialize for Level {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(match self {
            Level::Trace => 1,
            Level::Debug => 2,
            Level::Info => 3,
            Level::Warn => 4,
            Level::Error => 5,
        })
    }
}

pub struct TauriLogger;
impl log::Log for TauriLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let args = LogArgs {
                level: record.level().into(),
                location: record.target().to_string(),
                message: format!("{}", record.args()),
                file: record.file().map(ToString::to_string),
                line: record.line(),
            };

            wasm_bindgen_futures::spawn_local(async move {
                tauri::invoke::<_, ()>("plugin:log|log", &args)
                    .await
                    .unwrap();
            });
        }
    }

    fn flush(&self) {}
}
