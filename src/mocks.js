// tauri/tooling/api/src/mocks.ts
function mockIPC(cb) {
  window.__TAURI_IPC__ = async ({
    cmd,
    callback,
    error,
    ...args
  }) => {
    try {
      window[`_${callback}`](await cb(cmd, args));
    } catch (err) {
      window[`_${error}`](err);
    }
  };
}
function mockWindows(current, ...additionalWindows) {
  window.__TAURI_METADATA__ = {
    __windows: [current, ...additionalWindows].map((label) => ({ label })),
    __currentWindow: { label: current }
  };
}
function mockConvertFileSrc(osName, windowsProtocolScheme = "https") {
  window.__TAURI__ = window.__TAURI__ ?? {};
  window.__TAURI__.convertFileSrc = function(filePath, protocol = "asset") {
    const path = encodeURIComponent(filePath);
    return osName === "windows" ? `${windowsProtocolScheme}://${protocol}.localhost/${path}` : `${protocol}://localhost/${path}`;
  };
}
function clearMocks() {
  if (window.__TAURI__?.convertFileSrc)
    delete window.__TAURI__.convertFileSrc;
  if (window.__TAURI_IPC__)
    delete window.__TAURI_IPC__;
  if (window.__TAURI_METADATA__)
    delete window.__TAURI_METADATA__;
}
export {
  clearMocks,
  mockConvertFileSrc,
  mockIPC,
  mockWindows
};
