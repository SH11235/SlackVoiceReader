// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::fs::File;
use tauri::command;
use tauri::Error;

#[derive(Serialize, Deserialize)]
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
