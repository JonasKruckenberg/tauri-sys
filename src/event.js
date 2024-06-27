// tauri/tooling/api/src/core.ts
function transformCallback(callback, once = false) {
	return window.__TAURI_INTERNALS__.transformCallback(callback, once)
}
async function invoke(cmd, args = {}) {
	// NB: `options` ignored as not used here.
	return window.__TAURI_INTERNALS__.invoke(cmd, args)
}

// tauri/tooling/api/src/event.ts
async function _unlisten(event, eventId) {
	await invoke('plugin:event|unlisten', {
		event,
		eventId
	})
}
async function emit(event, payload) {
	await invoke('plugin:event|emit', {
		event,
		payload
	})
}
async function emitTo(
	target,
	event,
	payload
) {
	await invoke('plugin:event|emit_to', {
		target,
		event,
		payload
	})
}
async function listen(event, handler, options) {
	const target =
		typeof options?.target === 'string'
			? { kind: 'AnyLabel', label: options.target }
			: options?.target ?? { kind: 'Any' }
	return invoke('plugin:event|listen', {
		event,
		target,
		handler: transformCallback(handler)
	}).then((eventId) => {
		return async () => _unlisten(event, eventId)
	})
}
async function once(event, handler, options) {
	return listen(
		event,
		(eventData) => {
			handler(eventData)
			_unlisten(event, eventData.id).catch(() => { })
		},
		options
	)
}

// tauri/tooling/api/src/event.ts
var TauriEvent = /* @__PURE__ */ ((TauriEvent2) => {
	TauriEvent2["WINDOW_RESIZED"] = 'tauri://resize';
	TauriEvent2["WINDOW_MOVED"] = 'tauri://move';
	TauriEvent2["WINDOW_CLOSE_REQUESTED"] = 'tauri://close-requested';
	TauriEvent2["WINDOW_DESTROYED"] = 'tauri://destroyed';
	TauriEvent2["WINDOW_FOCUS"] = 'tauri://focus';
	TauriEvent2["WINDOW_BLUR"] = 'tauri://blur';
	TauriEvent2["WINDOW_SCALE_FACTOR_CHANGED"] = 'tauri://scale-change';
	TauriEvent2["WINDOW_THEME_CHANGED"] = 'tauri://theme-changed';
	TauriEvent2["WINDOW_CREATED"] = 'tauri://window-created';
	TauriEvent2["WEBVIEW_CREATED"] = 'tauri://webview-created';
	TauriEvent2["DRAG"] = 'tauri://drag';
	TauriEvent2["DROP"] = 'tauri://drop';
	TauriEvent2["DROP_OVER"] = 'tauri://drop-over';
	TauriEvent2["DROP_CANCELLED"] = 'tauri://drag-cancelled';
	return TauriEvent2;
})(TauriEvent || {});
export {
	TauriEvent,
	emit,
	emitTo,
	listen,
	once
};
