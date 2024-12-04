async function invoke(cmd, args = {}) {
	// NB: `options` ignored as not used here.
	return window.__TAURI_INTERNALS__.invoke(cmd, args)
}

// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
/**
 * The path module provides utilities for working with file and directory paths.
 *
 * This package is also accessible with `window.__TAURI__.path` when [`app.withGlobalTauri`](https://v2.tauri.app/reference/config/#withglobaltauri) in `tauri.conf.json` is set to `true`.
 *
 * It is recommended to allowlist only the APIs you use for optimal bundle size and security.
 * @module
 */
/**
 * @since 2.0.0
 */
var BaseDirectory;
(function (BaseDirectory) {
    BaseDirectory[BaseDirectory["Audio"] = 1] = "Audio";
    BaseDirectory[BaseDirectory["Cache"] = 2] = "Cache";
    BaseDirectory[BaseDirectory["Config"] = 3] = "Config";
    BaseDirectory[BaseDirectory["Data"] = 4] = "Data";
    BaseDirectory[BaseDirectory["LocalData"] = 5] = "LocalData";
    BaseDirectory[BaseDirectory["Document"] = 6] = "Document";
    BaseDirectory[BaseDirectory["Download"] = 7] = "Download";
    BaseDirectory[BaseDirectory["Picture"] = 8] = "Picture";
    BaseDirectory[BaseDirectory["Public"] = 9] = "Public";
    BaseDirectory[BaseDirectory["Video"] = 10] = "Video";
    BaseDirectory[BaseDirectory["Resource"] = 11] = "Resource";
    BaseDirectory[BaseDirectory["Temp"] = 12] = "Temp";
    BaseDirectory[BaseDirectory["AppConfig"] = 13] = "AppConfig";
    BaseDirectory[BaseDirectory["AppData"] = 14] = "AppData";
    BaseDirectory[BaseDirectory["AppLocalData"] = 15] = "AppLocalData";
    BaseDirectory[BaseDirectory["AppCache"] = 16] = "AppCache";
    BaseDirectory[BaseDirectory["AppLog"] = 17] = "AppLog";
    BaseDirectory[BaseDirectory["Desktop"] = 18] = "Desktop";
    BaseDirectory[BaseDirectory["Executable"] = 19] = "Executable";
    BaseDirectory[BaseDirectory["Font"] = 20] = "Font";
    BaseDirectory[BaseDirectory["Home"] = 21] = "Home";
    BaseDirectory[BaseDirectory["Runtime"] = 22] = "Runtime";
    BaseDirectory[BaseDirectory["Template"] = 23] = "Template";
})(BaseDirectory || (BaseDirectory = {}));
/**
 * Returns the path to the suggested directory for your app's config files.
 * Resolves to `${configDir}/${bundleIdentifier}`, where `bundleIdentifier` is the [`identifier`](https://v2.tauri.app/reference/config/#identifier) value configured in `tauri.conf.json`.
 * @example
 * ```typescript
 * import { appConfigDir } from '@tauri-apps/api/path';
 * const appConfigDirPath = await appConfigDir();
 * ```
 *
 * @since 1.2.0
 */
async function appConfigDir() {
    return invoke('plugin:path|resolve_directory', {
        directory: BaseDirectory.AppConfig
    });
}
/**
 * Returns the path to the suggested directory for your app's data files.
 * Resolves to `${dataDir}/${bundleIdentifier}`, where `bundleIdentifier` is the [`identifier`](https://v2.tauri.app/reference/config/#identifier) value configured in `tauri.conf.json`.
 * @example
 * ```typescript
 * import { appDataDir } from '@tauri-apps/api/path';
 * const appDataDirPath = await appDataDir();
 * ```
 *
 * @since 1.2.0
 */
async function appDataDir() {
    return invoke('plugin:path|resolve_directory', {
        directory: BaseDirectory.AppData
    });
}
/**
 * Returns the path to the suggested directory for your app's local data files.
 * Resolves to `${localDataDir}/${bundleIdentifier}`, where `bundleIdentifier` is the [`identifier`](https://v2.tauri.app/reference/config/#identifier) value configured in `tauri.conf.json`.
 * @example
 * ```typescript
 * import { appLocalDataDir } from '@tauri-apps/api/path';
 * const appLocalDataDirPath = await appLocalDataDir();
 * ```
 *
 * @since 1.2.0
 */
