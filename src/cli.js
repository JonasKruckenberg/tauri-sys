async function getMatches() {
  return await window.__TAURI_INVOKE__("plugin:cli|cli_matches");
}

export { getMatches };
