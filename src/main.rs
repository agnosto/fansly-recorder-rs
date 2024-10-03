mod api;
mod config;
mod processing;

use crate::api::fansly::{get_account_data, get_stream_data};
use crate::config::Config;
use crate::processing::recorder::start_recording;
use anyhow::Result;
use clap::Parser;
use tokio::time::{sleep, Duration};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    username: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = Config::load_or_create()?;

    let account_data = get_account_data(&cli.username, &config).await?;
    //println!("Account data: {:?}", account_data);

    let stream_url = format!(
        "https://apiv3.fansly.com/api/v1/streaming/channel/{}?ngsw-bypass=true",
        account_data.response[0].id
    );

    println!("[INFO] Starting online check for {}", cli.username);

    loop {
        let stream_data = get_stream_data(&stream_url, &config).await?;

        if stream_data.success && stream_data.response.is_some() {
            let stream_response = stream_data.response.as_ref().unwrap();
            if stream_response.stream.access {
                if config.webhook.enabled {
                    // Implement sending notification for user going live
                    //todo!()
                    println!("wokring on adding webhooks, please be patient :)")
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
