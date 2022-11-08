use anyhow;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{fs, path::PathBuf};

// TODO: Automagically reload
#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub user_settings: UserSettings,
    pub search_services: Vec<SearchServiceConfig>,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            user_settings: UserSettings::default(),
            search_services: Vec::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserSettings {
    pub entries_to_show: u16,
    pub fzf_algorithm: String,
    pub similarity: f32,
    pub shortcut: String,
}
impl Default for UserSettings {
    fn default() -> Self {
        UserSettings {
            entries_to_show: 5,
            fzf_algorithm: "jaro-winkler".to_string(),
            similarity: 0.7,
            shortcut: "CmdOrCtrl+Shift+/".to_string(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SearchServiceConfig {
    pub name: String,
    pub shortcut: String,
    pub algorithm: String,
    pub similarity: f32,
    pub file_settings: FileSettings,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FileSettings {
    pub source_file: String,
    pub file_type: String,
    pub sheet: String,
    pub rows_to_skip: i32,
    pub fields: Vec<FieldConfig>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FieldConfig {
    pub name: String,
    pub search: bool,
    pub display_name: String,
    pub display: bool,
    pub shortcut: String,
    pub qr_template: String,
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

        config
    }

    pub fn load_config() -> anyhow::Result<Config> {
        let prefs_path = Self::prefs_file();

        match prefs_path.exists() {
            true => {
                println!("Preference file exists at: {:?}", prefs_path);
                let mut settings: Config =
                    serde_yaml::from_str(&fs::read_to_string(prefs_path).unwrap())?;
                // Do any checks on config here
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
        return Self::prefs_dir().join("settings.yaml");
    }
}
