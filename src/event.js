async function _unlisten(event, eventId) {
  await window.__TAURI_INVOKE__("plugin:event|unlisten", {
    event,
    eventId,
  });
}

async function listen(event, handler, options) {
  return window.__TAURI_INVOKE__("plugin:event|listen", {
      event,
      windowLabel: options?.target,
      handler: window.__TAURI__.transformCallback(handler),
    })
    .then((eventId) => {
      return async () => _unlisten(event, eventId);
    });
}

async function once(event, handler, options) {
  return listen(
    event,
    (eventData) => {
      handler(eventData);
      _unlisten(event, eventData.id).catch(() => {});
    },
    options
  );
}

async function emit(event, payload, options) {
  await window.__TAURI_INVOKE__("plugin:event|emit", {
    event,
    windowLabel: options?.target,
    payload,
  });
}

export { listen, once, emit };
