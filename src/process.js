async function exit(code = 0) {
  return window.__TAURI_INVOKE__("plugin:process|exit", { code });
}

async function relaunch() {
  return window.__TAURI_INVOKE__("plugin:process|restart");
}

export { exit, relaunch };
