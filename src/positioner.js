async function moveWindow(to) {
  await window.__TAURI_INVOKE__("plugin:positioner|move_window", {
    position: parseInt(to),
  });
}

export { moveWindow };
