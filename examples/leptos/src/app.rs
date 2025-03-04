use futures::stream::StreamExt;
use leptos::{
    either::{either, Either},
    ev::MouseEvent,
    prelude::*,
    task::spawn_local,
};
use std::rc::Rc;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="container">
            <div>
                <h2>"core"</h2>
                <Core/>
            </div>

            <div>
                <h2>"app"</h2>
                <TauriApp/>
            </div>

            <div>
                <h2>"events"</h2>
                <Events/>
            </div>

            <div>
                <h2>"window"</h2>
                <Window/>
            </div>

            <div>
                <h2>"menu"</h2>
                <Menu/>
            </div>
        </main>
    }
}

#[component]
fn Core() -> impl IntoView {
    let (convert_path, set_convert_path) = signal("".to_string());
    let (converted_path, set_converted_path) = signal("".to_string());

    let do_convert_path = move |_| {
        let converted = tauri_sys::core::convert_file_src(convert_path());
        set_converted_path(converted);
    };

    view! {
        <div>
            <div>
                "is Tauri? "
                {if tauri_sys::core::is_tauri() {
                    "Yes".to_string()
                } else {
                    "No".to_string()
                }}
            </div>
            <div>
                <label>
                    "Convert path"
                    <input
                        prop:value=convert_path
                        on:input=move |e| set_convert_path(event_target_value(&e))
                    />
                </label>
                <button on:click=do_convert_path>"Convert"</button>
            </div>
            <div>{converted_path}</div>
        </div>
    }
}

#[component]
fn TauriApp() -> impl IntoView {
    let app_name = LocalResource::new(tauri_sys::app::get_name);
    let tauri_version = LocalResource::new(tauri_sys::app::get_tauri_version);
    let app_version = LocalResource::new(tauri_sys::app::get_version);
    let default_window_icon = LocalResource::new(tauri_sys::app::default_window_icon);
    let set_theme = Action::new_local(|theme: &tauri_sys::app::Theme| {
        let theme = theme.clone();
        async move {
            tauri_sys::app::set_theme(theme).await;
        }
    });

    let hide = Action::new_local(|_| async move {
        #[cfg(target_os = "macos")]
        tauri_sys::app::hide().await.unwrap();
    });

    view! {
        <div>
            <div>
                "App name: "
                {move || match app_name.get() {
                    None => "Loading...".to_string(),
                    Some(name) => name.to_string(),
                }}
            </div>
            <div>
                "Tauri version: "
                {move || match tauri_version.get() {
                    None => "Loading...".to_string(),
                    Some(version) => version.to_string(),
                }}
            </div>
            <div>
                "App version: "
                {move || match app_version.get() {
                    None => "Loading...".to_string(),
                    Some(version) => version.to_string(),
                }}
            </div>
            <div>
                "Set theme"
                <button
                    on:click=move |_| { set_theme.dispatch(tauri_sys::app::Theme::Light); }
                >
                    "Light"
                </button>
                <button
                    on:click=move |_| { set_theme.dispatch(tauri_sys::app::Theme::Dark); }
                >
                    "Dark"
                </button>
                <button
                    on:click=move |_| { set_theme.dispatch(tauri_sys::app::Theme::System); }
                >
                    "System"
                </button>
            </div>
            <div>
                <button
                    on:click=move |_| { hide.dispatch(()); }
                >
                    "Hide (macOS only)"
                </button>
            </div>
            <div>
                "Default image"
                <div>
                    <Suspense
                        fallback=move || view! { "Loading" }
                    >
                        {move || Suspend::new(async move {
                            let icon = default_window_icon.await;
                            match icon {
                                None => Either::Left("No default image".to_string()),
                                Some(icon) => Either::Right(view! {
                                    <DefaultWindowIcon icon />
                                })
                            }
                        })}
                    </Suspense>
                </div>
            </div>
        </div>
    }
}

#[component]
fn DefaultWindowIcon(icon: tauri_sys::app::Image) -> impl IntoView {
    let size = LocalResource::new({
        let icon = icon.clone();
        move || {
            let icon = icon.clone();
            async move { icon.size().await }
        }
    });
    let rgba = LocalResource::new({
        let icon = icon.clone();
        move || {
            let icon = icon.clone();
            async move { icon.rgba().await }
        }
    });

    view! {
        <Suspense
            fallback=move || view! { "Loading" }
        >
            {move || Suspend::new(async move {
                let size = size.await;
                let rgba = rgba.await;

                view! {
                    <div>
                        <div>
                            "Size: "
                            {size.width()} "x" {size.height()}
                        </div>
                        <div>
                            "RGBA:"
                            <textarea readonly=true>
                                {rgba.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(", ")}
                            </textarea>
                        </div>
                    </div>
                }
            })}
        </Suspense>
    }
}