async function appLocalDataDir() {
    return invoke('plugin:path|resolve_directory', {
        directory: BaseDirectory.AppLocalData
    });
}
/**
 * Returns the path to the suggested directory for your app's cache files.
 * Resolves to `${cacheDir}/${bundleIdentifier}`, where `bundleIdentifier` is the [`identifier`](https://v2.tauri.app/reference/config/#identifier) value configured in `tauri.conf.json`.
 * @example
 * ```typescript
 * import { appCacheDir } from '@tauri-apps/api/path';
 * const appCacheDirPath = await appCacheDir();
 * ```
 *
 * @since 1.2.0
 */
async function appCacheDir() {
    return invoke('plugin:path|resolve_directory', {
        directory: BaseDirectory.AppCache
    });
}
/**
 * Returns the path to the user's audio directory.
 *
 * #### Platform-specific
 *
 * - **Linux:** Resolves to [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)' `XDG_MUSIC_DIR`.
 * - **macOS:** Resolves to `$HOME/Music`.
 * - **Windows:** Resolves to `{FOLDERID_Music}`.
 * @example
 * ```typescript
 * import { audioDir } from '@tauri-apps/api/path';
 * const audioDirPath = await audioDir();
 * ```
 *
 * @since 1.0.0
 */
async function audioDir() {
    return invoke('plugin:path|resolve_directory', {
        directory: BaseDirectory.Audio
    });
}
/**
 * Returns the path to the user's cache directory.
 *
 * #### Platform-specific
 *
 * - **Linux:** Resolves to `$XDG_CACHE_HOME` or `$HOME/.cache`.
 * - **macOS:** Resolves to `$HOME/Library/Caches`.
 * - **Windows:** Resolves to `{FOLDERID_LocalAppData}`.
 * @example
 * ```typescript
 * import { cacheDir } from '@tauri-apps/api/path';
 * const cacheDirPath = await cacheDir();
 * ```
 *
 * @since 1.0.0
 */
async function cacheDir() {
    return invoke('plugin:path|resolve_directory', {
        directory: BaseDirectory.Cache
    });
}
/**
 * Returns the path to the user's config directory.
 *
 * #### Platform-specific
 *
 * - **Linux:** Resolves to `$XDG_CONFIG_HOME` or `$HOME/.config`.
 * - **macOS:** Resolves to `$HOME/Library/Application Support`.
 * - **Windows:** Resolves to `{FOLDERID_RoamingAppData}`.
 * @example
 * ```typescript
 * import { configDir } from '@tauri-apps/api/path';
 * const configDirPath = await configDir();
 * ```
 *
 * @since 1.0.0
 */
async function configDir() {
    return invoke('plugin:path|resolve_directory', {
        directory: BaseDirectory.Config
    });
}
/**
 * Returns the path to the user's data directory.
 *
 * #### Platform-specific
 *
 * - **Linux:** Resolves to `$XDG_DATA_HOME` or `$HOME/.local/share`.
 * - **macOS:** Resolves to `$HOME/Library/Application Support`.
 * - **Windows:** Resolves to `{FOLDERID_RoamingAppData}`.
 * @example
 * ```typescript
 * import { dataDir } from '@tauri-apps/api/path';
 * const dataDirPath = await dataDir();
 * ```
 *
 * @since 1.0.0
 */
async function dataDir() {
    return invoke('plugin:path|resolve_directory', {
        directory: BaseDirectory.Data
    });
}
/**
 * Returns the path to the user's desktop directory.
 *
 * #### Platform-specific
 *
 * - **Linux:** Resolves to [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)' `XDG_DESKTOP_DIR`.
 * - **macOS:** Resolves to `$HOME/Desktop`.
 * - **Windows:** Resolves to `{FOLDERID_Desktop}`.
 * @example
 * ```typescript
 * import { desktopDir } from '@tauri-apps/api/path';
 * const desktopPath = await desktopDir();
 * ```
 *
 * @since 1.0.0
 */
async function desktopDir() {
    return invoke('plugin:path|resolve_directory', {
        directory: BaseDirectory.Desktop
    });
}
/**
 * Returns the path to the user's document directory.
 * @example
 * ```typescript
 * import { documentDir } from '@tauri-apps/api/path';
 * const documentDirPath = await documentDir();
 * ```
 *
 * #### Platform-specific
 *
 * - **Linux:** Resolves to [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)' `XDG_DOCUMENTS_DIR`.
 * - **macOS:** Resolves to `$HOME/Documents`.
 * - **Windows:** Resolves to `{FOLDERID_Documents}`.
 *
 * @since 1.0.0
 */
