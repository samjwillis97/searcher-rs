use std::collections::HashMap;
use std::sync::Mutex;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use crate::cmd;
use crate::config;
use crate::config::FileSettings;
use calamine::{open_workbook, DataType, Reader, Xlsx};

#[derive(Debug)]
pub struct InnerData {
    pub id: String,
    pub data: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug)]
pub struct DataState(pub Mutex<InnerData>);

pub fn search_service(
    term: &str,
    service: &config::SearchServiceConfig,
    search_state: &tauri::State<DataState>,
) -> Vec<cmd::SearchResult> {
    get_service_data(&service, search_state);
    return search_data(term, search_state);
}

fn search_data(term: &str, search_state: &tauri::State<DataState>) -> Vec<cmd::SearchResult> {
    let mut search_results: Vec<cmd::SearchResult> = Vec::new();
    let matcher = SkimMatcherV2::default();

    let guarded_state = search_state.0.lock().unwrap();

    println!("total rows: {}", &guarded_state.data.len());
    for row in &guarded_state.data {
        match matcher.fuzzy_indices(row.0, term) {
            Some((score, indices)) => {
                // TODO: Check score here
                // TODO: Value may be different
                let result = cmd::SearchResult {
                    id: row.0.to_string(),
                    value: row.0.to_string(),
                    indices,
                    score,
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

    println!("total results: {}", search_results.len());

    return search_results;
}

fn xlsx_datatype_to_string(value: &DataType) -> String {
    match *value {
        DataType::String(ref s) => return s.to_string(),
        DataType::Int(ref s) => return s.to_string(),
        DataType::Float(ref s) => return s.to_string(),
        DataType::DateTime(ref s) => return s.to_string(),
        DataType::Bool(ref s) => return s.to_string(),
        _ => return "".to_string(),
    }
}

fn get_xlsx_columns(row: &[DataType]) -> Vec<(String, i32)> {
    let mut columns: Vec<(String, i32)> = Vec::new();
    row.into_iter().enumerate().for_each(|(i, v)| {
        columns.push((v.to_string(), i.try_into().unwrap()));
    });
    return columns;
}

fn parse_xlsx_file(service: &config::SearchServiceConfig, search_state: &tauri::State<DataState>) {
    let mut excel: Xlsx<_> = open_workbook(service.file_settings.source_file.to_string()).unwrap();
    let sheet_name = &service
        .file_settings
        .sheet
        .as_ref()
        .unwrap_or(&FileSettings::default().sheet.unwrap())
        .to_string();

    if let Some(Ok(r)) = excel.worksheet_range(&sheet_name) {
        let mut search_keys: Vec<String> = Vec::new();
        for field in &service.file_settings.fields {
            if field.search.unwrap_or(false) {
                search_keys.push(field.name.to_string());
            }
        }

        let mut field_locations: Vec<(String, i32)> = Vec::new();
        let mut row_data_map: HashMap<String, HashMap<String, String>> = HashMap::new();

        r.rows()
            .skip(
                service
                    .file_settings
                    .rows_to_skip
                    .unwrap_or(FileSettings::default().rows_to_skip.unwrap())
                    .try_into()
                    .unwrap(),
            )
            .enumerate()
            .for_each(|(i, row)| {
                if i == 0 {
                    field_locations = get_xlsx_columns(row);
                    return;
                }

                let mut row_map: HashMap<String, String> = HashMap::new();
                let mut search_key: String = "".to_owned();

                field_locations.to_owned().into_iter().for_each(|field| {
                    let value = xlsx_datatype_to_string(&row[field.1 as usize]);
                    if search_keys.contains(&field.0) {
                        search_key.push_str(&value);
                    }
                    row_map.insert(field.0.to_string(), value);
                });

                if search_key != "" {
                    row_data_map.insert(search_key, row_map);
                }
            });

        let data = InnerData {
            id: service.name.to_string(),
            data: row_data_map,
        };
        let mut search_state_guard = search_state.0.lock().unwrap();
        *search_state_guard = data;
    }
}

fn get_service_data(service: &config::SearchServiceConfig, search_state: &tauri::State<DataState>) {
    if !std::path::Path::new(&service.file_settings.source_file).exists() {
        // TODO: Send an error event to the UI
        println!("File does not exist {:}", service.file_settings.source_file);
        return;
    }

    // TODO: Clear State on change of service
    let state_guard = search_state.0.lock().unwrap();
    if state_guard.data.len() == 0 {
        drop(state_guard);
        match service.file_settings.file_type.as_str() {
            "xlsx" => {
                println!("Parsing XLSX File");
                parse_xlsx_file(service, search_state);
            }
            _ => println!("Do not know file type"),
        };
    }
}