#[component]
fn Events() -> impl IntoView {
    let (listen_event, set_listen_event) = signal(None);
    let (emit_count, set_emit_count) = signal(0);

    spawn_local(async move {
        let mut listener = tauri_sys::event::listen::<i32>("event::listen")
            .await
            .unwrap();

        while let Some(event) = listener.next().await {
            tracing::debug!(?event);
            let tauri_sys::event::Event {
                event: _,
                id: _,
                payload,
            } = event;
            set_listen_event.set(Some(payload));
        }
    });

    spawn_local(async move {
        let mut listener = tauri_sys::event::listen::<i32>("event::emit")
            .await
            .unwrap();

        while let Some(event) = listener.next().await {
            tracing::debug!(?event);
            let tauri_sys::event::Event {
                event: _,
                id: _,
                payload,
            } = event;
            set_emit_count.set(payload);
        }
    });

    let trigger_listen_events = move |_| {
        spawn_local(async move {
            tauri_sys::core::invoke::<()>("trigger_listen_events", &()).await;
        });
    };

    let trigger_emit_event = move |_| {
        spawn_local(async move {
            tauri_sys::event::emit("event::emit", &emit_count.with_untracked(|n| n + 1))
                .await
                .unwrap();
        });
    };

    view! {
        <div>
            <div>
                <button on:click=trigger_listen_events>"Trigger listen events"</button>
                <div>
                    <strong>"Last listen event: "</strong>
                    {move || listen_event()}
                </div>
            </div>

            <div>
                <button on:click=trigger_emit_event>"Trigger emit event"</button>
                <div>
                    <strong>"Events emitted: "</strong>
                    {move || emit_count()}
                </div>
            </div>
        </div>
    }
}

#[component]
fn Window() -> impl IntoView {
    view! {
        <div>
            <div>
                <h3>"Windows"</h3>
                <WindowWindows/>
            </div>

            <div>
                <h3>"Monitors"</h3>
                <WindowMonitors/>
            </div>

            <div>
                <h3>"Events"</h3>
                <WindowEvents/>
            </div>
        </div>
    }
}

#[component]
fn WindowWindows() -> impl IntoView {
    let (current_window, set_current_window) =
        signal(tauri_sys::window::get_current().label().clone());

    let all_windows = Action::new_local(|_| async move {
        tauri_sys::window::get_all()
            .await
            .iter()
            .map(|window| window.label().clone())
            .collect::<Vec<_>>()
    });
    all_windows.dispatch(());

    let refresh = move |_| {
        all_windows.dispatch(());
        let current = tauri_sys::window::get_current();
        set_current_window(current.label().clone());
    };

    view! {
        <div>
            <div style="display: flex; justify-content: center; gap: 10px;">
                <div>"Current window:"</div>
                {current_window}
            </div>
            <div style="display: flex; justify-content: center; gap: 10px;">
                <div>"All windows:"</div>
                {move || {
                    all_windows.value()
                        .with(|windows| either!(windows,
                            None => "Loading",
                            Some(windows) => format!("[{}]", windows.join(", ")),
                        ))
                }}
            </div>
            <button on:click=refresh>"Refresh"</button>
        </div>
    }
}

