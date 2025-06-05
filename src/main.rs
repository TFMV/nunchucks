mod actions;
mod config;
mod hotkeys;
mod notification;

use anyhow::{anyhow, Result};
use std::process;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration first to get log level
    let config = config::Config::load("~/.nunchucks.toml")?;

    // Initialize logging with configured level
    let log_level = match config.settings.log_level.as_str() {
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    let _subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .with_thread_names(false)
        .with_ansi(true)
        .pretty()
        .init();

    info!("🥷 Nunchucks starting up...");

    // Handle daemon mode
    if config.settings.daemon_mode {
        let pid = process::id();
        info!("Running in daemon mode (PID: {})", pid);
    }

    // Initialize components
    let notification_manager =
        notification::NotificationManager::new(config.settings.notification_timeout);
    let hotkey_manager = hotkeys::HotkeyManager::new();
    let action_handler = actions::ActionHandler::new();

    // Start hotkey listener
    hotkey_manager.start_listener()?;
    info!("✨ Nunchucks is ready to strike!");

    // Main event loop
    loop {
        if let Some(keys) = hotkey_manager.get_key_combination() {
            let key_combo = keys.join("+");
            info!("Detected key combination: {}", key_combo);

            // Find matching chuck
            for (name, chuck) in &config.chucks {
                if chuck.keys.iter().any(|k| k == &key_combo) {
                    info!("Executing chuck: {}", name);

                    // Show notification if configured
                    if let Some(notif) = &chuck.notification {
                        notification_manager.show("Nunchucks", notif)?;
                    }

                    // Execute command if configured with timeout and retries
                    if let Some(cmd) = &chuck.command {
                        for attempt in 1..=chuck.retry_count {
                            match tokio::time::timeout(
                                std::time::Duration::from_millis(chuck.timeout as u64),
                                action_handler.execute_command(cmd),
                            )
                            .await
                            {
                                Ok(result) => {
                                    if result.is_ok() {
                                        break;
                                    } else if attempt == chuck.retry_count {
                                        return Err(anyhow!(
                                            "Command failed after {} attempts",
                                            chuck.retry_count
                                        ));
                                    }
                                }
                                Err(_) => {
                                    if attempt == chuck.retry_count {
                                        return Err(anyhow!(
                                            "Command timed out after {} attempts",
                                            chuck.retry_count
                                        ));
                                    }
                                }
                            }
                        }
                    }

                    // Trigger webhook if configured
                    if let Some(webhook) = &chuck.webhook {
                        for attempt in 1..=chuck.retry_count {
                            match tokio::time::timeout(
                                std::time::Duration::from_millis(chuck.timeout as u64),
                                action_handler.trigger_webhook(webhook),
                            )
                            .await
                            {
                                Ok(result) => {
                                    if result.is_ok() {
                                        break;
                                    } else if attempt == chuck.retry_count {
                                        return Err(anyhow!(
                                            "Webhook failed after {} attempts",
                                            chuck.retry_count
                                        ));
                                    }
                                }
                                Err(_) => {
                                    if attempt == chuck.retry_count {
                                        return Err(anyhow!(
                                            "Webhook timed out after {} attempts",
                                            chuck.retry_count
                                        ));
                                    }
                                }
                            }
                        }
                    }

                    // Play sound if configured
                    if let Some(sound) = &chuck.sound {
                        action_handler.play_sound(sound).await?;
                    }

                    // Show ASCII if configured
                    if let Some(ascii) = &chuck.ascii {
                        action_handler.show_ascii(ascii).await?;
                    }
                }
            }
        }

        // Small sleep to prevent CPU spinning
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }
}
