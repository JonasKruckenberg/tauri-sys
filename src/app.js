// tauri/tooling/api/src/app.ts
async function getName() {
	return window.__TAURI__.app.getName()
}
async function getTauriVersion() {
	return window.__TAURI__.app.getTauriVersion()
}
async function getVersion() {
	return window.__TAURI__.app.getVersion()
}
async function setTheme(theme) {
    window.__TAURI__.app.setTheme(theme)
}
async function hide() {
    return window.__TAURI__.app.hide()
}
async function show() {
    return window.__TAURI__.app.show()
}

export {
    getName,
    getTauriVersion,
    getVersion,
    setTheme,
    show,
    hide,
}