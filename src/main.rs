mod config;

use crate::config::Config;
use anyhow::{Context, Result};
use clap::Parser;
use reqwest::header::HeaderMap;
use serde_json::Value;
//use serde::Deserialize;
//use std::path::PathBuf;
use tokio;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    username: String,
}

async fn get_account_data(username: &str, config: &Config) -> Result<serde_json::Value> {
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

    Ok(json_data)
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = Config::load_or_create()?;

    let account_data = get_account_data(&cli.username, &config).await?;
    println!("Account data: {:?}", account_data);

    Ok(())
}
