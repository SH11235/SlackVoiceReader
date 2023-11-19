use anyhow;
use dotenv::dotenv;
use reqwest;
use rodio::cpal::traits::{DeviceTrait, HostTrait};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::{self, Cursor, Write};
use std::{thread, time};

fn get_user_device() -> Result<cpal::Device, anyhow::Error> {
    // cpalのホストを取得
    let host = cpal::default_host();

    // 利用可能な出力デバイスを表示
    let output_devices = host.output_devices()?;
    println!("Available Output Devices:");
    for (index, device) in output_devices.enumerate() {
        let device_name = device.name()?;
        println!("{}: {}", index, device_name);
    }

    // ユーザーにデバイスを選択させる
    println!("Enter the number of the device you want to use:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let device_id: usize = input.trim().parse()?;

    // 選択されたデバイスを取得
    let device = host
        .output_devices()?
        .nth(device_id)
        .ok_or_else(|| anyhow::anyhow!("Invalid device ID"))?;
    println!("Selected device: {}", device.name()?);
    Ok(device)
}

// デバッグ用
async fn save_audio_data_to_file(audio_data: &[u8], file_path: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(file_path)?;
    file.write_all(audio_data)?;
    Ok(())
}

async fn get_audio_data_from_voicevox(
    text: &str,
    synthesize_endpoint: &str,
    audio_endpoint: &str,
) -> Result<Vec<u8>, anyhow::Error> {
    let client = reqwest::Client::new();
    let audio_query = client
        .post(synthesize_endpoint)
        .query(&[("text", text), ("speaker", "1")])
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let audio_data = client
        .post(audio_endpoint)
        .query(&[("speaker", "1")])
        .json(&audio_query)
        .send()
        .await?
        .bytes()
        .await?;

    Ok(audio_data.to_vec())
}

fn get_new_message(messages: &Value) -> Option<(String, String)> {
    let messages = messages["messages"].clone();
    messages.as_array().and_then(|msgs| msgs.last()).map(|msg| {
        (
            msg["ts"].as_str().unwrap_or_default().to_owned(),
            msg["text"].as_str().unwrap_or_default().to_owned(),
        )
    })
}

async fn play_audio_data(device: &cpal::Device, audio_data: &[u8]) -> Result<(), anyhow::Error> {
    let (_stream, stream_handle) = get_output_stream(&device.name()?)?;
    let sink = Sink::try_new(&stream_handle)?;
    let cursor = Cursor::new(audio_data.to_vec());
    let source = Decoder::new(cursor)?;
    sink.append(source);
    sink.sleep_until_end();
    Ok(())
}

fn get_output_stream(
    device_name: &str,
) -> Result<(OutputStream, OutputStreamHandle), rodio::StreamError> {
    let host = cpal::default_host();
    let devices = host.output_devices().unwrap();
    for device in devices {
        let dev: rodio::Device = device.into();
        if let Ok(dev_name) = dev.name() {
            if dev_name == device_name {
                println!("Device found: {}", dev_name);
                return OutputStream::try_from_device(&dev);
            }
        }
    }
    OutputStream::try_default()
}

async fn process_message_and_play_sound(
    text: &str,
    device: &cpal::Device,
    synthesize_endpoint: &str,
    audio_endpoint: &str,
) -> Result<(), anyhow::Error> {
    println!("Synthesizing: {}", text);
    let start_time = std::time::Instant::now();
    let audio_data =
        get_audio_data_from_voicevox(text, synthesize_endpoint, audio_endpoint).await?;
    let end_time = start_time.elapsed(); // 時間計測終了
    println!("VoiceVox API request time: {:?}", end_time);

    // デバッグモードでのみ音声データをファイルに保存
    #[cfg(debug_assertions)]
    save_audio_data_to_file(&audio_data, "output.wav").await?;
    play_audio_data(device, &audio_data).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Slack API 設定
    let slack_token = env::var("SLACK_TOKEN").expect("SLACK_TOKEN is not set in .env file");
    let channel_id = env::var("CHANNEL_ID").expect("CHANNEL_ID is not set in .env file");
    let thread_ts = env::var("THREAD_TS").expect("THREAD_TS is not set in .env file");
    let mut latest_timestamp = env::var("THREAD_TS")
        .expect("THREAD_TS is not set in .env file")
        .to_string();

    // VoiceVox APIのエンドポイント設定
    let voicevox_url = env::var("VOICEVOX_URL").expect("VOICEVOX_URL is not set in .env file");
    let synthesize_endpoint = format!("{}/audio_query", voicevox_url);
    let audio_endpoint = format!("{}/synthesis", voicevox_url);

    let device = get_user_device()?;

    loop {
        let start_time = std::time::Instant::now();

        // Slack API を使って最新のメッセージを取得
        let client = reqwest::Client::new();
        let res = client
            .get("https://slack.com/api/conversations.replies")
            .header("Authorization", format!("Bearer {}", slack_token))
            .query(&[("channel", &channel_id), ("ts", &thread_ts)])
            .send()
            .await?;

        let end_time = start_time.elapsed();
        println!("Slack API request time: {:?}", end_time);

        let messages: Value = res.json().await?;
        // meesagesは以下の形式
        // {
        //     "has_more": false,
        //     "messages": [...],
        //     "ok": true
        // }
        if let Some((ts, text)) = get_new_message(&messages) {
            if ts != latest_timestamp {
                // 最新のメッセージを更新
                latest_timestamp = ts;

                // メッセージのテキストを取得
                println!("New message: {}", text);

                // メッセージを処理して音声を再生
                process_message_and_play_sound(
                    &text,
                    &device,
                    &synthesize_endpoint,
                    &audio_endpoint,
                )
                .await?;
            }
        } else {
            println!("No new messages");
        }

        // 一定時間待機
        println!("Waiting...");
        thread::sleep(time::Duration::from_secs(1));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_new_message() {
        let messages = json!({
            "messages": [
                {"ts": "12345", "text": "Hello world"}
            ]
        });
        let result = get_new_message(&messages);
        assert_eq!(
            result,
            Some(("12345".to_string(), "Hello world".to_string()))
        );
    }
}
