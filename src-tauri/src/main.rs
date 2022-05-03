#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
    SystemTraySubmenu,
};

use system::SystemInfo;

mod system;

fn generate_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let hide = CustomMenuItem::new("hide".to_string(), "隐藏");

    let white_font = CustomMenuItem::new("white-font".to_string(), "白色");
    let black_font = CustomMenuItem::new("black-font".to_string(), "黑色");
    let font_color_submenu = SystemTraySubmenu::new(
        "字体颜色",
        SystemTrayMenu::new()
            .add_item(white_font)
            .add_item(black_font),
    );

    let tray_menu = SystemTrayMenu::new()
        .add_submenu(font_color_submenu)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}

fn main() {
    tauri::Builder::default()
        .system_tray(generate_tray())
        .invoke_handler(tauri::generate_handler![get_system_info])
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                }
                "white-font" => {
                    let window = app.get_window("main").unwrap();
                    window.eval("window['setFontColor']('white')").unwrap();
                }
                "black-font" => {
                    let window = app.get_window("main").unwrap();
                    window.eval("window['setFontColor']('black')").unwrap();
                }
                _ => {}
            },
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn get_system_info() -> SystemInfo {
    SystemInfo::get().await
}
