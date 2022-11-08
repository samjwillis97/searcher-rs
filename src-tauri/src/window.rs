use crate::constants;
use crate::event::ClientEvent;
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

pub fn show_search_bar(window: &Window) {
    let _ = window.emit(ClientEvent::FocusSearch.as_ref(), true);
    let _ = window.show();
    let _ = window.set_focus();
    let _ = window.set_always_on_top(true);
    center_search_bar(window);
}

pub fn center_search_bar(window: &Window) {
    if let Some(monitor) = find_monitor(window) {
        let size = monitor.size();
        let scale = monitor.scale_factor();

        let middle = (size.width as f64 / (scale * 2.0)) - (constants::INPUT_WIDTH / 2.0);

        let _ = window.set_position(tauri::Position::Logical(tauri::LogicalPosition {
            x: middle,
            y: constants::INPUT_Y,
        }));
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

pub fn alert(window: &Window, title: &str, message: &str) {
    MessageDialogBuilder::new(title, message)
        .parent(window)
        .buttons(MessageDialogButtons::Ok)
        .kind(MessageDialogKind::Error)
        .show(|_| {});
}

fn show_window(window: &Window) {
    let _ = window.show();
    // A little hack to bring window to the front if its hiding behind something.
    let _ = window.set_always_on_top(true);
    let _ = window.set_always_on_top(false);
    let _ = window.set_focus();
    let _ = window.center();
}

pub fn show_info_window(app: &AppHandle, id: &str) {
    let window = if let Some(window) = app.get_window(constants::INFO_WIN_NAME) {
        window
    } else {
        WindowBuilder::new(
            app,
            constants::INFO_WIN_NAME,
            WindowUrl::App(("/info/".to_string() + id).into()),
        )
        .title("Spyglass - Update Available!")
        // .min_inner_size(450.0, 375.0)
        // .max_inner_size(450.0, 375.0)
        .build()
        .expect("Unable to build window for updater")
    };

    show_window(&window);
}

pub fn close_window(window: &Window) {
    let _ = window.close();
}
