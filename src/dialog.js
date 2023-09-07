async function open(options = {}) {
  if (typeof options === "object") {
    Object.freeze(options);
  }
  return window.__TAURI_INVOKE__("plugin:dialog|open", { options });
}

async function save(options = {}) {
  if (typeof options === "object") {
    Object.freeze(options);
  }
  return window.__TAURI_INVOKE__("plugin:dialog|save", { options });
}

async function message(message, options) {
  var _a, _b;
  const opts = typeof options === "string" ? { title: options } : options;
  return window.__TAURI_INVOKE__("plugin:dialog|message", {
    message: message.toString(),
    title:
      (_a = opts === null || opts === void 0 ? void 0 : opts.title) === null ||
      _a === void 0
        ? void 0
        : _a.toString(),
    type: opts === null || opts === void 0 ? void 0 : opts.type,
    okButtonLabel:
      (_b = opts === null || opts === void 0 ? void 0 : opts.okLabel) ===
        null || _b === void 0
        ? void 0
        : _b.toString(),
  });
}

async function ask(message, options) {
  var _a, _b, _c, _d, _e;
  const opts = typeof options === "string" ? { title: options } : options;
  return window.__TAURI_INVOKE__("plugin:dialog|ask", {
    message: message.toString(),
    title:
      (_a = opts === null || opts === void 0 ? void 0 : opts.title) === null ||
      _a === void 0
        ? void 0
        : _a.toString(),
    type: opts === null || opts === void 0 ? void 0 : opts.type,
    okButtonLabel:
      (_c =
        (_b = opts === null || opts === void 0 ? void 0 : opts.okLabel) ===
          null || _b === void 0
          ? void 0
          : _b.toString()) !== null && _c !== void 0
        ? _c
        : "Yes",
    cancelButtonLabel:
      (_e =
        (_d = opts === null || opts === void 0 ? void 0 : opts.cancelLabel) ===
          null || _d === void 0
          ? void 0
          : _d.toString()) !== null && _e !== void 0
        ? _e
        : "No",
  });
}

async function confirm(message, options) {
  var _a, _b, _c, _d, _e;
  const opts = typeof options === "string" ? { title: options } : options;
  return window.__TAURI_INVOKE__("plugin:dialog|confirm", {
    message: message.toString(),
    title:
      (_a = opts === null || opts === void 0 ? void 0 : opts.title) === null ||
      _a === void 0
        ? void 0
        : _a.toString(),
    type: opts === null || opts === void 0 ? void 0 : opts.type,
    okButtonLabel:
      (_c =
        (_b = opts === null || opts === void 0 ? void 0 : opts.okLabel) ===
          null || _b === void 0
          ? void 0
          : _b.toString()) !== null && _c !== void 0
        ? _c
        : "Ok",
    cancelButtonLabel:
      (_e =
        (_d = opts === null || opts === void 0 ? void 0 : opts.cancelLabel) ===
          null || _d === void 0
          ? void 0
          : _d.toString()) !== null && _e !== void 0
        ? _e
        : "Cancel",
  });
}

export { ask, confirm, message, open, save };
