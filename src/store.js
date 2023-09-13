async function _unlisten(event, eventId) {
  await window.__TAURI_INVOKE__("plugin:event|unlisten", {
    event,
    eventId,
  });
}

async function listen(event, handler, options) {
  return window
    .__TAURI_INVOKE__("plugin:event|listen", {
      event,
      windowLabel: options?.target,
      handler: window.__TAURI__.transformCallback(handler),
    })
    .then((eventId) => {
      return async () => _unlisten(event, eventId);
    });
}

export class Store {
  path;

  constructor(path) {
    this.path = path;
  }

  async set(key, value) {
    return await window.__TAURI_INVOKE__("plugin:store|set", {
      path: this.path,
      key,
      value,
    });
  }

  async get(key) {
    return await window.__TAURI_INVOKE__("plugin:store|get", {
      path: this.path,
      key,
    });
  }

  async has(key) {
    return await window.__TAURI_INVOKE__("plugin:store|has", {
      path: this.path,
      key,
    });
  }

  async delete(key) {
    return await window.__TAURI_INVOKE__("plugin:store|delete", {
      path: this.path,
      key,
    });
  }

  async clear() {
    return await window.__TAURI_INVOKE__("plugin:store|clear", {
      path: this.path,
    });
  }

  async reset() {
    return await window.__TAURI_INVOKE__("plugin:store|reset", {
      path: this.path,
    });
  }

  async keys() {
    return await window.__TAURI_INVOKE__("plugin:store|keys", {
      path: this.path,
    });
  }

  async values() {
    return await window.__TAURI_INVOKE__("plugin:store|values", {
      path: this.path,
    });
  }

  async entries() {
    console.log("path", this.path);
    return await window.__TAURI_INVOKE__("plugin:store|entries", {
      path: this.path,
    });
  }

  async length() {
    console.log("path", this.path);
    return await window.__TAURI_INVOKE__("plugin:store|length", {
      path: this.path,
    });
  }

  async load() {
    return await window.__TAURI_INVOKE__("plugin:store|load", {
      path: this.path,
    });
  }

  async save() {
    return await window.__TAURI_INVOKE__("plugin:store|save", {
      path: this.path,
    });
  }

  async onKeyChange(key, cb) {
    return (await listen)("store://change", (event) => {
      if (event.payload.path === this.path && event.payload.key === key) {
        cb(event.payload.value);
      }
    });
  }

  async onChange(cb) {
    return (await listen)("store://change", (event) => {
      if (event.payload.path === this.path) {
        cb(event.payload.key, event.payload.value);
      }
    });
  }
}
