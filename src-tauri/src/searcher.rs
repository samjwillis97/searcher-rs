use std::collections::HashMap;
use std::sync::Mutex;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use crate::cmd;
use crate::config;
use calamine::{open_workbook, DataType, Reader, Xlsx};

#[derive(Debug)]
pub struct InnerData {
    pub data: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug)]
pub struct DataState(pub Mutex<InnerData>);

pub fn search_service(
    term: &str,
    service: &config::SearchServiceConfig,
    search_state: &tauri::State<DataState>,
) -> Vec<cmd::SearchResult> {
    println!("Searching {}", term);
    println!("{:#?}", service.name);
    get_service_data(&service.file_settings, search_state);
    return search_data(term, search_state);
}

fn search_data(term: &str, search_state: &tauri::State<DataState>) -> Vec<cmd::SearchResult> {
    let mut search_results: Vec<cmd::SearchResult> = Vec::new();
    let matcher = SkimMatcherV2::default();

    let guarded_state = search_state.0.lock().unwrap();

    for row in &guarded_state.data {
        match matcher.fuzzy_indices(row.0, term) {
            Some((score, indices)) => {
                // TODO: Check score here
                // TODO: Value may be different
                let result = cmd::SearchResult {
                    id: row.0.to_string(),
                    value: row.0.to_string(),
                    indices: indices,
                    score: score,
                };
                if search_results.len() > 0 {
                    if score >= search_results.first().unwrap().score {
                        search_results.insert(0, result)
                    } else if score <= search_results.last().unwrap().score {
                        search_results.push(result);
                    } else {
                        // TODO: Could change this to a more efficient insert
                        for (i, v) in search_results.iter().enumerate() {
                            if v.score < score {
                                search_results.insert(i, result);
                                break;
                            }
                        }
                    }
                } else {
                    search_results.push(result);
                }
            }
            None => {}
        };
    }

    return search_results;
}

fn parse_xlsx_file(service: &config::FileSettings, search_state: &tauri::State<DataState>) {
    // This function should
    // attempt to open a file
    // CSV etc.
    // create a map with the row and corresponding search value
    // TODO: THIS SHOULD BE IN MEMORY AND ONLY CALCULATED ON THE FIRST SEARCH
    let mut excel: Xlsx<_> = open_workbook(service.source_file.to_string()).unwrap();
    if let Some(Ok(r)) = excel.worksheet_range(&service.sheet.to_string()) {
        let mut search_keys: Vec<String> = Vec::new();
        for field in &service.fields {
            if field.search {
                search_keys.push(field.name.to_string());
            }
        }

        let mut field_locations: Vec<(String, i32)> = Vec::new();
        let mut row_data_map: HashMap<String, HashMap<String, String>> = HashMap::new();

        let mut i = 0;
        for row in r.rows() {
            if i == service.rows_to_skip {
                let mut col_index = 0;
                for value in row {
                    match *value {
                        DataType::String(ref s) => {
                            field_locations.push((s.to_string(), col_index));
                        }
                        _ => println!("else"),
                    }
                    col_index += 1;
                }
            }
            if i > service.rows_to_skip {
                let mut row_map: HashMap<String, String> = HashMap::new();
                let mut search_key: String = "".to_owned();

                for field in &field_locations {
                    let row_value = match row[field.1 as usize] {
                        DataType::String(ref s) => s.to_string(),
                        _ => "".to_string(),
                    };
                    if search_keys.contains(&field.0) {
                        search_key.push_str(&row_value);
                    }
                    row_map.insert(field.0.to_string(), row_value);
                }

                if search_key != "" {
                    row_data_map.insert(search_key, row_map);
                }
            }
            i += 1;
        }
        let data = InnerData { data: row_data_map };
        let mut search_state_guard = search_state.0.lock().unwrap();
        *search_state_guard = data;
    }
}

fn get_service_data(service: &config::FileSettings, search_state: &tauri::State<DataState>) {
    if !std::path::Path::new(&service.source_file).exists() {
        // TODO: Send an error event to the UI
        println!("File does not exist {:}", service.source_file);
        return;
    }

    // TODO: Clear State on change of service
    let state_guard = search_state.0.lock().unwrap();
    if state_guard.data.len() == 0 {
        drop(state_guard);
        match service.file_type.as_str() {
            "xlsx" => parse_xlsx_file(service, search_state),
            _ => println!("Do not know file type"),
        };
    }
}
