use core::{
    fetch_slack_messages, get_audio_data_from_voicevox, get_new_message, get_user_device,
    play_audio_data, save_audio_data_to_file,
};
use dotenv::dotenv;
use std::{env, thread, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let slack_token = env::var("SLACK_TOKEN")?;
    let channel_id = env::var("CHANNEL_ID")?;
    let thread_ts = env::var("THREAD_TS")?;
    let voicevox_url = env::var("VOICEVOX_URL")?;
    let synthesize_endpoint = format!("{}/audio_query", voicevox_url);
    let audio_endpoint = format!("{}/synthesis", voicevox_url);
    let device = get_user_device()?;
    let speaker_style_id = env::var("SPEAKER_STYLE_ID")?;

    let mut latest_timestamp = thread_ts;

    loop {
        let messages = fetch_slack_messages(&slack_token, &channel_id, &latest_timestamp).await?;

        if let Some((ts, text)) = get_new_message(&messages) {
            if ts != latest_timestamp {
                latest_timestamp = ts;

                let audio_data = get_audio_data_from_voicevox(
                    &text,
                    &speaker_style_id,
                    &synthesize_endpoint,
                    &audio_endpoint,
                )
                .await?;

                // デバッグモードでのみ音声データをファイルに保存
                #[cfg(debug_assertions)]
                save_audio_data_to_file(&audio_data, "output.wav").await?;

                play_audio_data(&device, &audio_data).await?;
            }
        }

        thread::sleep(Duration::from_secs(1));
    }
}
