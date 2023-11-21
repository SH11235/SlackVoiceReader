use core::{
    fetch_slack_messages, get_audio_data_from_voicevox, get_new_message, get_user_device,
    play_audio_data, save_audio_data_to_file,
};
use dotenv::dotenv;
use log::{error, info};
use std::{env, thread, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let slack_token = env::var("SLACK_TOKEN").expect("SLACK_TOKEN is not set in .env file");
    let channel_id = env::var("CHANNEL_ID").expect("CHANNEL_ID is not set in .env file");
    let thread_ts = env::var("THREAD_TS").expect("THREAD_TS is not set in .env file");
    let voicevox_url = env::var("VOICEVOX_URL").expect("VOICEVOX_URL is not set in .env file");
    let synthesize_endpoint = format!("{}/audio_query", voicevox_url);
    let audio_endpoint = format!("{}/synthesis", voicevox_url);
    let device = get_user_device()?;
    let speaker_style_id =
        env::var("SPEAKER_STYLE_ID").expect("SPEAKER_STYLE_ID is not set in .env file");

    let mut latest_timestamp = thread_ts.clone();

    loop {
        match fetch_slack_messages(&slack_token, &channel_id, &thread_ts).await {
            Ok(messages) => {
                if let Some((ts, text)) = get_new_message(&messages) {
                    if ts != latest_timestamp {
                        latest_timestamp = ts;

                        info!("New message: {}", text);

                        match get_audio_data_from_voicevox(
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

                                play_audio_data(&device, &audio_data).await?;
                            }
                            Err(e) => {
                                error!("Failed to get audio data: {}", e);
                            }
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
