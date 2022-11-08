#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{collections::HashMap, sync::Mutex};

use auto_launch::AutoLaunchBuilder;
use cocoa;
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
                .set_app_name("com.williscloud.tif_search")
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
        .invoke_handler(
            tauri::generate_handler![
                cmd::search,
                cmd::get_list,
                cmd::get_config,
                cmd::resize_window,
                cmd::close_search,
                cmd::open_service,
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

            // Load user settings
            app.manage(config.clone());
            // Set Initial value for state
            app.manage(DataState( Mutex::new(InnerData{ data: HashMap::new()}) ));

            // Register global shortcut
            let window_clone = window.clone();
            let mut shortcuts = window.app_handle().global_shortcut_manager();
            match shortcuts.is_registered(&config.user_settings.shortcut) {
                Ok(is_registered) => {
                    if !is_registered
                    {
                        println!("Registering {} as shortcut", &config.user_settings.shortcut);
                        if let Err(e) = shortcuts
                            .register(&config.user_settings.shortcut, move || {
                                let window = window_clone.clone();
                                let _ = window.is_visible()
                                    .map(|is_visible| {
                                        if is_visible {
                                            window::hide_search_bar(&window);
                                        } else {
                                            window::show_search_bar(&window);
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