#[component]
fn WindowMonitors() -> impl IntoView {
    let current_monitor =
        Action::new_local(|_| async move { tauri_sys::window::current_monitor().await });

    let primary_monitor =
        Action::new_local(|_| async move { tauri_sys::window::primary_monitor().await });

    let available_monitors =
        Action::new_local(|_| async move { tauri_sys::window::available_monitors().await });

    let monitor_from_point = Action::new_local(|(x, y): &(isize, isize)| {
        let x = x.clone();
        let y = y.clone();
        async move { tauri_sys::window::monitor_from_point(x, y).await }
    });

    // let cursor_position =
    //     create_action(|_| async move { tauri_sys::window::cursor_position().await });

    let refresh = move |_| {
        current_monitor.dispatch(());
        primary_monitor.dispatch(());
        available_monitors.dispatch(());
    };

    let oninput_monitor_from_point = move |e| {
        let value = event_target_value(&e);
        let Some((x, y)) = value.split_once(',') else {
            return;
        };

        let Ok(x) = x.parse::<isize>() else {
            return;
        };

        let Ok(y) = y.parse::<isize>() else {
            return;
        };

        monitor_from_point.dispatch((x, y));
    };

    current_monitor.dispatch(());
    primary_monitor.dispatch(());
    available_monitors.dispatch(());

    view! {
        <div>
            <div>
                <div style="display: flex; justify-content: center; gap: 10px;">
                    <div>"Current monitor:"</div>
                    {move || {
                        current_monitor
                            .value()
                            .with(|monitor| either!(monitor,
                                None => "Loading",
                                Some(Some(monitor)) => view! { <Monitor monitor/> },
                                Some(None) => "Could not detect monitor.",
                            ))
                    }}

                </div>
                <div style="display: flex; justify-content: center; gap: 10px;">
                    <div>"Primary monitor:"</div>
                    {move || {
                        primary_monitor
                            .value()
                            .with(|monitor| either!(monitor,
                                None => "Loading",
                                Some(Some(monitor)) => view! { <Monitor monitor/> },
                                Some(None) => "Could not detect monitor.",
                            ))
                    }}

                </div>
                <div style="display: flex; justify-content: center; gap: 10px;">
                    <div>"Available monitors:"</div>
                    {move || available_monitors
                        .value()
                        .with(|monitors| either!(monitors,
                            None => "Loading",
                            Some(monitors) => monitors
                                .iter()
                                .map(|monitor| view! { <Monitor monitor/> })
                                .collect::<Vec<_>>(),
                        ))
                    }
                </div>
                <button on:click=refresh>"Refresh"</button>
            </div>
            <div>
                <label>"Monitor from point" <input on:input=oninput_monitor_from_point/></label>
                <div style="margin: 0 auto;">
                    {move || {
                        monitor_from_point
                            .value()
                            .with(|monitor| either!(monitor,
                                None => "Enter an `x, y` coordinate.",
                                Some(Some(monitor)) => view! { <Monitor monitor/> },
                                Some(None) => "Could not detect monitor.",
                            ))
                    }}

                </div>
            </div>

            <div>
                // {move || {
                // cursor_position
                // .value()
                // .with(|position| {
                // position
                // .as_ref()
                // .map(|position| {
                // view! {
                // {position.x()}
                // ", "
                // {position.y()}
                // }
                // })
                // })
                // }}
                <div>"Cursor position: "</div>
                <div style="width: 50vw; height: 30vh; margin: 0 auto; border: 2px solid black; border-radius: 5px;">
                    // on:mousemove=move |_| cursor_position.dispatch(())
                    "TODO (See https://github.com/tauri-apps/tauri/issues/10340)"
                </div>
            </div>
        </div>
    }
}

#[component]
fn WindowEvents() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (drag_drop, set_drag_drop) = signal(None);

    let increment_count: Action<_, _> = Action::new_unsync(move |count: &usize| {
        let window = tauri_sys::window::get_current();
        let count = count.clone();
        async move {
            window.emit("count", count).await.unwrap();
        }
    });

    spawn_local(async move {
        let mut window = tauri_sys::window::get_current();
        let mut listener = window.listen::<usize>("count").await.unwrap();
        while let Some(event) = listener.next().await {
            set_count(event.payload);
        }
    });

    spawn_local(async move {
        let window = tauri_sys::window::get_current();
        let mut listener = window.on_drag_drop_event().await.unwrap();
        while let Some(event) = listener.next().await {
            set_drag_drop(Some(event));
        }
    });

    view! {
        <div>
            <div>
                "Count: " {count}
                <button on:click=move |_| {increment_count.dispatch(count() + 1);}>"+"</button>
            </div>

            <div>
                <h3>"Drag drop event"</h3>
                <div>
                    {move || if let Some(event) = drag_drop.get() {
                        Either::Left(view! { <DragDrop event=event.payload/> })
                    } else {
                        Either::Right("No event")
                    }}
                </div>
            </div>
        </div>
    }
}

#[component]
fn DragDrop(event: tauri_sys::window::DragDropEvent) -> impl IntoView {
    use tauri_sys::window::DragDropEvent;

    either!(event,
        DragDropEvent::Enter(payload) => {
            view! {
                <div>
                    <strong>"Enter"</strong>
                    <div>
                        "Paths: ["
                        {payload
                            .paths()
                            .iter()
                            .map(|path| path.to_string_lossy().to_string())
                            .collect::<Vec<_>>()
                            .join(", ")} "]"
                    </div>
                    <div>
                        "Position: " {payload.position().x()} ", " {payload.position().y()}
                    </div>
                </div>
            }
        },
        DragDropEvent::Over(payload) => {
            view! {
                <div>
                    <strong>"Over"</strong>
                    <div>
                        "Position: " {payload.position().x()} ", " {payload.position().y()}
                    </div>
                </div>
            }
        },
        DragDropEvent::Drop(payload) => {
            view! {
                <div>
                    <strong>"Drop"</strong>
                    <div>
                        "Paths: ["
                        {payload
                            .paths()
                            .iter()
                            .map(|path| path.to_string_lossy().to_string())
                            .collect::<Vec<_>>()
                            .join(", ")} "]"
                    </div>
                    <div>
                        "Position: " {payload.position().x()} ", " {payload.position().y()}
                    </div>
                </div>
            }
        },
        DragDropEvent::Leave => {
            view! { <strong>"Leave"</strong> }
        },
    )
}

