// tauri/tooling/api/src/helpers/event.ts
async function _unlisten(event, eventId) {
  await window.__TAURI_INVOKE__("plugin:event|unlisten", {
    event,
    eventId,
  });
}

async function listen(event, handler, options) {
  return window
    .__TAURI_INVOKE__("plugin:event|listen", {
      event,
      windowLabel: options?.target,
      handler: window.__TAURI__.transformCallback(handler),
    })
    .then((eventId) => {
      return async () => _unlisten(event, eventId);
    });
}

async function once(event, handler, options) {
  return listen(
    event,
    (eventData) => {
      handler(eventData);
      _unlisten(event, eventData.id).catch(() => {});
    },
    options
  );
}

async function emit(event, payload, options) {
  await window.__TAURI_INVOKE__("plugin:event|emit", {
    event,
    windowLabel: options?.target,
    payload,
  });
}

class LogicalSize {
  type = "Logical";
  width;
  height;

  constructor(width, height) {
    this.width = width;
    this.height = height;
  }
}

class PhysicalSize {
  type = "Physical";
  width;
  height;

  constructor(width, height) {
    this.width = width;
    this.height = height;
  }

  toLogical(scaleFactor) {
    return new LogicalSize(this.width / scaleFactor, this.height / scaleFactor);
  }
}

class LogicalPosition {
  type = "Logical";
  x;
  y;

  constructor(x, y) {
    this.x = x;
    this.y = y;
  }
}

class PhysicalPosition {
  type = "Physical";
  x;
  y;

  constructor(x, y) {
    this.x = x;
    this.y = y;
  }

  toLogical(scaleFactor) {
    return new LogicalPosition(this.x / scaleFactor, this.y / scaleFactor);
  }
}

class CloseRequestedEvent {
  event;
  windowLabel;

  id;
  _preventDefault = false;

  constructor(event) {
    this.event = event.event;
    this.windowLabel = event.windowLabel;
    this.id = event.id;
  }

  preventDefault() {
    this._preventDefault = true;
  }

  isPreventDefault() {
    return this._preventDefault;
  }
}

function getCurrent() {
  return new Window(window.__TAURI_METADATA__.__currentWindow.label, {
    skip: true,
  });
}

function getAll() {
  return window.__TAURI_METADATA__.__windows.map(
    (w) =>
      new Window(w.label, {
        skip: true,
      })
  );
}

const localTauriEvents = ["tauri://created", "tauri://error"];

class WindowHandle {
  label;
  listeners;

  constructor(label) {
    this.label = label;
    this.listeners = Object.create(null);
  }

  async listen(event, handler) {
    if (this._handleTauriEvent(event, handler)) {
      return Promise.resolve(() => {
        const listeners = this.listeners[event];
        listeners.splice(listeners.indexOf(handler), 1);
      });
    }
    return listen(event, handler, { target: this.label });
  }

  async once(event, handler) {
    if (this._handleTauriEvent(event, handler)) {
      return Promise.resolve(() => {
        const listeners = this.listeners[event];
        listeners.splice(listeners.indexOf(handler), 1);
      });
    }
    return once(event, handler, { target: this.label });
  }

  async emit(event, payload) {
    if (localTauriEvents.includes(event)) {
      for (const handler of this.listeners[event] || []) {
        handler({ event, id: -1, windowLabel: this.label, payload });
      }
      return Promise.resolve();
    }
    return emit(event, payload, { target: this.label });
  }

  _handleTauriEvent(event, handler) {
    if (localTauriEvents.includes(event)) {
      if (!(event in this.listeners)) {
        this.listeners[event] = [handler];
      } else {
        this.listeners[event].push(handler);
      }
      return true;
    }
    return false;
  }
}

class WindowManager extends WindowHandle {
  async scaleFactor() {
    return window.__TAURI_INVOKE__("plugin:window|scale_factor", {
      label: this.label,
    });
  }

  async innerPosition() {
    return window
      .__TAURI_INVOKE__("plugin:window|inner_position", {
        label: this.label,
      })
      .then(({ x, y }) => new PhysicalPosition(x, y));
  }

  async outerPosition() {
    return window
      .__TAURI_INVOKE__("plugin:window|outer_position", {
        label: this.label,
      })
      .then(({ x, y }) => new PhysicalPosition(x, y));
  }

