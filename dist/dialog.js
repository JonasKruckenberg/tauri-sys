// tauri/tooling/api/src/tauri.ts
function uid() {
  return window.crypto.getRandomValues(new Uint32Array(1))[0];
}
function transformCallback(callback, once3 = false) {
  const identifier = uid();
  const prop = `_${identifier}`;
  Object.defineProperty(window, prop, {
    value: (result) => {
      if (once3) {
        Reflect.deleteProperty(window, prop);
      }
      return callback?.(result);
    },
    writable: false,
    configurable: true
  });
  return identifier;
}
async function invoke(cmd, args = {}) {
  return new Promise((resolve, reject) => {
    const callback = transformCallback((e) => {
      resolve(e);
      Reflect.deleteProperty(window, `_${error}`);
    }, true);
    const error = transformCallback((e) => {
      reject(e);
      Reflect.deleteProperty(window, `_${callback}`);
    }, true);
    window.__TAURI_IPC__({
      cmd,
      callback,
      error,
      ...args
    });
  });
}

// tauri/tooling/api/src/helpers/tauri.ts
async function invokeTauriCommand(command) {
  return invoke("tauri", command);
}

// tauri/tooling/api/src/helpers/dialog.ts
async function ask(message, options) {
  return await invokeTauriCommand({
    __tauriModule: "Dialog",
    message: {
      cmd: "askDialog",
      message: message.toString(),
      title: options?.title?.toString(),
      type: options?.type
    }
  });
}
async function confirm(message, options) {
  return await invokeTauriCommand({
    __tauriModule: "Dialog",
    message: {
      cmd: "confirmDialog",
      message: message.toString(),
      title: options?.title?.toString(),
      type: options?.type
    }
  });
}
async function open(options) {
  if(!options) {
      options = {multiple: false};
  }
  return await invokeTauriCommand({
    __tauriModule: "Dialog",
    message: {
      cmd: "openDialog",
      options
    }
  });
}
async function open_multiple(options) {
  if(!options) {
    options = {multiple: true};
  }
  return await invokeTauriCommand({
    __tauriModule: "Dialog",
    message: {
      cmd: "openDialog",
      options
    }
  });
}
async function message(message, options) {
  await invokeTauriCommand({
    __tauriModule: "Dialog",
    message: {
      cmd: "messageDialog",
      message: message.toString(),
      title: options?.title?.toString(),
      type: options?.type
    }
  });
}
async function save(options) {
  return await invokeTauriCommand({
    __tauriModule: "Dialog",
    message: {
      cmd: "saveDialog",
      options
    }
  });
}
export {
    ask,
    confirm,
    open,
    open_multiple,
    message,
    save
}
