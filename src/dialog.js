const { invoke } = window.__TAURI__.primitives;

async function open(options) {
  if (typeof options === "object") {
    Object.freeze(options);
  }

  return invoke("plugin:dialog|open", { options });
}

async function open(options) {
  if (typeof options === "object") {
    Object.freeze(options);
  }

  return invoke("plugin:dialog|open", { options });
}

async function save(options) {
  if (typeof options === "object") {
    Object.freeze(options);
  }

  return invoke("plugin:dialog|save", { options });
}

async function message(message, options) {
  const opts = typeof options === "string" ? { title: options } : options;
  return invoke("plugin:dialog|message", {
    message: message.toString(),
    title: opts?.title?.toString(),
    type: opts?.type,
    okButtonLabel: opts?.okLabel?.toString(),
  });
}

async function ask(message, options) {
  const opts = typeof options === "string" ? { title: options } : options;
  return invoke("plugin:dialog|ask", {
    message: message.toString(),
    title: opts?.title?.toString(),
    type: opts?.type,
    okButtonLabel: opts?.okLabel?.toString(),
    cancelButtonLabel: opts?.cancelLabel?.toString(),
  });
}

async function confirm(message, options) {
  const opts = typeof options === "string" ? { title: options } : options;
  return invoke("plugin:dialog|confirm", {
    message: message.toString(),
    title: opts?.title?.toString(),
    type: opts?.type,
    okButtonLabel: opts?.okLabel?.toString(),
    cancelButtonLabel: opts?.cancelLabel?.toString(),
  });
}

export { open, save, message, ask, confirm };
