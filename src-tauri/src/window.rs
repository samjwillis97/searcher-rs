use std::collections::HashMap;

use crate::event::ClientEvent;
use crate::searcher::DataState;
use crate::{config, constants};
use tauri::api::dialog::{MessageDialogBuilder, MessageDialogButtons, MessageDialogKind};
use tauri::{AppHandle, LogicalSize, Manager, Monitor, Size, Window, WindowBuilder, WindowUrl};

/// Try and detect which monitor the window is on so that we can determine the
/// screen size
fn find_monitor(window: &Window) -> Option<Monitor> {
    if let Ok(Some(mon)) = window.primary_monitor() {
        Some(mon)
    } else if let Ok(Some(mon)) = window.current_monitor() {
        Some(mon)
    } else if let Ok(mut monitors) = window.available_monitors() {
        if monitors.is_empty() {
            None
        } else {
            monitors.pop()
        }
    } else {
        None
    }
}

pub fn hide_search_bar(window: &Window) {
    let _ = window.hide();
    let _ = window.emit(ClientEvent::ClearSearch.as_ref(), true);
    let _ = window.emit(ClientEvent::SetService.as_ref(), "");
}

pub fn show_search_bar(window: &Window, config: &config::Config) {
    let _ = window.emit(ClientEvent::FocusSearch.as_ref(), true);
    let _ = window.show();
    let _ = window.set_focus();
    let _ = window.set_always_on_top(true);

    position_search_bar(
        window,
        // FIXME: I am surely doing somethign wrong here..
        config
            .app_settings
            .as_ref()
            .unwrap()
            .position
            .as_ref()
            .unwrap()
            .to_string(),
    );
}

pub fn position_search_bar(window: &Window, position: String) {
    if let Some(monitor) = find_monitor(window) {
        let size = monitor.size();
        let scale = monitor.scale_factor();

        // NOTE: Position is measure from the top in Y - bigger number further down the screen
        let mut x: f64;
        let mut y: f64;

        if position.as_str().contains("top") {
            y = constants::INPUT_Y;
        } else if position.as_str().contains("bottom") {
            y = size.height as f64 / (scale * 2.0) + constants::INPUT_Y;
        } else {
            y = size.height as f64 / (scale * 2.0);
        }

        if position.as_str().contains("left") {
            x = (size.width as f64 / (scale * 4.0)) - (constants::INPUT_WIDTH / 2.0);
        } else if position.as_str().contains("right") {
            x = (size.width as f64 / (scale * 4.0)) * 3.00 - (constants::INPUT_WIDTH / 2.0);
        } else {
            x = (size.width as f64 / (scale * 2.0)) - (constants::INPUT_WIDTH / 2.0);
        }

        let _ = window.set_position(tauri::Position::Logical(tauri::LogicalPosition { x, y }));
    } else {
        println!("Unable to detect any monitors.");
    }
}

pub fn position_info(window: &Window, position: String) {
    if let Some(monitor) = find_monitor(window) {
        let size = monitor.size();
        let scale = monitor.scale_factor();

        // NOTE: Position is measure from the top in Y - bigger number further down the screen
        let mut x: f64;
        let mut y: f64;

        if position.as_str().contains("top") {
            y = constants::INPUT_Y;
        } else if position.as_str().contains("bottom") {
            y = size.height as f64 / (scale * 2.0) + constants::INPUT_Y;
        } else {
            y = size.height as f64 / (scale * 2.0);
        }

        if position.as_str().contains("left") {
            x = (size.width as f64 / (scale * 4.0)) - (455.0 / 2.0);
        } else if position.as_str().contains("right") {
            x = (size.width as f64 / (scale * 4.0)) * 3.00 - (455.0 / 2.0);
        } else {
            x = (size.width as f64 / (scale * 2.0)) - (455.0 / 2.0);
        }

        let _ = window.set_position(tauri::Position::Logical(tauri::LogicalPosition { x, y }));
    } else {
        println!("Unable to detect any monitors.");
    }
}

pub fn resize_window(window: &Window, height: f64) {
    let window_height = {
        if let Some(monitor) = find_monitor(window) {
            let size = monitor.size();
            let scale = monitor.scale_factor();
            Some((size.height as f64) / scale - constants::INPUT_Y)
        } else {
            None
        }
    };

    let height = if let Some(window_height) = window_height {
        window_height.min(height)
    } else {
        height
    };

    let _ = window.set_size(Size::Logical(LogicalSize {
        width: constants::INPUT_WIDTH,
        height,
    }));
}

pub fn resize_info_window(window: &Window, height: f64) {
    println!("Resizing info window");
    let window_height = {
        if let Some(monitor) = find_monitor(window) {
            let size = monitor.size();
            let scale = monitor.scale_factor();
            Some((size.height as f64) / scale)
        } else {
            None
        }
    };

    let height = if let Some(window_height) = window_height {
        window_height.min(height)
    } else {
        height
    };

    let _ = window.set_size(Size::Logical(LogicalSize {
        width: 455.00,
        height,
    }));
}

pub fn alert(window: &Window, title: &str, message: &str) {
    MessageDialogBuilder::new(title, message)
        .parent(window)
        .buttons(MessageDialogButtons::Ok)
        .kind(MessageDialogKind::Error)
        .show(|_| {});
}

fn show_window(window: &Window, config: &config::Config) {
    let _ = window.show();
    // A little hack to bring window to the front if its hiding behind something.
    let _ = window.set_always_on_top(true);
    let _ = window.set_always_on_top(false);
    let _ = window.set_focus();

    position_info(
        window,
        config
            .app_settings
            .as_ref()
            .unwrap()
            .position
            .as_ref()
            .unwrap()
            .to_string(),
    );
}

pub fn show_info_window(app: &AppHandle, config: &config::Config, id: &str, title: &str) {
    let window = if let Some(window) = app.get_window(constants::INFO_WIN_NAME) {
        window
    } else {
        WindowBuilder::new(
            app,
            constants::INFO_WIN_NAME,
            WindowUrl::App(("/info/".to_string() + id).into()),
        )
        .title(title)
        // .min_inner_size(450.0, 500.0)
        .max_inner_size(455.0, 500.0)
        .build()
        .expect("Unable to build window for updater")
    };
    show_window(&window, config);
}

pub fn close_window(window: &Window) {
    let _ = window.close();
}
