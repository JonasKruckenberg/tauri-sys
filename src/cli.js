const { invoke } = window.__TAURI__.primitives;

async function getMatches() {
  return await invoke("plugin:cli|cli_matches");
}

export { getMatches };
