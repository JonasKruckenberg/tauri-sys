async function writeText(text, opts) {
  return window.__TAURI_INVOKE__("plugin:clipboard|write", {
    data: {
      kind: "PlainText",
      options: {
        label: opts?.label,
        text,
      },
    },
  });
}

async function readText() {
  const kind = await window.__TAURI_INVOKE__("plugin:clipboard|read");
  return kind.options;
}

export { readText, writeText };
