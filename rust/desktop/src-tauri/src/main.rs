// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
use anyhow::Result;
use app_core::audio::{get_output_stream, get_user_device};
use app_core::slack::extract_slack_ids;
use app_core::{AUDIO_ENDPOINT, SYNTHESIZE_ENDPOINT};
use cpal::traits::DeviceTrait;
use error::AppError;
use reqwest;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::sync::Mutex;
use tauri::{command, Event, State};

struct AppState {
    is_running: Mutex<bool>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct Settings {
    slack_token: String,
    thread_url: String,
    voicevox_url: String,
    speaker_style_id: String,
}

#[command]
fn load_settings() -> Result<Settings, AppError> {
    let file = File::open("settings.json")?;
    let settings: Settings = serde_json::from_reader(file)?;
    Ok(settings)
}

#[command]
fn save_settings(settings: Settings) -> Result<(), AppError> {
    let file = File::create("settings.json")?;
    serde_json::to_writer(file, &settings)?;
    Ok(())
}

#[command]
fn run_voice_reader(state: State<'_, AppState>) -> Result<(), AppError> {
    let settings = load_settings()?;
    let (channel_id, thread_ts) = extract_slack_ids(&settings.thread_url)?;
    let synthesize_endpoint = format!("{}{}", settings.voicevox_url, SYNTHESIZE_ENDPOINT);
    let audio_endpoint = format!("{}{}", settings.voicevox_url, AUDIO_ENDPOINT);
    let device = get_user_device()?;
    let (_stream, stream_handle) = get_output_stream(&device.name()?)?;
    let client = reqwest::Client::new();
    // AppState.is_runningをtrueにする
    let mut is_running = state.is_running.lock().unwrap();
    *is_running = true;
    todo!();
    Ok(())
}

#[command]
fn stop_voice_reader(state: State<'_, AppState>) -> Result<(), AppError> {
    let mut is_running = state.is_running.lock().unwrap();
    *is_running = false;
    Ok(())
}

fn main() {
    let initial_state = AppState {
        is_running: Mutex::new(false),
    };
    tauri::Builder::default()
        .manage(initial_state)
        .invoke_handler(tauri::generate_handler![
            load_settings,
            save_settings,
            run_voice_reader,
            stop_voice_reader
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_and_load_settings() -> Result<(), AppError> {
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
