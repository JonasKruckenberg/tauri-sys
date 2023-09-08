function eol() {
  return window.__TAURI__.os.__eol;
}

async function platform() {
  return window.__TAURI_INVOKE__("plugin:os|platform");
}

async function version() {
  return window.__TAURI_INVOKE__("plugin:os|version");
}

async function family() {
  return window.__TAURI_INVOKE__("plugin:os|family");
}

async function type() {
  return window.__TAURI_INVOKE__("plugin:os|os_type");
}

async function arch() {
  return window.__TAURI_INVOKE__("plugin:os|arch");
}

async function locale() {
  return window.__TAURI_INVOKE__("plugin:os|locale");
}

async function exeExtension() {
  return window.__TAURI_INVOKE__("plugin:os|exe_extension");
}

async function hostname() {
  return window.__TAURI_INVOKE__("plugin:os|hostname");
}

export { eol, platform, family, version, type, arch, locale, exeExtension, hostname };
