//! Device detector for Akko keyboards
//! Maps models to their VID/PID values

use serde::{Deserialize, Serialize};

/// Supported Akko keyboard models
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AkkoModel {
    Mod007b,
    Akko24GWireless,
    // Add new models here
}

impl AkkoModel {
    /// Get VID for this model
    pub fn vid(&self) -> u16 {
        match self {
            AkkoModel::Mod007b => 0x3151,
            AkkoModel::Akko24GWireless => 0x3151,
        }
    }

    /// Get PID for this model
    pub fn pid(&self) -> u16 {
        match self {
            AkkoModel::Mod007b => 0x5009,
            AkkoModel::Akko24GWireless => 0x4011,
        }
    }

    /// Parse model from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "mod007b" => Some(AkkoModel::Mod007b),
            "akko24gwireless" | "akko 2.4g wireless keyboard" => Some(AkkoModel::Akko24GWireless),
            _ => None,
        }
    }

    /// Get model name as string
    pub fn name(&self) -> &'static str {
        match self {
            AkkoModel::Mod007b => "MOD007B",
            AkkoModel::Akko24GWireless => "Akko 2.4G Wireless Keyboard",
        }
    }
}

/// Detect connected Akko devices
/// Returns list of found models
pub fn detect_akko_devices() -> Vec<AkkoModel> {
    use hidapi::HidApi;

    let mut found = Vec::new();

    if let Ok(api) = HidApi::new() {
        let models = [AkkoModel::Mod007b, AkkoModel::Akko24GWireless];

        for model in models {
            for device in api.device_list() {
                if device.vendor_id() == model.vid() && device.product_id() == model.pid() {
                    found.push(model);
                    break;
                }
            }
        }
    }

    found
}
