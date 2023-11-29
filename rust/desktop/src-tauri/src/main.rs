// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
use anyhow::Result;
use app_core::audio::{get_audio_output_devices, play_audio_data};
use app_core::slack::{extract_slack_ids, fetch_slack_messages, get_new_message};
use app_core::{get_audio_data_from_voicevox, AUDIO_ENDPOINT, SYNTHESIZE_ENDPOINT};
use cpal::traits::DeviceTrait;
use error::AppError;
use reqwest;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::sync::Mutex;
use std::{thread, time::Duration};
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
fn device_list() -> Result<Vec<String>, AppError> {
    let devices = get_audio_output_devices()?;
    let device_names = devices
        .iter()
        .map(|device| device.name().unwrap())
        .collect();
    Ok(device_names)
}

#[command]
async fn run_voice_reader(device: String, state: State<'_, AppState>) -> Result<(), AppError> {
    println!("Starting voice reader");
    let settings = load_settings()?;
    let (channel_id, thread_ts) = extract_slack_ids(&settings.thread_url)?;
    let synthesize_endpoint = format!("{}{}", settings.voicevox_url, SYNTHESIZE_ENDPOINT);
    let audio_endpoint = format!("{}{}", settings.voicevox_url, AUDIO_ENDPOINT);
    let client = reqwest::Client::new();
    {
        let mut is_running = state.is_running.lock().unwrap();
        *is_running = true;
    }

    let mut latest_timestamp = thread_ts.clone();
    loop {
        match fetch_slack_messages(&client, &settings.slack_token, &channel_id, &thread_ts).await {
            Ok(messages) => {
                if let Some((ts, text)) = get_new_message(&messages, &latest_timestamp) {
                    latest_timestamp = ts;
                    println!("New message: {}", text);
                    match get_audio_data_from_voicevox(
                        &client,
                        &text,
                        &settings.speaker_style_id,
                        &synthesize_endpoint,
                        &audio_endpoint,
                    )
                    .await
                    {
                        Ok(audio_data) => {
                            play_audio_data(None, &device, &audio_data)
                                .await
                                .map_err(|e| {
                                    println!("Failed to play audio data: {}", e);
                                    e
                                })?;
                        }
                        Err(e) => {
                            println!("Failed to get audio data: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("Failed to fetch messages: {}", e);
            }
        }
        thread::sleep(Duration::from_secs(1));
        let is_running = state.is_running.lock().unwrap();
        if !*is_running {
            break;
        }
    }
    Ok(())
}

#[command]
fn stop_voice_reader(state: State<'_, AppState>) -> Result<(), AppError> {
    println!("Stopping voice reader");
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
            device_list,
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
