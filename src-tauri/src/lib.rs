use tauri::{
    menu::{CheckMenuItemBuilder, MenuBuilder, MenuItemBuilder, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    window::Color,
    AppHandle, Manager, PhysicalPosition, Runtime, Url, WebviewUrl, WebviewWindow,
    WebviewWindowBuilder, WindowEvent,
};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

const PANEL_LABEL: &str = "panel";
const CHATGPT_URL: &str = "https://chatgpt.com";

// Lazily created on first toggle so the WebView2 process (and chatgpt.com's
// heavy React bundle) never loads until the user actually opens the panel.
fn get_or_create_panel<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<WebviewWindow<R>> {
    if let Some(window) = app.get_webview_window(PANEL_LABEL) {
        return Ok(window);
    }

    let window = WebviewWindowBuilder::new(
        app,
        PANEL_LABEL,
        WebviewUrl::External(Url::parse(CHATGPT_URL).expect("valid url")),
    )
    .inner_size(400.0, 600.0)
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .resizable(false)
    .shadow(true)
    .visible(false)
    .focused(false)
    .background_color(Color(13, 13, 13, 255))
    .build()?;

    let hide_on_blur = window.clone();
    window.on_window_event(move |event| {
        if let WindowEvent::Focused(false) = event {
            hide_on_blur.hide().ok();
        }
    });

    Ok(window)
}

fn position_bottom_right<R: Runtime>(window: &WebviewWindow<R>) {
    let (Ok(Some(monitor)), Ok(size)) = (window.primary_monitor(), window.outer_size()) else {
        return;
    };
    let work_area = monitor.work_area();
    let margin = 12i32;
    let x = work_area.position.x + work_area.size.width as i32 - size.width as i32 - margin;
    let y = work_area.position.y + work_area.size.height as i32 - size.height as i32 - margin;
    window.set_position(PhysicalPosition::new(x, y)).ok();
}

fn toggle_panel<R: Runtime>(window: &WebviewWindow<R>) {
    if window.is_visible().unwrap_or(false) {
        window.hide().ok();
    } else {
        position_bottom_right(window);
        window.show().ok();
        window.set_focus().ok();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        .setup(|app| {
            let shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::Space);

            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_handler(move |app, hit_shortcut, event| {
                        if hit_shortcut.id() == shortcut.id() && event.state() == ShortcutState::Pressed
                        {
                            if let Ok(window) = get_or_create_panel(app) {
                                toggle_panel(&window);
                            }
                        }
                    })
                    .build(),
            )?;
            app.global_shortcut().register(shortcut)?;

            let toggle_item = MenuItemBuilder::with_id("toggle", "Toggle").build(app)?;
            let autostart_enabled = app.autolaunch().is_enabled().unwrap_or(false);
            let autostart_item = CheckMenuItemBuilder::with_id("autostart", "Launch at Login")
                .checked(autostart_enabled)
                .build(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let menu = MenuBuilder::new(app)
                .items(&[
                    &toggle_item,
                    &autostart_item,
                    &PredefinedMenuItem::separator(app)?,
                    &quit_item,
                ])
                .build()?;

            let autostart_item_for_menu = autostart_item.clone();
            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "toggle" => {
                        if let Ok(window) = get_or_create_panel(app) {
                            toggle_panel(&window);
                        }
                    }
                    "autostart" => {
                        let mgr = app.autolaunch();
                        let enabled = mgr.is_enabled().unwrap_or(false);
                        let result = if enabled { mgr.disable() } else { mgr.enable() };
                        if result.is_ok() {
                            autostart_item_for_menu.set_checked(!enabled).ok();
                        }
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Ok(window) = get_or_create_panel(app) {
                            // toggle_panel() always repositions bottom-right on open,
                            // so there's no need to compute a position from the tray rect here.
                            toggle_panel(&window);
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
