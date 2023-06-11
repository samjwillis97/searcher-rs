use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

use crate::{config, constants};

pub fn new() -> SystemTray {
    // let config = config::Config::new();
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    // TODO: Work out how the fuck to do this
    // config.search_services.iter().for_each(|v| {
    //     &tray_menu.add_item(CustomMenuItem::new(v.name.to_string(), v.name.to_string()));
    // });

    return SystemTray::new().with_menu(tray_menu);
}

pub fn event_handler(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "hide" => {
                let window = app.get_window(constants::SEARCH_WIN_NAME).unwrap();
                window.hide().unwrap();
            }
            _ => {}
        },
        _ => {}
    }
}
