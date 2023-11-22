pub mod audio;
pub mod slack;

use anyhow::{Error, Result};
use reqwest;

pub async fn get_audio_data_from_voicevox(
    client: &reqwest::Client,
    text: &str,
    speaker_style_id: &str,
    synthesize_endpoint: &str,
    audio_endpoint: &str,
) -> Result<Vec<u8>, Error> {
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
