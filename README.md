<div align="center">
  <h1>
    <code>tauri-sys</code>
  </h1>
  <p>
    <strong>Raw bindings to the <a href="https://tauri.app/v1/api/js/"><code>Tauri API</code></a>
      for projects using <a href="https://github.com/rustwasm/wasm-bindgen"><code>wasm-bindgen</code></a></strong>
  </p>
</div>

[![Documentation master][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-url]

[docs-badge]: https://img.shields.io/badge/docs-main-blue
[docs-url]: https://jonaskruckenberg.github.io/tauri-sys/tauri_sys
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE

## Installation

This crate is not yet published to crates.io, so you need to use it from git. You also need a global installation of [`esbuild`].

```toml
tauri-sys = { git = "https://github.com/JonasKruckenberg/tauri-sys" } // tauri v1 api, main repo
// OR
tauri-sys = { git = "https://github.com/JonasKruckenberg/tauri-sys", branch = "v2" } // tauri v2 api
```

## Usage

```rust
use serde::{Deserialize, Serialize};
use tauri_sys::tauri;

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

fn main() {
    wasm_bindgen_futures::spawn_local(async move {
        let new_msg: String = tauri::invoke("greet", &GreetArgs { name: &name.get() }).await.unwrap();

        println!("{}", new_msg);
    });
}
```

## Features

All modules are gated by accordingly named Cargo features. It is recommended you keep this synced with the features enabled in your [Tauri Allowlist] but no automated tool for this exists (yet).

- **all**: Enables all modules.
- **core**: Enables the `core` module. (Only `invoke` and `convertFileSrc` currently implemented.)
- **event**: Enables the `event` module.
- **window**: Enables the `windows` module. (~20% implemented)

## Are we Tauri yet?

These API bindings are not completely on-par with `@tauri-apps/api` yet, but here is the current status-quo:

- [ ] `app`
- [x] `core` (partial implementation)
- [x] `dpi`
- [x] `event`
- [ ] `image`
- [ ] `menu`
- [ ] `mocks`
- [ ] `path`
- [ ] `tray`
- [ ] `webview`
- [ ] `webviewWindow`
- [x] `window` (partial implementation)

The current API also very closely mirrors the JS API even though that might not be the most ergonomic choice, ideas for improving the API with quality-of-life features beyond the regular JS API interface are very welcome.

[wasm-bindgen]: https://github.com/rustwasm/wasm-bindgen
[tauri allowlist]: https://tauri.app/v1/api/config#allowlistconfig
[`esbuild`]: https://esbuild.github.io/getting-started/#install-esbuild
