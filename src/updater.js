const { invoke, transformCallback } = window.__TAURI__.primitives;

class Channel {
  id;
  __TAURI_CHANNEL_MARKER__ = true;
  #onmessage = () => {};

  constructor() {
    this.id = transformCallback((response) => {
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

class Update {
  available;
  currentVersion;
  version;
  date;
  body;

  constructor(metadata) {
    this.available = metadata.available;
    this.currentVersion = metadata.currentVersion;
    this.version = metadata.version;
    this.date = metadata.date;
    this.body = metadata.body;
  }

  async downloadAndInstall(onEvent) {
    const channel = new Channel();

    if (onEvent != null) {
      channel.onmessage = onEvent;
    }

    return invoke("plugin:updater|download_and_install", {
      onEvent: channel,
    });
  }
}

async function check(options) {
  if (options?.headers) {
    options.headers = Array.from(new Headers(options.headers).entries());
  }

  return window.__TAURI_INVOKE__("plugin:updater|check", { ...options }).then((meta) => (meta.available ? new Update(meta) : null));
}

export { Update, check };