  async innerSize() {
    return window
      .__TAURI_INVOKE__("plugin:window|inner_size", {
        label: this.label,
      })
      .then(({ width, height }) => new PhysicalSize(width, height));
  }

  async outerSize() {
    return window
      .__TAURI_INVOKE__("plugin:window|outer_size", {
        label: this.label,
      })
      .then(({ width, height }) => new PhysicalSize(width, height));
  }

  async isFullscreen() {
    return window.__TAURI_INVOKE__("plugin:window|is_fullscreen", {
      label: this.label,
    });
  }

  async isMinimized() {
    return window.__TAURI_INVOKE__("plugin:window|is_minimized", {
      label: this.label,
    });
  }

  async isMaximized() {
    return window.__TAURI_INVOKE__("plugin:window|is_maximized", {
      label: this.label,
    });
  }

  async isFocused() {
    return window.__TAURI_INVOKE__("plugin:window|is_focused", {
      label: this.label,
    });
  }

  async isDecorated() {
    return window.__TAURI_INVOKE__("plugin:window|is_decorated", {
      label: this.label,
    });
  }

  async isResizable() {
    return window.__TAURI_INVOKE__("plugin:window|is_resizable", {
      label: this.label,
    });
  }

  async isMaximizable() {
    return window.__TAURI_INVOKE__("plugin:window|is_maximizable", {
      label: this.label,
    });
  }

  async isMinimizable() {
    return window.__TAURI_INVOKE__("plugin:window|is_minimizable", {
      label: this.label,
    });
  }

  async isClosable() {
    return window.__TAURI_INVOKE__("plugin:window|is_closable", {
      label: this.label,
    });
  }

  async isVisible() {
    return window.__TAURI_INVOKE__("plugin:window|is_visible", {
      label: this.label,
    });
  }

  async title() {
    return window.__TAURI_INVOKE__("plugin:window|title", {
      label: this.label,
    });
  }

  async theme() {
    return window.__TAURI_INVOKE__("plugin:window|theme", {
      label: this.label,
    });
  }

  async center() {
    return window.__TAURI_INVOKE__("plugin:window|center", {
      label: this.label,
    });
  }

  async requestUserAttention(requestType) {
    let requestType_ = null;
    if (requestType) {
      if (requestType === 1) {
        requestType_ = { type: "Critical" };
      } else {
        requestType_ = { type: "Informational" };
      }
    }

    return window.__TAURI_INVOKE__("plugin:window|request_user_attention", {
      label: this.label,
      value: requestType_,
    });
  }

  async setResizable(resizable) {
    return window.__TAURI_INVOKE__("plugin:window|set_resizable", {
      label: this.label,
      value: resizable,
    });
  }

  async setMaximizable(maximizable) {
    return window.__TAURI_INVOKE__("plugin:window|set_maximizable", {
      label: this.label,
      value: maximizable,
    });
  }

  async setMinimizable(minimizable) {
    return window.__TAURI_INVOKE__("plugin:window|set_minimizable", {
      label: this.label,
      value: minimizable,
    });
  }

  async setClosable(closable) {
    return window.__TAURI_INVOKE__("plugin:window|set_closable", {
      label: this.label,
      value: closable,
    });
  }

  async setTitle(title) {
    return window.__TAURI_INVOKE__("plugin:window|set_title", {
      label: this.label,
      value: title,
    });
  }

  async maximize() {
    return window.__TAURI_INVOKE__("plugin:window|maximize", {
      label: this.label,
    });
  }
  async unmaximize() {
    return window.__TAURI_INVOKE__("plugin:window|unmaximize", {
      label: this.label,
    });
  }

  async toggleMaximize() {
    return window.__TAURI_INVOKE__("plugin:window|toggle_maximize", {
      label: this.label,
    });
  }

  async minimize() {
    return window.__TAURI_INVOKE__("plugin:window|minimize", {
      label: this.label,
    });
  }

  async unminimize() {
    return window.__TAURI_INVOKE__("plugin:window|unminimize", {
      label: this.label,
    });
  }

  async show() {
    return window.__TAURI_INVOKE__("plugin:window|show", {
      label: this.label,
    });
  }

  async hide() {
    return window.__TAURI_INVOKE__("plugin:window|hide", {
      label: this.label,
    });
  }

  async close() {
    return window.__TAURI_INVOKE__("plugin:window|close", {
      label: this.label,
    });
  }

