mod api;
mod config;
mod processing;
mod webhooks;

use crate::api::fansly::{get_account_data, get_stream_data};
use crate::config::{Config, WebhookConfig};
use crate::processing::recorder::start_recording;
use crate::webhooks::send_live_noti;
use anyhow::{Context, Result};
use clap::Parser;
use std::process::Command;
use tokio::time::{sleep, Duration};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    username: String,
}

fn check_ffmpeg() -> Result<()> {
    let output = Command::new("ffmpeg")
        .arg("-version")
        .output()
        .context("[WARN] Check failed to run command\n")?;

    if output.status.success() {
        print!("[INFO] FFmpeg installed, continuing.\n");
        Ok(())
    } else {
        anyhow::bail!(
            "[ERROR] FFmpeg is not installed or not in PATH. Please install FFmpeg and try again."
        )
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    check_ffmpeg()?;

    let cli = Cli::parse();

    let config = Config::load_or_create()?;

    let account_data = get_account_data(&cli.username, &config).await?;
    //println!("Account data: {:?}", account_data);

    let stream_url = format!(
        "https://apiv3.fansly.com/api/v1/streaming/channel/{}?ngsw-bypass=true",
        account_data.response[0].id
    );

    println!("[INFO] Starting online check for {}", cli.username);

    let webhook_config = WebhookConfig {
        enabled: config.webhook.enabled,
        live_webhook: config.webhook.live_webhook.clone(),
        info_webhook: config.webhook.info_webhook.clone(),
        webhook_mention: config.webhook.webhook_mention.clone(),
    };

    loop {
        let stream_data = get_stream_data(&stream_url, &config).await?;

        if stream_data.success && stream_data.response.is_some() {
            let stream_response = stream_data.response.as_ref().unwrap();
            if stream_response.stream.access {
                if config.webhook.enabled {
                    send_live_noti(
                        &webhook_config,
                        &account_data.response[0].username,
                        &account_data.response[0].avatar.location,
                    )
                    .await?;
                }
                println!(
                    "[INFO] {} Stream is online, starting archiver",
                    cli.username
                );
                start_recording(&account_data, &stream_data, &config).await?;
            }
        } else {
            println!("[INFO] {} is offline, checking again in 130s", cli.username);
            sleep(Duration::from_secs(130)).await;
        }
    }
    //Ok(())
}
