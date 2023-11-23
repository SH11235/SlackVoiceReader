use anyhow::anyhow;
use anyhow::{Error, Result};
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Client;
use serde_derive::Deserialize;

lazy_static! {
    static ref SLACK_URL_RE: Regex =
        Regex::new(r"https://.+\.slack\.com/archives/(C[A-Z0-9]+)/p(\d{10})(\d{6})").unwrap();
}

pub fn extract_slack_ids(url: &str) -> Result<(String, String)> {
    if let Some(caps) = SLACK_URL_RE.captures(url) {
        let channel_id = caps
            .get(1)
            .ok_or_else(|| anyhow!("Failed to extract Channel ID from {}", url))?
            .as_str()
            .to_string();
        let timestamp_part1 = caps
            .get(2)
            .ok_or_else(|| anyhow!("Failed to extract timestamp from {}", url))?
            .as_str();
        let timestamp_part2 = caps
            .get(3)
            .ok_or_else(|| anyhow!("Failed to extract timestamp fraction from {}", url))?
            .as_str();

        let thread_ts = format!("{}.{}", timestamp_part1, timestamp_part2);
        Ok((channel_id, thread_ts))
    } else {
        Err(anyhow!("Invalid URL format: {}", url))
    }
}

#[derive(Debug, Deserialize)]
pub struct SlackMessage {
    // client_msg_id: String,
    // #[serde(rename = "type")]
    // msg_type: String,
    pub text: String,
    // user: String,
    pub ts: String,
    // blocks: Vec<serde_json::Value>,
    // team: String,
    // thread_ts: String,
    // parent_user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct SlackResponse {
    pub ok: bool,
    pub messages: Vec<SlackMessage>,
    // pub has_more: bool, // コメント数が多い場合に対応する場合はこれを使う
}

pub async fn fetch_slack_messages(
    client: &Client,
    token: &str,
    channel_id: &str,
    thread_ts: &str,
) -> Result<SlackResponse, Error> {
    let res = client
        .get("https://slack.com/api/conversations.replies")
        .header("Authorization", format!("Bearer {}", token))
        .query(&[("channel", channel_id), ("ts", thread_ts)])
        .send()
        .await?;

    let messages: SlackResponse = res.json().await?;
    Ok(messages)
}

pub fn get_new_message(messages: &SlackResponse, timestamp: &str) -> Option<(String, String)> {
    messages.messages.iter().last().and_then(|message| {
        if message.ts != timestamp {
            Some((message.ts.clone(), message.text.clone()))
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_get_new_message() {
        let messages = SlackResponse {
            ok: true,
            messages: vec![SlackMessage {
                ts: "1234567890.123457".to_string(),
                text: "Hello world".to_string(),
            }],
        };
        let result = get_new_message(&messages, "1234567890.123456");
        assert_eq!(
            result,
            Some(("1234567890.123457".to_string(), "Hello world".to_string()))
        );
    }

    #[test]
    fn test_get_new_message_none() {
        let messages = SlackResponse {
            ok: true,
            messages: vec![SlackMessage {
                ts: "1234567890.123456".to_string(),
                text: "Hello world".to_string(),
            }],
        };
        let result = get_new_message(&messages, "1234567890.123456");
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_slack_ids_valid_url() -> Result<()> {
        let url = "https://workspace.slack.com/archives/C12345678/p1234567890123456";
        let expected = ("C12345678".to_string(), "1234567890.123456".to_string());

        let result = extract_slack_ids(url)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_extract_slack_ids_invalid_url() {
        let url = "https://invalid.url";
        let result = extract_slack_ids(url);
        assert!(result.is_err());
    }
}