#[component]
fn Monitor<'a>(monitor: &'a tauri_sys::window::Monitor) -> impl IntoView {
    view! {
        <div style="display: inline-block; text-align: left;">
            <div>"Name: " {monitor.name().clone()}</div>
            <div>"Size: " {monitor.size().width()} " x " {monitor.size().height()}</div>
            <div>"Position: " {monitor.position().x()} ", " {monitor.position().y()}</div>
            <div>"Scale: " {monitor.scale_factor()}</div>
        </div>
    }
}

#[component]
fn Menu() -> impl IntoView {
    let (event_manual, set_event_manual) = signal::<Option<String>>(None);
    let (event_with_items, set_event_with_items) = signal::<Option<String>>(None);

    // NB: Ensure `Menu::default` is working.
    spawn_local(async move {
        tauri_sys::menu::Menu::default().await;
    });

    let menu_manual = LocalResource::new(move || async move {
        let menu = tauri_sys::menu::Menu::with_id("tauri-sys-menu").await;
        let mut item_open =
            tauri_sys::menu::item::MenuItem::with_id("Item 1 - Manual", "manual-item_1").await;
        let mut item_close =
            tauri_sys::menu::item::MenuItem::with_id("Item 2 - Manual", "manual-item_2").await;
        menu.append_item(&item_open).await.unwrap();
        menu.append_item(&item_close).await.unwrap();

        spawn_local(async move {
            let mut listener_item_open = item_open.listen().fuse();
            let mut listener_item_close = item_close.listen().fuse();
            loop {
                futures::select! {
                    event = listener_item_open.next() => match event{
                        None => continue,
                        Some(event) => set_event_manual(Some(event.clone())),
                    },
                    event = listener_item_close.next() => match event{
                        None => continue,
                        Some(event) => set_event_manual(Some(event.clone())),
                    },
                }
            }
        });

        Rc::new(menu)
    });

    let menu_with_items = LocalResource::new(move || async move {
        let mut item_open = tauri_sys::menu::item::MenuItemOptions::new("Item 1 - w/ items");
        item_open.set_id("w_items-item_1");
        let mut item_close = tauri_sys::menu::item::MenuItemOptions::new("Item 2 - w/ items");
        item_close.set_id("w_items-item_2");
        let items = vec![item_open.into(), item_close.into()];

        let (menu, mut listeners) =
            tauri_sys::menu::Menu::with_id_and_items("tauri-sys_menu_w_items", items).await;
        let mut listener_item_open = listeners.remove(0).unwrap().fuse();
        let mut listener_item_close = listeners.remove(0).unwrap().fuse();

        spawn_local(async move {
            loop {
                futures::select! {
                    event = listener_item_open.next() => match event{
                        None => continue,
                        Some(event) => {
                            set_event_with_items(Some(event.clone()))
                        },
                    },
                    event = listener_item_close.next() => match event{
                        None => continue,
                        Some(event) => set_event_with_items(Some(event.clone())),
                    },
                }
            }
        });

        Rc::new(menu)
    });

    let open_menu_manual = move |_e: MouseEvent| {
        let menu = menu_manual.get().unwrap();
        spawn_local(async move {
            menu.popup().await.unwrap();
        });
    };

    let open_menu_with_items = move |_e: MouseEvent| {
        let menu = menu_with_items.get().unwrap();
        spawn_local(async move {
            menu.popup().await.unwrap();
        });
    };

    view! {
        <div
            on:mousedown=open_menu_manual
            style="margin: 0 auto 2em; width: 50vw; height: 10em; border: 1px black solid; border-radius: 5px;"
        >
            {event_manual}
        </div>

        <div
            on:mousedown=open_menu_with_items
            style="margin: auto; width: 50vw; height: 10em; border: 1px black solid; border-radius: 5px;"
        >
            {event_with_items}
        </div>
    }
}
