const invoke = window.__TAURI__.primitives.invoke;

function eol() {
  return window.__TAURI__.os.__eol;
}

async function platform() {
  return invoke("plugin:os|platform");
}

async function version() {
  return invoke("plugin:os|version");
}

async function family() {
  return invoke("plugin:os|family");
}

async function type() {
  return invoke("plugin:os|os_type");
}

async function arch() {
  return invoke("plugin:os|arch");
}

async function locale() {
  return invoke("plugin:os|locale");
}

async function exeExtension() {
  return invoke("plugin:os|exe_extension");
}

async function hostname() {
  return invoke("plugin:os|hostname");
}

export { eol, platform, family, version, type, arch, locale, exeExtension, hostname };
