use crate::api::fansly::{AccountMetadata, StreamMetadata, StreamResponse};
use crate::config::Config;
use anyhow::Result;
use chrono::Utc;
use std::path::PathBuf;
use std::process::Command;
use tokio::time::{sleep, Duration};

pub async fn start_recording(
    account_data: &AccountMetadata,
    stream_data: &StreamMetadata,
    config: &Config,
) -> Result<()> {
    if let Some(stream_response) = &stream_data.response {
        let current_datetime = Utc::now().format("%Y%m%d_%H%M%S").to_string();

        let username = &account_data.response[0].username;
        let stream_id = &stream_response.stream.id;
        let filename = format!("{}_{}_v{}", username, current_datetime, stream_id);

        let ts_filename = ffmpeg_sync(&filename, stream_response, account_data, config).await?;
        let mp4_filename = convert_to_mp4(&ts_filename, config).await?;

        if config.general.mt {
            generate_contact_sheet(&mp4_filename).await?;
        }

        if config.general.ffmpeg_convert {
            std::fs::remove_file(ts_filename)?;
        }
        println!("[INFO] Stream complete. Resuming online check");
        sleep(Duration::from_secs(130)).await;
    }
    Ok(())
}

async fn ffmpeg_sync(
    filename: &str,
    stream_response: &StreamResponse,
    account_data: &AccountMetadata,
    config: &Config,
) -> Result<PathBuf> {
    let directory = config
        .general
        .save_path
        .join(&account_data.response[0].username);
    std::fs::create_dir_all(&directory)?;
    let ts_filename = directory.join(format!("{}.ts", filename));

    println!("[FFMPEG] Saving livestream to {}", ts_filename.display());

    let playback_url = stream_response.stream.playback_url.as_ref().unwrap();
    let output = Command::new("ffmpeg")
        .args(&[
            "-i",
            playback_url,
            "-c",
            "copy",
            "-movflags",
            "use_metadata_tags",
            "-map_metadata",
            "0",
            "-timeout",
            "300",
            "-reconnect",
            "300",
            "-reconnect_at_eof",
            "300",
            "-reconnect_streamed",
            "300",
            "-reconnect_delay_max",
            "300",
            "-rtmp_live",
            "live",
            ts_filename.to_str().unwrap(),
        ])
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "ffmpeg command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    println!(
        "[FFMPEG] Done saving livestream to {}",
        ts_filename.display()
    );
    Ok(ts_filename)
}

async fn convert_to_mp4(ts_filename: &PathBuf, config: &Config) -> Result<PathBuf> {
    let mp4_filename = ts_filename.with_extension("mp4");

    if config.general.ffmpeg_convert {
        let output = Command::new("ffmpeg")
            .args(&[
                "-i",
                ts_filename.to_str().unwrap(),
                "-c:v",
                "copy",
                "-c:a",
                "copy",
                mp4_filename.to_str().unwrap(),
            ])
            .output()?;

        if !output.status.success() {
            anyhow::bail!(
                "ffmpeg conversion failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    } else {
        std::fs::rename(ts_filename, &mp4_filename)?;
    }

    Ok(mp4_filename)
}

async fn generate_contact_sheet(mp4_filename: &PathBuf) -> Result<PathBuf> {
    let contact_sheet_filename = mp4_filename.with_extension("jpg");

    let output = Command::new("./mt")
        .args(&[
            "--columns=4",
            "--numcaps=24",
            "--header-meta",
            "--fast",
            "--comment=Archive - Fansly VODs",
            &format!("--output={}", contact_sheet_filename.display()),
            mp4_filename.to_str().unwrap(),
        ])
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "mt command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(contact_sheet_filename)
}
