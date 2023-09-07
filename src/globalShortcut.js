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

  return window.__TAURI_INVOKE__("plugin:globalShortcut|register", {
    shortcut,
    handler: h,
  }).then(() => {
    console.log('return unregister');
    return async () => unregister(shortcut);
  });
}

async function registerAll(shortcuts, handler) {
  const h = new Channel();
  h.onmessage = handler;

  return window.__TAURI_INVOKE__("plugin:globalShortcut|register_all", {
    shortcuts,
    handler: h,
  });
}

async function isRegistered(shortcut) {
  return window.__TAURI_INVOKE__("plugin:globalShortcut|is_registered", {
    shortcut,
  });
}

async function unregister(shortcut) {
  return window.__TAURI_INVOKE__("plugin:globalShortcut|unregister", {
    shortcut,
  });
}

async function unregisterAll() {
  return window.__TAURI_INVOKE__("plugin:globalShortcut|unregister_all");
}

export { register, registerAll, isRegistered, unregister, unregisterAll };
