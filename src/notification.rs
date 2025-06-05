use anyhow::{anyhow, Result};
use notify_rust::Notification;
use std::time::Duration;

pub struct NotificationManager {
    timeout: Duration,
}

impl NotificationManager {
    pub fn new(timeout_ms: u32) -> Self {
        Self {
            timeout: Duration::from_millis(timeout_ms as u64),
        }
    }

    pub fn show(&self, title: &str, body: &str) -> Result<()> {
        Notification::new()
            .summary(title)
            .body(body)
            .timeout(self.timeout)
            .show()
            .map_err(|e| anyhow!("Notification failed: {}", e))?;

        Ok(())
    }
}
