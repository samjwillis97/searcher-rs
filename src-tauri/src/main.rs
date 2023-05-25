#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{collections::HashMap, sync::Mutex};

use auto_launch::AutoLaunchBuilder;

#[cfg(target_os = "macos")]
use cocoa;
#[cfg(target_os = "macos")]
use cocoa::appkit::NSWindow;

use tauri::{GlobalShortcutManager, Manager, RunEvent};
use tokio::sync::broadcast;

use crate::searcher::{DataState, InnerData};

mod cmd;
mod config;
mod constants;
mod event;
mod searcher;
mod window;

#[derive(Clone)]
pub struct AppShutdown;

// TODO: Automatic updates
// TODO: Tray Menu
// TODO: Handle Escape
// TODO: Handle Clearing
fn main() {
    let ctx = tauri::generate_context!();
    let config = config::Config::new();

    // Configure Application to run on boot
    if let Ok(path) = std::env::current_exe() {
        if let Some(path) = path.to_str() {
            if let Ok(auto) = AutoLaunchBuilder::new()
                .set_app_name("com.williscloud.tif-search")
                .set_app_path(path)
                .set_use_launch_agent(true)
                .build()
            {
                if let Err(e) = auto.enable() {
                    println!("Unable to add tif_search to startup items: {}", e);
                }
            }
        }
    }

    let app = tauri::Builder::default()
        // .plugin(tauri_plugin_log::Builder::default().targets([
        //     LogTarget::LogDir,
        //     LogTarget::Stdout,
        //     LogTarget::Webview,
        // ]).build())
        .invoke_handler(
            tauri::generate_handler![
                cmd::search,
                cmd::get_list,
                cmd::get_config,
                cmd::resize_window,
                cmd::resize_info_window,
                cmd::close_search,
                cmd::open_service,
                cmd::open_previous_service,
                cmd::open_info,
                cmd::get_info,
                cmd::close_window,
            ]
        )
        .setup(move |app| {
            let app_handle = app.app_handle();
            // TODO: Show startup
            // TODO: Investigate event driven

            let (shutdown_tx, _) = broadcast::channel::<AppShutdown>(1);
            app.manage(shutdown_tx);

            let config = config::Config::new();
            print!("Loading prefs from: {:?}", config::Config::prefs_dir());

            // macOS: hide from dock (also hides menu bar)
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let window = app
                .get_window(constants::SEARCH_WIN_NAME)
                .expect("Main window not found");
            // TODO: Center Window
            let _ = window.hide();

            // macOS: Handle multiple spaces correctly
            #[cfg(target_os = "macos")]
            {
                unsafe {
                    let ns_window = window.ns_window().expect("Unable to get ns_window") as cocoa::base::id;
                    ns_window.setCollectionBehavior_(cocoa::appkit::NSWindowCollectionBehavior::NSWindowCollectionBehaviorMoveToActiveSpace);
                }
            }

            let config_clone = config.clone();

            // Load user settings
            app.manage(config_clone.clone());
            // Set Initial value for state
            app.manage(DataState( Mutex::new(InnerData{ id: "".to_string(), search_data: Vec::new(), lookup_data: HashMap::new()}) ));

            // Register global shortcut
            let user_settings = config.user_settings.unwrap_or(config::UserSettings::default()).clone();
            let shortcut = user_settings.shortcut.unwrap_or(config::UserSettings::default().shortcut.unwrap()).clone();
            let window_clone = window.clone();
            let mut shortcuts = window.app_handle().global_shortcut_manager();
            match shortcuts.is_registered(&shortcut) {
                Ok(is_registered) => {
                    if !is_registered
                    {
                        println!("Registering {} as shortcut", &shortcut);
                        if let Err(e) = shortcuts
                            .register(&shortcut, move || {
                                let window = window_clone.clone();
                                let config = &config_clone.clone();
                                let _ = window.is_visible()
                                    .map(|is_visible| {
                                        if is_visible {
                                            window::hide_search_bar(&window);
                                        } else {
                                            window::show_search_bar(&window, config);
                                        }
                                    });
                            }) {
                            window::alert(&window, "Error registering global shortcut", &format!("{}", e));
                        }
                    }
                }
                Err(e) => window::alert(&window_clone, "Error registering global shortcut", &format!("{}", e))
            }

            Ok(())
        })
        .build(ctx)
        .expect("error while running tauri application");

    app.run(|app_handle, e| match e {
        RunEvent::ExitRequested { .. } => {
            // Do some cleanup for long running tasks
            let shutdown_tx = app_handle.state::<broadcast::Sender<AppShutdown>>();
            let _ = shutdown_tx.send(AppShutdown);
        }
        RunEvent::Exit { .. } => {
            println!("😔 bye bye");
        }
        _ => {}
    });
}
