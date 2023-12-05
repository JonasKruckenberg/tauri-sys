const invoke = window.__TAURI__.primitives.invoke;

async function readTextFile(filePath, options = {}) {
  return await invoke("plugin:fs|read_text_file", {
    path: filePath,
    options,
  });
}

async function readBinaryFile(filePath, options = {}) {
  const arr = await invoke("plugin:fs|read_file", {
    path: filePath,
    options,
  });

  return Uint8Array.from(arr);
}

async function writeTextFile(path, contents, options) {
  if (typeof options === "object") {
    Object.freeze(options);
  }
  if (typeof path === "object") {
    Object.freeze(path);
  }

  const file = { path: "", contents: "" };
  let fileOptions = options;
  if (typeof path === "string") {
    file.path = path;
  } else {
    file.path = path.path;
    file.contents = path.contents;
  }

  if (typeof contents === "string") {
    file.contents = contents ?? "";
  } else {
    fileOptions = contents;
  }

  return await invoke("plugin:fs|write_file", {
    path: file.path,
    contents: Array.from(new TextEncoder().encode(file.contents)),
    options: fileOptions,
  });
}

async function writeBinaryFile(path, contents, options) {
  if (typeof options === "object") {
    Object.freeze(options);
  }
  if (typeof path === "object") {
    Object.freeze(path);
  }

  const file = { path: "", contents: [] };
  let fileOptions = options;
  if (typeof path === "string") {
    file.path = path;
  } else {
    file.path = path.path;
    file.contents = path.contents;
  }

  if (contents && "dir" in contents) {
    fileOptions = contents;
  } else if (typeof path === "string") {
    file.contents = contents ?? [];
  }

  return await invoke("plugin:fs|write_file", {
    path: file.path,
    contents: Array.from(file.contents instanceof ArrayBuffer ? new Uint8Array(file.contents) : file.contents),
    options: fileOptions,
  });
}

async function readDir(dir, options = {}) {
  return await invoke("plugin:fs|read_dir", {
    path: dir,
    options,
  });
}

async function createDir(dir, options = {}) {
  return await invoke("plugin:fs|create_dir", {
    path: dir,
    options,
  });
}

async function removeDir(dir, options = {}) {
  return await invoke("plugin:fs|remove_dir", {
    path: dir,
    options,
  });
}

async function copyFile(source, destination, options = {}) {
  return await invoke("plugin:fs|copy_file", {
    source,
    destination,
    options,
  });
}

async function removeFile(file, options = {}) {
  return await invoke("plugin:fs|remove_file", {
    path: file,
    options,
  });
}

async function renameFile(oldPath, newPath, options = {}) {
  return await invoke("plugin:fs|rename_file", {
    oldPath,
    newPath,
    options,
  });
}

async function exists(path, options = {}) {
  return await invoke("plugin:fs|exists", { path, options });
}

async function metadata(path) {
  return await invoke("plugin:fs|metadata", {
    path,
  });
}

export {
  readTextFile,
  readBinaryFile,
  writeTextFile,
  writeTextFile as writeFile,
  writeBinaryFile,
  readDir,
  createDir,
  removeDir,
  copyFile,
  removeFile,
  renameFile,
  exists,
  metadata,
};