async function documentDir() {
    return invoke('plugin:path|resolve_directory', {
        directory: BaseDirectory.Document
    });
}
/**
 * Returns the path to the user's download directory.
 *
 * #### Platform-specific
 *
 * - **Linux**: Resolves to [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)' `XDG_DOWNLOAD_DIR`.
 * - **macOS**: Resolves to `$HOME/Downloads`.
 * - **Windows**: Resolves to `{FOLDERID_Downloads}`.
 * @example
 * ```typescript
 * import { downloadDir } from '@tauri-apps/api/path';
 * const downloadDirPath = await downloadDir();
 * ```
 *
 * @since 1.0.0
 */
async function downloadDir() {
    return invoke('plugin:path|resolve_directory', {
        directory: BaseDirectory.Download
    });
}
/**
 * Returns the path to the user's executable directory.
 *
 * #### Platform-specific
 *
 * - **Linux:** Resolves to `$XDG_BIN_HOME/../bin` or `$XDG_DATA_HOME/../bin` or `$HOME/.local/bin`.
 * - **macOS:** Not supported.
 * - **Windows:** Not supported.
 * @example
 * ```typescript
 * import { executableDir } from '@tauri-apps/api/path';
 * const executableDirPath = await executableDir();
 * ```
 *
 * @since 1.0.0
 */
async function executableDir() {
    return invoke('plugin:path|resolve_directory', {
        directory: BaseDirectory.Executable
    });
}
/**
 * Returns the path to the user's font directory.
 *
 * #### Platform-specific
 *
 * - **Linux:** Resolves to `$XDG_DATA_HOME/fonts` or `$HOME/.local/share/fonts`.
 * - **macOS:** Resolves to `$HOME/Library/Fonts`.
 * - **Windows:** Not supported.
 * @example
 * ```typescript
 * import { fontDir } from '@tauri-apps/api/path';
 * const fontDirPath = await fontDir();
 * ```
 *
 * @since 1.0.0
 */
async function fontDir() {
    return invoke('plugin:path|resolve_directory', {
        directory: BaseDirectory.Font
    });
}
/**
 * Returns the path to the user's home directory.
 *
 * #### Platform-specific
 *
 * - **Linux:** Resolves to `$HOME`.
 * - **macOS:** Resolves to `$HOME`.
 * - **Windows:** Resolves to `{FOLDERID_Profile}`.
 * @example
 * ```typescript
 * import { homeDir } from '@tauri-apps/api/path';
 * const homeDirPath = await homeDir();
 * ```
 *
 * @since 1.0.0
 */
async function homeDir() {
    return invoke('plugin:path|resolve_directory', {
        directory: BaseDirectory.Home
    });
}
/**
 * Returns the path to the user's local data directory.
 *
 * #### Platform-specific
 *
 * - **Linux:** Resolves to `$XDG_DATA_HOME` or `$HOME/.local/share`.
 * - **macOS:** Resolves to `$HOME/Library/Application Support`.
 * - **Windows:** Resolves to `{FOLDERID_LocalAppData}`.
 * @example
 * ```typescript
 * import { localDataDir } from '@tauri-apps/api/path';
 * const localDataDirPath = await localDataDir();
 * ```
 *
 * @since 1.0.0
 */
async function localDataDir() {
    return invoke('plugin:path|resolve_directory', {
        directory: BaseDirectory.LocalData
    });
}
/**
 * Returns the path to the user's picture directory.
 *
 * #### Platform-specific
 *
 * - **Linux:** Resolves to [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)' `XDG_PICTURES_DIR`.
 * - **macOS:** Resolves to `$HOME/Pictures`.
 * - **Windows:** Resolves to `{FOLDERID_Pictures}`.
 * @example
 * ```typescript
 * import { pictureDir } from '@tauri-apps/api/path';
 * const pictureDirPath = await pictureDir();
 * ```
 *
 * @since 1.0.0
 */
