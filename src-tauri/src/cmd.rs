use crate::config;
use crate::event::ClientEvent;
use crate::searcher;
use crate::window;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchResult {
    pub id: String,
    pub value: String,
    pub indices: Vec<usize>,
    pub score: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Shortcut {
    modifier: String, // TODO: Enum this
    key: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Field {
    pub name: String,
    pub value: String,
    pub shortcut: Shortcut,
}

#[tauri::command]
pub fn search(
    id: &str,
    term: &str,
    config: tauri::State<config::Config>,
    search_state: tauri::State<searcher::DataState>,
) -> Vec<SearchResult> {
    if id != "" && term == "" {
        return Vec::new();
    }

    // TODO: This could probably be an array with a fixed length
    let mut search_results: Vec<SearchResult> = Vec::new();

    let matcher = SkimMatcherV2::default();
    if id == "" {
        for service in &config.search_services {
            match matcher.fuzzy_indices(&service.name, &term) {
                Some((score, indices)) => {
                    let result = SearchResult {
                        id: service.name.to_string(),
                        value: service.name.to_string(),
                        indices: indices,
                        score: score,
                    };
                    search_results.push(result);
                }
                None => {}
            };
        }
    } else {
        println!("Not implemented");
        println!("Searching {} Service", id);
        for service in &config.search_services {
            if id == service.name {
                search_results = searcher::search_service(term, service, &search_state);
            }
        }
    }
    search_results
}

#[tauri::command]
pub fn get_list() -> Vec<String> {
    let mut vec = Vec::new();
    vec.push("what".to_string());
    vec
}

#[tauri::command]
pub fn open_service(app: AppHandle, window: tauri::Window, service: &str) {
    println!("Lets Open {}", service);
    let _ = window.emit(ClientEvent::ClearSearch.as_ref(), true);
    let _ = window.emit(ClientEvent::SetService.as_ref(), service.to_string());
    let _ = window.emit(ClientEvent::FocusSearch.as_ref(), true);
}

#[tauri::command]
pub fn get_info(id: &str) -> Vec<Field> {
    println!("Get {}", id);
    let mut vec = Vec::new();
    vec.push(Field {
        name: "Field Name".to_string(),
        value: "Field Value".to_string(),
        shortcut: Shortcut {
            modifier: "cmd".to_string(),
            key: "1".to_string(),
        },
    });

    return vec;
}

#[tauri::command]
pub fn open_info(app: AppHandle, window: tauri::Window, id: &str) {
    let _ = window.emit(ClientEvent::ClearSearch.as_ref(), true);
    println!("Open ID: {:?}", id);
    window::hide_search_bar(&window);
    window::show_info_window(&app, id);
}

#[tauri::command]
pub fn get_config() -> config::Config {
    return config::Config::new();
}

#[tauri::command]
pub fn resize_window(window: tauri::Window, height: f64) {
    window::resize_window(&window, height);
}

#[tauri::command]
pub fn close_search(window: tauri::Window) {
    window::hide_search_bar(&window);
}

#[tauri::command]
pub fn close_window(window: tauri::Window) {
    window::close_window(&window);
}
