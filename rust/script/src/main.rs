use core::slack::{extract_slack_ids, fetch_slack_messages, get_new_message};
use core::{
    get_audio_data_from_voicevox, get_output_stream, get_user_device, play_audio_data,
    save_audio_data_to_file,
};
use cpal::traits::DeviceTrait;
use dotenv::dotenv;
use log::{error, info};
use reqwest;
use std::{env, thread, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv().ok();
    let slack_token = env::var("SLACK_TOKEN").expect("SLACK_TOKEN is not set in .env file");
    let thread_url = env::var("THREAD_URL").expect("THREAD_URL is not set in .env file");
    let (channel_id, thread_ts) =
        extract_slack_ids(&thread_url).expect("Failed to extract IDs from THREAD_URL");
    let voicevox_url = env::var("VOICEVOX_URL").expect("VOICEVOX_URL is not set in .env file");
    let synthesize_endpoint = format!("{}/audio_query", voicevox_url);
    let audio_endpoint = format!("{}/synthesis", voicevox_url);
    let device = get_user_device()?;
    let (_stream, stream_handle) = get_output_stream(&device.name()?)?;
    let speaker_style_id =
        env::var("SPEAKER_STYLE_ID").expect("SPEAKER_STYLE_ID is not set in .env file");
    let client = reqwest::Client::new();

    let mut latest_timestamp = thread_ts.clone();

    loop {
        match fetch_slack_messages(&client, &slack_token, &channel_id, &thread_ts).await {
            Ok(messages) => {
                if let Some((ts, text)) = get_new_message(&messages, &latest_timestamp) {
                    latest_timestamp = ts;

                    info!("New message: {}", text);

                    match get_audio_data_from_voicevox(
                        &client,
                        &text,
                        &speaker_style_id,
                        &synthesize_endpoint,
                        &audio_endpoint,
                    )
                    .await
                    {
                        Ok(audio_data) => {
                            // デバッグモードでのみ音声データをファイルに保存
                            #[cfg(debug_assertions)]
                            save_audio_data_to_file(&audio_data, "output.wav").await?;

                            play_audio_data(&stream_handle, &audio_data).await?;
                        }
                        Err(e) => {
                            error!("Failed to get audio data: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                error!("Failed to fetch messages: {}", e);
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
}
