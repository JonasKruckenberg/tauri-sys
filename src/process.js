const invoke = window.__TAURI__.primitives.invoke;

async function exit(code = 0) {
  return invoke("plugin:process|exit", { code });
}

async function relaunch() {
  return invoke("plugin:process|restart");
}

export { exit, relaunch };
