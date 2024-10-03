use crate::config::WebhookConfig;
use anyhow::Result;
use webhook::client::WebhookClient;
//use Chrono::Utc;

pub async fn send_live_noti(
    config: &WebhookConfig,
    username: &str,
    avatar_url: &str,
) -> Result<()> {
    if !config.enabled {
        return Ok(());
    }

    let client = WebhookClient::new(&config.live_webhook);
    let live_url = format!("https://fansly.com/live/{}", username);

    let _ = client
        .send(|message| {
            message
                .content(&format!(
                    "{} {} is now live on Fansly!",
                    config.webhook_mention, username
                ))
                .username("Fansly Recorder RS")
                .embed(|embed| {
                    embed
                        .title("Stream Live!")
                        .color("0x03b2f8")
                        .url(&live_url)
                        .author(username, Some(avatar_url.to_string()), None)
                        .thumbnail(avatar_url)
                        .timestamp(&chrono::Utc::now().to_rfc3339())
                })
        })
        .await;

    println!("[INFO] Sent live notification for {}", username);
    Ok(())
}

#[allow(dead_code)]
pub async fn send_upload_notification(
    config: &WebhookConfig,
    mp4_name: &str,
    sheet_name: &str,
    _sheet_path: &str,
) -> Result<()> {
    if !config.enabled {
        return Ok(());
    }

    let client = WebhookClient::new(&config.info_webhook);

    let _ = client
        .send(|message| {
            message
                .content(&format!("{} Vod Uploaded", config.webhook_mention))
                .username("Fansly Recorder")
                .embed(|embed| {
                    embed
                        .title("Stream Recording Uploaded")
                        .description(&format!(
                            "Uploaded {} with contact sheet {}",
                            mp4_name, sheet_name
                        ))
                        .color("0x03b2f8")
                        .image("attachment://contact_sheet.jpg")
                        .timestamp(&chrono::Utc::now().to_rfc3339())
                })
            //.file("contact_sheet.jpg", sheet_path)
        })
        .await;

    println!("[info] Sent upload notification for {}", mp4_name);
    Ok(())
}