async function pictureDir() {
    return invoke('plugin:path|resolve_directory', {
        directory: BaseDirectory.Picture
    });
}
/**
 * Returns the path to the user's public directory.
 *
 * #### Platform-specific
 *
 * - **Linux:** Resolves to [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)' `XDG_PUBLICSHARE_DIR`.
 * - **macOS:** Resolves to `$HOME/Public`.
 * - **Windows:** Resolves to `{FOLDERID_Public}`.
 * @example
 * ```typescript
 * import { publicDir } from '@tauri-apps/api/path';
 * const publicDirPath = await publicDir();
 * ```
 *
 * @since 1.0.0
 */
async function publicDir() {
    return invoke('plugin:path|resolve_directory', {
        directory: BaseDirectory.Public
    });
}
/**
 * Returns the path to the application's resource directory.
 * To resolve a resource path, see the [[resolveResource | `resolveResource API`]].
 * @example
 * ```typescript
 * import { resourceDir } from '@tauri-apps/api/path';
 * const resourceDirPath = await resourceDir();
 * ```
 *
 * @since 1.0.0
 */
async function resourceDir() {
    return invoke('plugin:path|resolve_directory', {
        directory: BaseDirectory.Resource
    });
}
/**
 * Resolve the path to a resource file.
 * @example
 * ```typescript
 * import { resolveResource } from '@tauri-apps/api/path';
 * const resourcePath = await resolveResource('script.sh');
 * ```
 *
 * @param resourcePath The path to the resource.
 * Must follow the same syntax as defined in `tauri.conf.json > bundle > resources`, i.e. keeping subfolders and parent dir components (`../`).
 * @returns The full path to the resource.
 *
 * @since 1.0.0
 */
async function resolveResource(resourcePath) {
    return invoke('plugin:path|resolve_directory', {
        directory: BaseDirectory.Resource,
        path: resourcePath
    });
}
/**
 * Returns the path to the user's runtime directory.
 *
 * #### Platform-specific
 *
 * - **Linux:** Resolves to `$XDG_RUNTIME_DIR`.
 * - **macOS:** Not supported.
 * - **Windows:** Not supported.
 * @example
 * ```typescript
 * import { runtimeDir } from '@tauri-apps/api/path';
 * const runtimeDirPath = await runtimeDir();
 * ```
 *
 * @since 1.0.0
 */
async function runtimeDir() {
    return invoke('plugin:path|resolve_directory', {
        directory: BaseDirectory.Runtime
    });
}
/**
 * Returns the path to the user's template directory.
 *
 * #### Platform-specific
 *
 * - **Linux:** Resolves to [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)' `XDG_TEMPLATES_DIR`.
 * - **macOS:** Not supported.
 * - **Windows:** Resolves to `{FOLDERID_Templates}`.
 * @example
 * ```typescript
 * import { templateDir } from '@tauri-apps/api/path';
 * const templateDirPath = await templateDir();
 * ```
 *
 * @since 1.0.0
 */
async function templateDir() {
    return invoke('plugin:path|resolve_directory', {
        directory: BaseDirectory.Template
    });
}
/**
 * Returns the path to the user's video directory.
 *
 * #### Platform-specific
 *
 * - **Linux:** Resolves to [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)' `XDG_VIDEOS_DIR`.
 * - **macOS:** Resolves to `$HOME/Movies`.
 * - **Windows:** Resolves to `{FOLDERID_Videos}`.
 * @example
 * ```typescript
 * import { videoDir } from '@tauri-apps/api/path';
 * const videoDirPath = await videoDir();
 * ```
 *
 * @since 1.0.0
 */
async function videoDir() {
    return invoke('plugin:path|resolve_directory', {
        directory: BaseDirectory.Video
    });
}
/**
 * Returns the path to the suggested directory for your app's log files.
 *
 * #### Platform-specific
 *
 * - **Linux:** Resolves to `${configDir}/${bundleIdentifier}/logs`.
 * - **macOS:** Resolves to `${homeDir}/Library/Logs/{bundleIdentifier}`
 * - **Windows:** Resolves to `${configDir}/${bundleIdentifier}/logs`.
 * @example
 * ```typescript
 * import { appLogDir } from '@tauri-apps/api/path';
 * const appLogDirPath = await appLogDir();
 * ```
 *
 * @since 1.2.0
 */
