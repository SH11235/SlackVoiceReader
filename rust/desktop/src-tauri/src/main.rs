// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::fs::File;
use tauri::command;
use tauri::Error;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct Settings {
    slack_token: String,
    thread_url: String,
    voicevox_url: String,
    speaker_style_id: String,
}

#[command]
fn load_settings() -> Result<Settings, Error> {
    let file = File::open("settings.json")?;
    let settings: Settings = serde_json::from_reader(file)?;
    Ok(settings)
}

#[command]
fn save_settings(settings: Settings) -> Result<(), Error> {
    let file = File::create("settings.json")?;
    serde_json::to_writer(file, &settings)?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_settings, save_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_and_load_settings() -> Result<(), Error> {
        // 現状のファイルを読み込んでおく
        let file = File::open("settings.json")?;
        let dev_settings: Settings = serde_json::from_reader(file)?;

        let settings = Settings {
            slack_token: "test_slack_token".to_string(),
            thread_url: "test_thread_url".to_string(),
            voicevox_url: "test_voicevox_url".to_string(),
            speaker_style_id: "test_speaker_style_id".to_string(),
        };

        let save_result = save_settings(settings.clone());
        assert!(save_result.is_ok(), "Failed to save settings");

        let load_result = load_settings();
        assert!(load_result.is_ok(), "Failed to load settings");

        let loaded_settings = load_result.unwrap();
        assert_eq!(
            loaded_settings, settings,
            "Loaded settings do not match saved settings"
        );

        // setting.json を元に戻す
        let restore_result = save_settings(dev_settings);
        assert!(restore_result.is_ok(), "Failed to restore settings");

        Ok(())
    }
}
