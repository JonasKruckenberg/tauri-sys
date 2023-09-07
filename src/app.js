async function getVersion() {
  return window.__TAURI_INVOKE__("plugin:app|version");
}

async function getName() {
  return window.__TAURI_INVOKE__("plugin:app|name");
}

async function getTauriVersion() {
  return window.__TAURI_INVOKE__("plugin:app|tauri_version");
}

async function show() {
  return window.__TAURI_INVOKE__("plugin:app|show");
}

async function hide() {
  return window.__TAURI_INVOKE__("plugin:app|hide");
}

export { getName, getVersion, getTauriVersion, show, hide };
