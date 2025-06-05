use device_query::{DeviceQuery, DeviceState, Keycode};
use std::collections::HashSet;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub struct HotkeyManager {
    tx: Sender<Vec<String>>,
    rx: Receiver<Vec<String>>,
}

impl HotkeyManager {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self { tx, rx }
    }

    pub fn start_listener(&self) -> crate::Result<()> {
        let tx = self.tx.clone();

        thread::spawn(move || {
            let device_state = DeviceState::new();
            let mut last_keys: HashSet<Keycode> = HashSet::new();

            loop {
                let keys: HashSet<Keycode> = device_state.get_keys().into_iter().collect();

                if keys != last_keys {
                    let key_names: Vec<String> = keys.iter().map(|k| format!("{:?}", k)).collect();

                    if !key_names.is_empty() {
                        if let Err(e) = tx.send(key_names) {
                            eprintln!("Error sending key combination: {}", e);
                            break;
                        }
                    }
                }

                last_keys = keys;
                thread::sleep(std::time::Duration::from_millis(10));
            }
        });

        Ok(())
    }

    pub fn get_key_combination(&self) -> Option<Vec<String>> {
        self.rx.try_recv().ok()
    }
}
