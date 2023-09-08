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
tauri-sys = { git = "https://github.com/LetrixZ/tauri-sys" }
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

## Plugins

Some plugins are now separated from the Tauri core. https://github.com/tauri-apps/plugins-workspace

`app`, `event`, `mocks`, `path` and `tauri` are still part of the core.

## Features

Bindings are behind features. Use `all` to enable all features or use the name of the plugin as the feature to enable it.

## Are we Tauri v2 yet?

These API bindings are not completely on-par with `@tauri-apps/api` and the rest of the plugins yet, but here is the current status-quo:

- [x] `app`
- [ ] `cli`
- [x] `clipboard`
- [x] `dialog`
- [x] `event`
- [ ] `fs`
- [x] `global_shortcut`
- [ ] `http`
- [ ] `mocks`
- [x] `notification`
- [x] `os`
- [ ] `path`
- [x] `positioner`
- [x] `process`
- [ ] `shell`
- [x] `tauri`
- [ ] `updater`
- [x] `window`

The current API also very closely mirrors the JS API even though that might not be the most ergonomic choice, ideas for improving the API with quality-of-life features beyond the regular JS API interface are very welcome.

[wasm-bindgen]: https://github.com/rustwasm/wasm-bindgen
[tauri allowlist]: https://tauri.app/v1/api/config#allowlistconfig
[`esbuild`]: https://esbuild.github.io/getting-started/#install-esbuild
