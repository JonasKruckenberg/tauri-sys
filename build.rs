use std::process::Command;

fn main() {
    /* Shared arguments */
    let sargs: [&str; 8] = [
        "--outdir=dist",
        "--format=esm",
        "--bundle",
        "tauri/tooling/api/src/app.ts",
        "tauri/tooling/api/src/clipboard.ts",
        "tauri/tooling/api/src/tauri.ts",
        "tauri/tooling/api/src/event.ts",
        "tauri/tooling/api/src/mocks.ts",
    ];

    if cfg!(target_os = "windows") {
        /* Use cmd if the target is windows */
        Command::new("cmd")
            .args(&["/C", "esbuild"])
            .args(&sargs)
            .output()
            .unwrap();
    } else {
        Command::new("esbuild")
            .args(&sargs)
            .output()
            .unwrap();
    };
}
