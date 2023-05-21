use anyhow;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{fs, path::PathBuf};

// TODO: Automagically reload
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub user_settings: Option<UserSettings>,
    pub search_services: Vec<SearchServiceConfig>,
    pub app_settings: Option<AppSettings>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            user_settings: Some(UserSettings::default()),
            search_services: Vec::new(),
            app_settings: Some(AppSettings::default()),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AppSettings {
    pub escape_closes_info: Option<bool>,
    pub escape_closes_service_search: Option<bool>,
    pub position: Option<String>,
}
impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            escape_closes_info: Some(false),
            escape_closes_service_search: Some(true),
            position: Some("center".to_string()),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserSettings {
    pub entries_to_show: Option<u16>,
    pub fzf_algorithm: Option<String>,
    pub similarity: Option<f32>,
    pub shortcut: Option<String>,
}
impl Default for UserSettings {
    fn default() -> Self {
        UserSettings {
            entries_to_show: Some(5),
            fzf_algorithm: Some("jaro-winkler".to_string()),
            similarity: Some(0.7),
            shortcut: Some("CmdOrCtrl+Shift+/".to_string()),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SearchServiceConfig {
    pub name: String,
    pub shortcut: Option<String>,
    pub algorithm: Option<String>,
    pub similarity: Option<f32>,
    pub file_settings: FileSettings,
}
impl Default for SearchServiceConfig {
    fn default() -> Self {
        SearchServiceConfig {
            name: "Default Service".to_string(),
            shortcut: None,
            algorithm: None,
            similarity: Some(0.3),
            file_settings: FileSettings::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FileSettings {
    pub source_file: String,
    pub file_type: String,
    pub sheet: Option<String>,
    pub rows_to_skip: Option<i32>,
    pub fields: Vec<FieldConfig>,
}
impl Default for FileSettings {
    fn default() -> Self {
        FileSettings {
            source_file: "data.xlsx".to_string(),
            file_type: "xlsx".to_string(),
            sheet: Some("Sheet 1".to_string()),
            rows_to_skip: Some(0),
            fields: Vec::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FieldConfig {
    pub name: String,
    pub search: Option<bool>,
    pub display_name: Option<String>,
    pub display: Option<bool>,
    pub shortcut: Option<String>,
    pub qr_template: Option<String>,
}
impl Default for FieldConfig {
    fn default() -> Self {
        FieldConfig {
            name: "".to_string(),
            search: Some(false),
            display_name: None,
            display: Some(false),
            shortcut: None,
            qr_template: None,
        }
    }
}

// TODO: Load Services
impl Config {
    pub fn new() -> Self {
        let prefs_dir = Self::prefs_dir();
        fs::create_dir_all(&prefs_dir).expect("Unable to create preferences directory.");

        let config = Self::load_config().unwrap_or_else(|err| {
            println!("Invalid user settings file!: Reason {}", err);
            Default::default()
        });
        // Handle errors here

        return config;
    }

    pub fn load_config() -> anyhow::Result<Config> {
        let prefs_path = Self::prefs_file();

        match prefs_path.exists() {
            true => {
                println!("Preference file exists at: {:?}", prefs_path);
                let mut settings: Config =
                    serde_yaml::from_str(&fs::read_to_string(prefs_path).unwrap())?;
                // Do any checks on config here
                if (settings.app_settings.is_none()) {
                    settings.app_settings = Some(AppSettings::default());
                }
                Ok(settings)
            }
            _ => {
                let settings = Config::default();
                fs::write(prefs_path, serde_yaml::to_string(&settings).unwrap())
                    .expect("Unable to save User Configuration.");
                Ok(settings)
            }
        }
    }

    pub fn prefs_dir() -> PathBuf {
        let prefs_dir = ProjectDirs::from("com", "williscloud", "tif_search").unwrap();
        return prefs_dir.preference_dir().to_path_buf();
    }

    pub fn prefs_file() -> PathBuf {
        let mut path = PathBuf::new();
        path.push("/Users");
        path.push("samwillis");
        path.push("projects");
        path.push("searcher-rs");
        path.push("testing");
        path.push("settings");
        path.set_extension("yaml");
        println!("Preference path: {:?}", path);
        return path;
        // return Self::prefs_dir().join("settings.yaml");
    }
}
