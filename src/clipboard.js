const { invoke } = window.__TAURI__.primitives;

async function writeText(text, opts) {
  return invoke("plugin:clipboard|write", {
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
  const kind = await invoke("plugin:clipboard|read");
  return kind.options;
}

export { readText, writeText };
