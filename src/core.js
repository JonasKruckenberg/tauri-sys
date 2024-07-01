// tauri/tooling/api/src/core.ts
async function invoke(cmd, args = {}) {
	// NB: `options` ignored as not used here.
	return window.__TAURI_INTERNALS__.invoke(cmd, args)
}
function convertFileSrc(filePath, protocol = 'asset') {
	return window.__TAURI_INTERNALS__.convertFileSrc(filePath, protocol)
}
export {
	invoke,
	convertFileSrc
}
