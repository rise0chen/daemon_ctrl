use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Contral {
    pub reboot: bool,
}

impl Contral {
    pub fn read(file: &str) -> Self {
        if let Ok(ctrl) = fs::read(file) {
            if let Ok(ctrl) = serde_json::from_slice(&ctrl) {
                return ctrl;
            }
        }
        Self { reboot: false }
    }
    pub fn save(&self, file: &str) {
        let ctrl = serde_json::to_string(self).unwrap();
        fs::write(file, ctrl).unwrap();
    }
}
