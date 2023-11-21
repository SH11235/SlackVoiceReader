use anyhow::Error;
use reqwest;
use rodio::cpal::traits::{DeviceTrait, HostTrait};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use serde_json::Value;
use std::fs::File;
use std::io::{self, Cursor, Write};

pub async fn get_audio_data_from_voicevox(
    text: &str,
    speaker_style_id: &str,
    synthesize_endpoint: &str,
    audio_endpoint: &str,
) -> Result<Vec<u8>, Error> {
    let client = reqwest::Client::new();
    let audio_query = client
        .post(synthesize_endpoint)
        .query(&[("text", text), ("speaker", speaker_style_id)])
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let audio_data = client
        .post(audio_endpoint)
        .query(&[("speaker", speaker_style_id)])
        .json(&audio_query)
        .send()
        .await?
        .bytes()
        .await?;

    Ok(audio_data.to_vec())
}

pub fn get_user_device() -> Result<cpal::Device, anyhow::Error> {
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

pub async fn save_audio_data_to_file(
    audio_data: &[u8],
    file_path: &str,
) -> Result<(), std::io::Error> {
    let mut file = File::create(file_path)?;
    file.write_all(audio_data)?;
    Ok(())
}

pub async fn play_audio_data(
    device: &cpal::Device,
    audio_data: &[u8],
) -> Result<(), anyhow::Error> {
    let (_stream, stream_handle) = get_output_stream(&device.name()?)?;
    let sink = Sink::try_new(&stream_handle)?;
    let cursor = Cursor::new(audio_data.to_vec());
    let source = Decoder::new(cursor)?;
    sink.append(source);
    sink.sleep_until_end();
    Ok(())
}

pub fn get_output_stream(
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

pub async fn fetch_slack_messages(
    token: &str,
    channel_id: &str,
    thread_ts: &str,
) -> Result<Value, Error> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://slack.com/api/conversations.replies")
        .header("Authorization", format!("Bearer {}", token))
        .query(&[("channel", channel_id), ("ts", thread_ts)])
        .send()
        .await?;

    let messages: Value = res.json().await?;
    Ok(messages)
}

pub fn get_new_message(messages: &Value) -> Option<(String, String)> {
    let messages = messages["messages"].clone();
    messages.as_array().and_then(|msgs| msgs.last()).map(|msg| {
        (
            msg["ts"].as_str().unwrap_or_default().to_owned(),
            msg["text"].as_str().unwrap_or_default().to_owned(),
        )
    })
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
