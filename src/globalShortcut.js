const invoke = window.__TAURI__.primitives.invoke;

class Channel {
  id;
  __TAURI_CHANNEL_MARKER__ = true;
  #onmessage = () => {};

  constructor() {
    this.id = window.__TAURI__.transformCallback((response) => {
      this.#onmessage(response);
    });
  }

  set onmessage(handler) {
    this.#onmessage = handler;
  }

  get onmessage() {
    return this.#onmessage;
  }

  toJSON() {
    return `__CHANNEL__:${this.id}`;
  }
}

async function register(shortcut, handler) {
  const h = new Channel();
  h.onmessage = handler;

  return invoke("plugin:globalShortcut|register", {
    shortcut,
    handler: h,
  });
}

async function registerAll(shortcuts, handler) {
  const h = new Channel();
  h.onmessage = handler;

  return invoke("plugin:globalShortcut|register_all", {
    shortcuts,
    handler: h,
  });
}

async function isRegistered(shortcut) {
  return invoke("plugin:globalShortcut|is_registered", {
    shortcut,
  });
}

async function unregister(shortcut) {
  return invoke("plugin:globalShortcut|unregister", {
    shortcut,
  });
}

async function unregisterAll() {
  return invoke("plugin:globalShortcut|unregister_all");
}

export { register, registerAll, isRegistered, unregister, unregisterAll };