async function appLogDir() {
    return invoke('plugin:path|resolve_directory', {
        directory: BaseDirectory.AppLog
    });
}
/**
 * Returns a temporary directory.
 * @example
 * ```typescript
 * import { tempDir } from '@tauri-apps/api/path';
 * const temp = await tempDir();
 * ```
 *
 * @since 2.0.0
 */
async function tempDir() {
    return invoke('plugin:path|resolve_directory', {
        directory: BaseDirectory.Temp
    });
}
/**
 * Returns the platform-specific path segment separator:
 * - `\` on Windows
 * - `/` on POSIX
 *
 * @since 2.0.0
 */
function sep() {
    return window.__TAURI_INTERNALS__.plugins.path.sep;
}
/**
 * Returns the platform-specific path segment delimiter:
 * - `;` on Windows
 * - `:` on POSIX
 *
 * @since 2.0.0
 */
function delimiter() {
    return window.__TAURI_INTERNALS__.plugins.path.delimiter;
}
/**
 * Resolves a sequence of `paths` or `path` segments into an absolute path.
 * @example
 * ```typescript
 * import { resolve, appDataDir } from '@tauri-apps/api/path';
 * const appDataDirPath = await appDataDir();
 * const path = await resolve(appDataDirPath, '..', 'users', 'tauri', 'avatar.png');
 * ```
 *
 * @since 1.0.0
 */
async function resolve(...paths) {
    return invoke('plugin:path|resolve', { paths });
}
/**
 * Normalizes the given `path`, resolving `'..'` and `'.'` segments and resolve symbolic links.
 * @example
 * ```typescript
 * import { normalize, appDataDir } from '@tauri-apps/api/path';
 * const appDataDirPath = await appDataDir();
 * const path = await normalize(`${appDataDirPath}/../users/tauri/avatar.png`);
 * ```
 *
 * @since 1.0.0
 */
async function normalize(path) {
    return invoke('plugin:path|normalize', { path });
}
/**
 *  Joins all given `path` segments together using the platform-specific separator as a delimiter, then normalizes the resulting path.
 * @example
 * ```typescript
 * import { join, appDataDir } from '@tauri-apps/api/path';
 * const appDataDirPath = await appDataDir();
 * const path = await join(appDataDirPath, 'users', 'tauri', 'avatar.png');
 * ```
 *
 * @since 1.0.0
 */
async function join(...paths) {
    return invoke('plugin:path|join', { paths });
}
/**
 * Returns the directory name of a `path`. Trailing directory separators are ignored.
 * @example
 * ```typescript
 * import { dirname } from '@tauri-apps/api/path';
 * const dir = await dirname('/path/to/somedir/');
 * assert(dir === 'somedir');
 * ```
 *
 * @since 1.0.0
 */
async function dirname(path) {
    return invoke('plugin:path|dirname', { path });
}
/**
 * Returns the extension of the `path`.
 * @example
 * ```typescript
 * import { extname } from '@tauri-apps/api/path';
 * const ext = await extname('/path/to/file.html');
 * assert(ext === 'html');
 * ```
 *
 * @since 1.0.0
 */
async function extname(path) {
    return invoke('plugin:path|extname', { path });
}
/**
 * Returns the last portion of a `path`. Trailing directory separators are ignored.
 * @example
 * ```typescript
 * import { basename } from '@tauri-apps/api/path';
 * const base = await basename('path/to/app.conf');
 * assert(base === 'app.conf');
 * ```
 * @param ext An optional file extension to be removed from the returned path.
 *
 * @since 1.0.0
 */
async function basename(path, ext) {
    return invoke('plugin:path|basename', { path, ext });
}
/**
 * Returns whether the path is absolute or not.
 * @example
 * ```typescript
 * import { isAbsolute } from '@tauri-apps/api/path';
 * assert(await isAbsolute('/home/tauri'));
 * ```
 *
 * @since 1.0.0
 */
async function isAbsolute(path) {
    return invoke('plugin:path|isAbsolute', { path });
}

export { BaseDirectory, appCacheDir, appConfigDir, appDataDir, appLocalDataDir, appLogDir, audioDir, basename, cacheDir, configDir, dataDir, delimiter, desktopDir, dirname, documentDir, downloadDir, executableDir, extname, fontDir, homeDir, isAbsolute, join, localDataDir, normalize, pictureDir, publicDir, resolve, resolveResource, resourceDir, runtimeDir, sep, tempDir, templateDir, videoDir };
