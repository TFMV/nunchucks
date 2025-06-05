use anyhow::anyhow;
use reqwest::Client;
use rodio::{Decoder, OutputStream, Sink};
use std::path::Path;
use std::process::Command;
use tracing::info;

pub struct ActionHandler {
    client: Client,
}

impl ActionHandler {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn execute_command(&self, command: &str) -> crate::Result<()> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .map_err(|e| anyhow!("Command failed: {}", e))?;

        if !output.status.success() {
            return Err(anyhow!(
                "Command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        info!("Command executed successfully");
        Ok(())
    }

    pub async fn trigger_webhook(&self, url: &str) -> crate::Result<()> {
        self.client
            .post(url)
            .send()
            .await
            .map_err(|e| anyhow!("Webhook failed: {}", e))?;

        info!("Webhook triggered successfully");
        Ok(())
    }

    pub async fn play_sound(&self, path: &Path) -> crate::Result<()> {
        let (_stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| anyhow!("Failed to create audio stream: {}", e))?;

        let file =
            std::fs::File::open(path).map_err(|e| anyhow!("Failed to open sound file: {}", e))?;

        let sink = Sink::try_new(&stream_handle)
            .map_err(|e| anyhow!("Failed to create audio sink: {}", e))?;

        let source =
            Decoder::new(file).map_err(|e| anyhow!("Failed to decode sound file: {}", e))?;

        sink.append(source);
        sink.sleep_until_end();

        info!("Sound played successfully");
        Ok(())
    }

    pub async fn show_ascii(&self, path: &Path) -> crate::Result<()> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| anyhow!("Failed to read ASCII file: {}", e))?;

        println!("\n{}", content);
        info!("ASCII art displayed successfully");
        Ok(())
    }
}
