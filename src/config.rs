use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub general: GeneralConfig,
    pub webhook: WebhookConfig,
    pub headers: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GeneralConfig {
    pub mt: bool,
    pub ffmpeg_convert: bool,
    pub save_path: PathBuf,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WebhookConfig {
    pub enabled: bool,
    pub live_webhook: String,
    pub info_webhook: String,
    pub webhook_mention: String,
}

impl Config {
    pub fn load_or_create() -> Result<Self> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let config_str =
                fs::read_to_string(&config_path).context("Failed to read config file")?;
            toml::from_str(&config_str).context("Failed to parse config file")
        } else {
            let config = Self::default();
            fs::create_dir_all(config_path.parent().unwrap())
                .context("Failed to create config directory")?;
            let toml_string =
                toml::to_string(&config).context("Failed to serialize default config")?;
            fs::write(&config_path, toml_string).context("Failed to write default config file")?;
            println!("Created default config file at {}", config_path.display());
            Ok(config)
        }
    }

    fn config_path() -> Result<PathBuf> {
        let mut path = dirs::config_dir().context("Could not find config directory")?;
        path.push("fansly-recorder");
        path.push("config.toml");
        Ok(path)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            general: GeneralConfig {
                mt: true,
                ffmpeg_convert: true,
                save_path: PathBuf::from("./captures"),
            },
            webhook: WebhookConfig {
                enabled: true,
                live_webhook: "https://discord.com/api/webhooks/1234567890/abcde".to_string(),
                info_webhook: "https://discord.com/api/webhooks/1234567890/abcde".to_string(),
                webhook_mention: "<@!123456789>".to_string(),
            },
            headers: [
                ("authority".to_string(), "apiv3.fansly.com".to_string()),
                ("accept".to_string(), "application/json, text/plain, */*".to_string()),
                ("accept-language".to_string(), "en;q=0.8,en-US;q=0.7".to_string()),
                ("authorization".to_string(), "your_auth_token_here".to_string()),
                ("origin".to_string(), "https://fansly.com".to_string()),
                ("referer".to_string(), "https://fansly.com/".to_string()),
                ("sec-ch-ua".to_string(), "\"Not.A/Brand\";v=\"8\", \"Chromium\";v=\"114\", \"Google Chrome\";v=\"114\"".to_string()),
                ("sec-ch-ua-mobile".to_string(), "?0".to_string()),
                ("sec-ch-ua-platform".to_string(), "\"Windows\"".to_string()),
                ("sec-fetch-dest".to_string(), "empty".to_string()),
                ("sec-fetch-mode".to_string(), "cors".to_string()),
                ("sec-fetch-site".to_string(), "same-site".to_string()),
                ("user-agent".to_string(), "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36".to_string()),
            ].iter().cloned().collect(),
        }
    }
}
