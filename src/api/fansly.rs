use crate::config::Config;
use anyhow::{bail, Context, Result};
use reqwest::header::HeaderMap;
use serde_json::Value;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AccountMetadata {
    pub success: bool,
    pub response: Vec<AccountResponse>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AccountResponse {
    pub id: String,
    pub username: String,
    pub avatar: Avatar,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Avatar {
    pub id: String,
    pub mimetype: String,
    pub location: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StreamMetadata {
    pub success: bool,
    pub response: Option<StreamResponse>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StreamResponse {
    pub id: String,
    pub account_id: String,
    pub playback_url: String,
    pub stream: StreamInfo,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StreamInfo {
    pub id: String,
    pub title: String,
    pub status: String,
    pub last_fetched_at: i64,
    pub started_at: i64,
    pub access: bool,
    pub playback_url: Option<String>,
}

pub async fn get_account_data(username: &str, config: &Config) -> Result<AccountMetadata> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://apiv3.fansly.com/api/v1/account?usernames={}&ngsw-bypass=true",
        username
    );

    let headers: HeaderMap = (&config.headers)
        .try_into()
        .context("Failed to convert headers")?;

    let response = client
        .get(&url)
        .headers(headers)
        .send()
        .await
        .context("Failed to send request")?;

    let json_data: Value = response
        .json()
        .await
        .context("Failed to parse JSON resposne")?;

    if !json_data["success"].as_bool().unwrap_or(false)
        || json_data["response"]
            .as_array()
            .map_or(true, |arr| arr.is_empty())
    {
        bail!("Error: could not retrieve account data");
    }

    let metadata = AccountMetadata {
        success: json_data["success"].as_bool().unwrap_or(false),
        response: vec![AccountResponse {
            id: json_data["response"][0]["id"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            username: json_data["response"][0]["username"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            avatar: Avatar {
                id: json_data["response"][0]["avatar"]["id"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                mimetype: json_data["response"][0]["avatar"]["mimetype"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                location: json_data["response"][0]["avatar"]["location"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
            },
        }],
    };

    Ok(metadata)
}

pub async fn get_stream_data(stream_url: &str, config: &Config) -> Result<StreamMetadata> {
    let client = reqwest::Client::new();
    let headers: HeaderMap = (&config.headers)
        .try_into()
        .context("Failed to convert headers")?;

    let response = client
        .get(stream_url)
        .headers(headers)
        .send()
        .await
        .context("Failed to send request")?;

    let data: Value = response
        .json()
        .await
        .context("Failed to parse JSON response")?;

    let access = data["response"]["stream"]["access"]
        .as_bool()
        .unwrap_or(false);
    let status = data["response"]["stream"]["status"].as_i64().unwrap_or(1);

    if !access || status != 2 {
        return Ok(StreamMetadata {
            success: false,
            response: None,
        });
    }

    let metadata = StreamMetadata {
        success: data["success"].as_bool().unwrap_or(false),
        response: Some(StreamResponse {
            id: data["response"]["id"].as_str().unwrap_or("").to_string(),
            account_id: data["response"]["accountId"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            playback_url: data["response"]["playbackUrl"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            stream: StreamInfo {
                id: data["response"]["stream"]["id"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                title: data["response"]["stream"]["title"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                status: data["response"]["stream"]["status"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                last_fetched_at: data["response"]["stream"]["lastFetchedAt"]
                    .as_i64()
                    .unwrap_or(0),
                started_at: data["response"]["stream"]["startedAt"]
                    .as_i64()
                    .unwrap_or(0),
                access,
                playback_url: data["response"]["stream"]["playbackUrl"]
                    .as_str()
                    .map(String::from),
            },
        }),
    };

    Ok(metadata)
}
