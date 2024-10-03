use crate::config::WebhookConfig;
use anyhow::{Context, Result};
use serenity::all::Colour;
use serenity::builder::{CreateAttachment, CreateEmbed, ExecuteWebhook};
use serenity::{http::Http, model::webhook::Webhook};

pub async fn send_live_noti(
    config: &WebhookConfig,
    username: &str,
    avatar_url: &str,
) -> Result<()> {
    if !config.enabled {
        return Ok(());
    }

    let http = Http::new("");
    let webhook = Webhook::from_url(&http, &config.live_webhook)
        .await
        .context("Failed to create webhook")?;

    let live_url = format!("https://fansly.com/live/{}", username);

    let embed = CreateEmbed::default()
        .title(format!("{} is now Live!", username))
        .color(Colour::from_rgb(3, 178, 248))
        .url(live_url)
        //.author(|a: &mut CreateEmbed| a.name(username).icon_url(avatar_url))
        .thumbnail(avatar_url)
        .timestamp(chrono::Utc::now());

    let builder = ExecuteWebhook::default()
        .content(format!(
            "{} {} is now live on Fansly!",
            config.webhook_mention, username
        ))
        .username("Fansly Recorder RS")
        .embed(embed);

    webhook
        .execute(&http, false, builder)
        .await
        .context("Failed to execute webhook")?;

    println!("[INFO] Sent live notification for {}", username);
    Ok(())
}

#[allow(dead_code)]
pub async fn send_upload_notification(
    config: &WebhookConfig,
    mp4_name: &str,
    sheet_name: &str,
    sheet_path: &str,
) -> Result<()> {
    if !config.enabled {
        return Ok(());
    }

    let http = Http::new("");
    let webhook = Webhook::from_url(&http, &config.info_webhook)
        .await
        .context("Failed to create webhook")?;

    let embed = CreateEmbed::default()
        .title("Stream Recording Uploaded")
        .description(format!(
            "Uploaded {} with contact sheet {}",
            mp4_name, sheet_name
        ))
        .color(Colour::from_rgb(3, 178, 248))
        .image("attachment://contact_sheet.jpg")
        .timestamp(chrono::Utc::now());

    let file = CreateAttachment::path(sheet_path)
        .await
        .context("Failed to create attachment")?;

    let builder = ExecuteWebhook::default()
        .content(format!("{} Vod Uploaded", config.webhook_mention))
        .username("Fansly Recorder")
        .add_file(file)
        .embed(embed);

    webhook
        .execute(&http, false, builder)
        .await
        .context("Failed to execute webhook")?;

    println!("[INFO] Sent upload notification for {}", mp4_name);
    Ok(())
}
