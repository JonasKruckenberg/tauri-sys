async function isPermissionGranted() {
  if (window.Notification.permission !== "default") {
    return Promise.resolve(window.Notification.permission === "granted");
  }
  return window.___TAURI_INVOKE__("plugin:notification|is_permission_granted");
}

async function requestPermission() {
  return window.Notification.requestPermission();
}

function sendNotification(options) {
  if (typeof options === "string") {
    new window.Notification(options);
  } else {
    new window.Notification(options.title, options);
  }
}

export { sendNotification, requestPermission, isPermissionGranted };
