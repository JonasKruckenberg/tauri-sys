use std::process::Command;

fn main() {
    /* Shared arguments */
    let sargs: &[&str] = &[
        "--outdir=dist",
        "--format=esm",
        "--bundle",
        "tauri/tooling/api/src/app.ts",
        "tauri/tooling/api/src/clipboard.ts",
        "tauri/tooling/api/src/tauri.ts",
        "tauri/tooling/api/src/event.ts",
        "tauri/tooling/api/src/mocks.ts",
        "tauri/tooling/api/src/window.ts"
    ];

    if cfg!(windows) {
        /* Use cmd if the target is windows */
        Command::new("cmd")
            .args(&["/C", "esbuild"])
            .args(sargs)
            .output()
            .unwrap();
    } else if cfg!(unix) {
        Command::new("esbuild")
            .args(sargs)
            .output()
            .unwrap();
    } else {
        panic!("Unsupported build target");
    }
}