  async setDecorations(decorations) {
    return window.__TAURI_INVOKE__("plugin:window|set_decorations", {
      label: this.label,
      value: decorations,
    });
  }

  // TODO:
  async setShadow(enable) {
    return window.__TAURI_INVOKE__("plugin:window|set_shadow", {
      label: this.label,
      value: enable,
    });
  }

  // TODO:
  async setEffects(effects) {
    return window.__TAURI_INVOKE__("plugin:window|set_effects", {
      label: this.label,
      value: effects,
    });
  }

  // TODO:
  async clearEffects() {
    return window.__TAURI_INVOKE__("plugin:window|set_effects", {
      label: this.label,
      value: null,
    });
  }

  async setAlwaysOnTop(alwaysOnTop) {
    return window.__TAURI_INVOKE__("plugin:window|set_always_on_top", {
      label: this.label,
      value: alwaysOnTop,
    });
  }

  async setContentProtected(protected_) {
    return window.__TAURI_INVOKE__("plugin:window|set_content_protected", {
      label: this.label,
      value: protected_,
    });
  }

  async setSize(size) {
    if (!size || (size.type !== "Logical" && size.type !== "Physical")) {
      throw new Error("the `size` argument must be either a LogicalSize or a PhysicalSize instance");
    }

    return window.__TAURI_INVOKE__("plugin:window|set_size", {
      label: this.label,
      value: {
        type: size.type,
        data: {
          width: size.width,
          height: size.height,
        },
      },
    });
  }

  async setMinSize(size) {
    if (size && size.type !== "Logical" && size.type !== "Physical") {
      throw new Error("the `size` argument must be either a LogicalSize or a PhysicalSize instance");
    }

    return window.__TAURI_INVOKE__("plugin:window|set_min_size", {
      label: this.label,
      value: size
        ? {
            type: size.type,
            data: {
              width: size.width,
              height: size.height,
            },
          }
        : null,
    });
  }

  async setMaxSize(size) {
    if (size && size.type !== "Logical" && size.type !== "Physical") {
      throw new Error("the `size` argument must be either a LogicalSize or a PhysicalSize instance");
    }

    return window.__TAURI_INVOKE__("plugin:window|set_max_size", {
      label: this.label,
      value: size
        ? {
            type: size.type,
            data: {
              width: size.width,
              height: size.height,
            },
          }
        : null,
    });
  }

  async setPosition(position) {
    if (!position || (position.type !== "Logical" && position.type !== "Physical")) {
      throw new Error("the `position` argument must be either a LogicalPosition or a PhysicalPosition instance");
    }

    return window.__TAURI_INVOKE__("plugin:window|set_position", {
      label: this.label,
      value: {
        type: position.type,
        data: {
          x: position.x,
          y: position.y,
        },
      },
    });
  }

  async setFullscreen(fullscreen) {
    return window.__TAURI_INVOKE__("plugin:window|set_fullscreen", {
      label: this.label,
      value: fullscreen,
    });
  }

  async setFocus() {
    return window.__TAURI_INVOKE__("plugin:window|set_focus", {
      label: this.label,
    });
  }

  async setIcon(icon) {
    return window.__TAURI_INVOKE__("plugin:window|set_icon", {
      label: this.label,
      value: typeof icon === "string" ? icon : Array.from(icon),
    });
  }

  async setSkipTaskbar(skip) {
    return window.__TAURI_INVOKE__("plugin:window|set_skip_taskbar", {
      label: this.label,
      value: skip,
    });
  }

  async setCursorGrab(grab) {
    return window.__TAURI_INVOKE__("plugin:window|set_cursor_grab", {
      label: this.label,
      value: grab,
    });
  }

  async setCursorVisible(visible) {
    return window.__TAURI_INVOKE__("plugin:window|set_cursor_visible", {
      label: this.label,
      value: visible,
    });
  }

  async setCursorIcon(icon) {
    return window.__TAURI_INVOKE__("plugin:window|set_cursor_icon", {
      label: this.label,
      value: icon,
    });
  }

  async setCursorPosition(position) {
    if (!position || (position.type !== "Logical" && position.type !== "Physical")) {
      throw new Error("the `position` argument must be either a LogicalPosition or a PhysicalPosition instance");
    }

    return window.__TAURI_INVOKE__("plugin:window|set_cursor_position", {
      label: this.label,
      value: {
        type: position.type,
        data: {
          x: position.x,
          y: position.y,
        },
      },
    });
  }

