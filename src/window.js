// tauri/tooling/api/src/core.ts
async function invoke(cmd, args = {}) {
	// NB: `options` ignored as not used here.
	return window.__TAURI_INTERNALS__.invoke(cmd, args)
}

// tauri/tooling/api/src/window.ts
function getCurrent() {
	return window.__TAURI_INTERNALS__.metadata.currentWindow
}
function getAll() {
	return window.__TAURI_INTERNALS__.metadata.windows
}
async function currentMonitor() {
	return invoke('plugin:window|current_monitor')
}
async function primaryMonitor() {
	return invoke('plugin:window|primary_monitor')
}
async function monitorFromPoint(x, y) {
	return invoke('plugin:window|monitor_from_point', { x, y })
}
async function availableMonitors() {
	return invoke('plugin:window|available_monitors')
}
async function cursorPosition() {
	return invoke('plugin:window|cursor_position')
}

export {
	getCurrent,
	getAll,
	currentMonitor,
	primaryMonitor,
	monitorFromPoint,
	availableMonitors,
	cursorPosition,
}