  async setIgnoreCursorEvents(ignore) {
    return window.__TAURI_INVOKE__("plugin:window|set_ignore_cursor_events", {
      label: this.label,
      value: ignore,
    });
  }

  async startDragging() {
    return window.__TAURI_INVOKE__("plugin:window|start_dragging", {
      label: this.label,
    });
  }

  async onResized(handler) {
    return this.listen(TauriEvent.WINDOW_RESIZED, (e) => {
      e.payload = mapPhysicalSize(e.payload);
      handler(e);
    });
  }

  async onMoved(handler) {
    return this.listen(TauriEvent.WINDOW_MOVED, (e) => {
      e.payload = mapPhysicalPosition(e.payload);
      handler(e);
    });
  }

  async onCloseRequested(handler) {
    return this.listen(TauriEvent.WINDOW_CLOSE_REQUESTED, (event) => {
      const evt = new CloseRequestedEvent(event);
      void Promise.resolve(handler(evt)).then(() => {
        if (!evt.isPreventDefault()) {
          return this.close();
        }
      });
    });
  }

  async onFocusChanged(handler) {
    const unlistenFocus = await this.listen(TauriEvent.WINDOW_FOCUS, (event) => {
      handler({ ...event, payload: true });
    });
    const unlistenBlur = await this.listen(TauriEvent.WINDOW_BLUR, (event) => {
      handler({ ...event, payload: false });
    });
    return () => {
      unlistenFocus();
      unlistenBlur();
    };
  }

  async onScaleChanged(handler) {
    return this.listen(TauriEvent.WINDOW_SCALE_FACTOR_CHANGED, handler);
  }

  async onMenuClicked(handler) {
    return this.listen(TauriEvent.MENU, handler);
  }

  async onFileDropEvent(handler) {
    const unlistenFileDrop = await this.listen(TauriEvent.WINDOW_FILE_DROP, (event) => {
      handler({ ...event, payload: { type: "drop", paths: event.payload } });
    });

    const unlistenFileHover = await this.listen(TauriEvent.WINDOW_FILE_DROP_HOVER, (event) => {
      handler({ ...event, payload: { type: "hover", paths: event.payload } });
    });

    const unlistenCancel = await this.listen(TauriEvent.WINDOW_FILE_DROP_CANCELLED, (event) => {
      handler({ ...event, payload: { type: "cancel" } });
    });

    return () => {
      unlistenFileDrop();
      unlistenFileHover();
      unlistenCancel();
    };
  }

  async onThemeChanged(handler) {
    return this.listen(TauriEvent.WINDOW_THEME_CHANGED, handler);
  }
}

class Window extends WindowManager {
  constructor(label, options = {}) {
    super(label);

    if (!options?.skip) {
      window
        .__TAURI_INVOKE__("plugin:window|create", {
          options: {
            ...options,
            label,
          },
        })
        .then(async () => this.emit("tauri://created"))
        .catch(async (e) => this.emit("tauri://error", e));
    }
  }

  static getByLabel(label) {
    if (getAll().some((w) => w.label === label)) {
      return new Window(label, { skip: true });
    }
    return null;
  }
}

function mapMonitor(m) {
  return m === null
    ? null
    : {
        name: m.name,
        scaleFactor: m.scaleFactor,
        osition: new PhysicalPosition(m.position.x, m.position.y),
        size: new PhysicalSize(m.size.width, m.size.height),
      };
}

async function currentMonitor() {
  return window.__TAURI_INVOKE__("plugin:window|current_monitor").then(mapMonitor);
}

async function primaryMonitor() {
  return window.__TAURI_INVOKE__("plugin:window|primary_monitor").then(mapMonitor);
}

async function availableMonitors() {
  return window.__TAURI_INVOKE__("plugin:window|available_monitors").then((ms) => ms.map(mapMonitor));
}

export {
  Window,
  WindowHandle,
  WindowManager,
  PhysicalPosition,
  PhysicalSize,
  LogicalPosition,
  LogicalSize,
  CloseRequestedEvent,
  getCurrent,
  getAll,
  currentMonitor,
  primaryMonitor,
  availableMonitors,
};
