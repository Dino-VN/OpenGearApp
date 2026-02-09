//! Akko HID Protocol Layer
//! Packet building, parsing, and opcode definitions
//!
//! PROTOCOL DISCOVERY (from Akko Cloud capture):
//! - Byte 0: Opcode
//! - Byte 1-2: Parameters
//! - Byte 3-6: Reserved
//! - Byte 7: Checksum = 0xFF - Opcode
//! - Byte 8-63: Payload

/// Packet size for all Akko commands
pub const PACKET_SIZE: usize = 64;

/// Calculate checksum for opcode (byte 7 = 0xFF - opcode)
#[inline]
pub fn calc_checksum(opcode: u8) -> u8 {
    0xFF_u8.wrapping_sub(opcode)
}

/// Known Akko opcodes (from real Akko Cloud capture)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AkkoOpcode {
    /// Handshake / Init (0x8F) - Response: [8F, fw_major, fw_minor, ...]
    Handshake = 0x8F,

    /// Get profile count (0xF0) - Response: [F0, count, active_profile, ...]
    GetProfileCount = 0xF0,

    /// Get device info (0x80) - Response: [80, type, status, ...]
    GetDeviceInfo = 0x80,

    /// Set RGB settings (0x07) - Command: [07, 04, brightness, speed, 07, R, G, B]
    SetRgbSettings = 0x07,

    /// Get RGB settings (0x87) - Response: [87, mode, speed, dir, 0, R, G, B]
    GetRgbSettings = 0x87,

    /// Get RGB mode (0x88) - Response: [88, mode, p1, p2, brightness, R, G, B]
    GetRgbMode = 0x88,

    /// Get performance settings (0x92) - Response: [92, debounce_dn, 0, debounce_up, ...]
    GetPerformance = 0x92,

    /// Get FN lock status (0x84) - Response: [84, 0, enabled, ...]
    GetFnLockStatus = 0x84,

    /// Get indicator LED settings (0x91) - Response: [91, 0, enabled, ...]
    GetIndicatorLed = 0x91,

    /// Get sleep settings (0x97)
    GetSleepSettings = 0x97,

    /// Get custom RGB (0x86)
    GetCustomRgb = 0x86,

    /// Get macro status (0xAE)
    GetMacroStatus = 0xAE,

    /// Get macro data (0xAD)
    GetMacroData = 0xAD,

    /// Get layout info (0x85)
    GetLayoutInfo = 0x85,

    /// Get battery status (0x9D)
    GetBatteryStatus = 0x9D,

    /// Unknown opcode for probing
    Unknown(u8),
}

impl From<u8> for AkkoOpcode {
    fn from(value: u8) -> Self {
        match value {
            0x07 => AkkoOpcode::SetRgbSettings,
            0x8F => AkkoOpcode::Handshake,
            0xF0 => AkkoOpcode::GetProfileCount,
            0x80 => AkkoOpcode::GetDeviceInfo,
            0x87 => AkkoOpcode::GetRgbSettings,
            0x88 => AkkoOpcode::GetRgbMode,
            0x92 => AkkoOpcode::GetPerformance,
            0x84 => AkkoOpcode::GetFnLockStatus,
            0x91 => AkkoOpcode::GetIndicatorLed,
            0x97 => AkkoOpcode::GetSleepSettings,
            0x86 => AkkoOpcode::GetCustomRgb,
            0xAE => AkkoOpcode::GetMacroStatus,
            0xAD => AkkoOpcode::GetMacroData,
            0x85 => AkkoOpcode::GetLayoutInfo,
            0x9D => AkkoOpcode::GetBatteryStatus,
            other => AkkoOpcode::Unknown(other),
        }
    }
}

impl From<AkkoOpcode> for u8 {
    fn from(opcode: AkkoOpcode) -> Self {
        match opcode {
            AkkoOpcode::SetRgbSettings => 0x07,
            AkkoOpcode::Handshake => 0x8F,
            AkkoOpcode::GetProfileCount => 0xF0,
            AkkoOpcode::GetDeviceInfo => 0x80,
            AkkoOpcode::GetRgbSettings => 0x87,
            AkkoOpcode::GetRgbMode => 0x88,
            AkkoOpcode::GetPerformance => 0x92,
            AkkoOpcode::GetFnLockStatus => 0x84,
            AkkoOpcode::GetIndicatorLed => 0x91,
            AkkoOpcode::GetSleepSettings => 0x97,
            AkkoOpcode::GetCustomRgb => 0x86,
            AkkoOpcode::GetMacroStatus => 0xAE,
            AkkoOpcode::GetMacroData => 0xAD,
            AkkoOpcode::GetLayoutInfo => 0x85,
            AkkoOpcode::GetBatteryStatus => 0x9D,
            AkkoOpcode::Unknown(v) => v,
        }
    }
}

impl AkkoOpcode {
    /// Get command name for logging
    pub fn name(&self) -> &'static str {
        match self {
            AkkoOpcode::SetRgbSettings => "SetRgbSettings",
            AkkoOpcode::Handshake => "Handshake",
            AkkoOpcode::GetProfileCount => "GetProfileCount",
            AkkoOpcode::GetDeviceInfo => "GetDeviceInfo",
            AkkoOpcode::GetRgbSettings => "GetRgbSettings",
            AkkoOpcode::GetRgbMode => "GetRgbMode",
            AkkoOpcode::GetPerformance => "GetPerformance",
            AkkoOpcode::GetFnLockStatus => "GetFnLockStatus",
            AkkoOpcode::GetIndicatorLed => "GetIndicatorLed",
            AkkoOpcode::GetSleepSettings => "GetSleepSettings",
            AkkoOpcode::GetCustomRgb => "GetCustomRgb",
            AkkoOpcode::GetMacroStatus => "GetMacroStatus",
            AkkoOpcode::GetMacroData => "GetMacroData",
            AkkoOpcode::GetLayoutInfo => "GetLayoutInfo",
            AkkoOpcode::GetBatteryStatus => "GetBatteryStatus",
            AkkoOpcode::Unknown(_) => "Unknown",
        }
    }
}

/// Akko HID Packet builder and parser
#[derive(Debug, Clone)]
pub struct AkkoPacket {
    data: [u8; PACKET_SIZE],
}

impl AkkoPacket {
    /// Create empty packet
    pub fn new() -> Self {
        Self {
            data: [0u8; PACKET_SIZE],
        }
    }

    /// Create packet with opcode and auto-calculated checksum
    pub fn with_opcode(opcode: AkkoOpcode) -> Self {
        let mut packet = Self::new();
        let op_byte: u8 = opcode.into();
        packet.data[0] = op_byte;
        packet.data[7] = calc_checksum(op_byte);
        packet
    }

    /// Create packet with opcode and parameters
    pub fn with_opcode_params(opcode: AkkoOpcode, param1: u8, param2: u8) -> Self {
        let mut packet = Self::with_opcode(opcode);
        packet.data[1] = param1;
        packet.data[2] = param2;
        packet
    }

    /// Create packet from raw bytes
    pub fn from_bytes(data: &[u8]) -> Self {
        let mut packet = Self::new();
        let len = data.len().min(PACKET_SIZE);
        packet.data[..len].copy_from_slice(&data[..len]);
        packet
    }

    /// Get opcode from packet
    pub fn opcode(&self) -> AkkoOpcode {
        self.data[0].into()
    }

    /// Get parameters (bytes 1-2)
    pub fn params(&self) -> (u8, u8) {
        (self.data[1], self.data[2])
    }

    /// Get checksum byte
    pub fn checksum(&self) -> u8 {
        self.data[7]
    }

    /// Verify checksum is valid
    pub fn is_checksum_valid(&self) -> bool {
        self.data[7] == calc_checksum(self.data[0])
    }

    /// Get payload slice (bytes 8-63)
    pub fn payload(&self) -> &[u8] {
        &self.data[8..]
    }

    /// Set payload data
    pub fn set_payload(&mut self, payload: &[u8]) {
        let len = payload.len().min(PACKET_SIZE - 8);
        self.data[8..8 + len].copy_from_slice(&payload[..len]);
    }

    /// Get raw bytes
    pub fn as_bytes(&self) -> &[u8; PACKET_SIZE] {
        &self.data
    }

    /// Get mutable raw bytes
    pub fn as_bytes_mut(&mut self) -> &mut [u8; PACKET_SIZE] {
        &mut self.data
    }

    /// Check if response has data (non-zero after opcode)
    pub fn has_data(&self) -> bool {
        self.data[1] != 0
            || self.data[2] != 0
            || self.data[3] != 0
            || self.data[4] != 0
            || self.data[5] != 0
            || self.data[6] != 0
    }

    /// Format packet as hex string
    pub fn to_hex_string(&self) -> String {
        self.data
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Format first 16 bytes as compact hex
    pub fn to_hex_short(&self) -> String {
        self.data[..16]
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl Default for AkkoPacket {
    fn default() -> Self {
        Self::new()
    }
}

/// Parsed RGB settings from GetRgbSettings response
#[derive(Debug, Clone)]
pub struct RgbSettings {
    pub mode: u8,
    pub speed: u8,
    pub direction: u8,
    pub color: (u8, u8, u8),
}

impl RgbSettings {
    pub fn from_response(data: &[u8]) -> Option<Self> {
        if data.len() < 8 || data[0] != 0x87 {
            return None;
        }
        Some(Self {
            mode: data[1],
            speed: data[2],
            direction: data[3],
            color: (data[5], data[6], data[7]),
        })
    }
}

/// Parsed performance settings from GetPerformance response
#[derive(Debug, Clone)]
pub struct PerformanceSettings {
    pub debounce_down: u8,
    pub debounce_up: u8,
}

impl PerformanceSettings {
    pub fn from_response(data: &[u8]) -> Option<Self> {
        if data.len() < 4 || data[0] != 0x92 {
            return None;
        }
        Some(Self {
            debounce_down: data[1],
            debounce_up: data[3],
        })
    }
}

/// Parsed profile info from GetProfileCount response
#[derive(Debug, Clone)]
pub struct ProfileInfo {
    pub count: u8,
    pub active: u8,
}

impl ProfileInfo {
    pub fn from_response(data: &[u8]) -> Option<Self> {
        if data.len() < 3 || data[0] != 0xF0 {
            return None;
        }
        Some(Self {
            count: data[1],
            active: data[2],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum() {
        assert_eq!(calc_checksum(0x8F), 0x70);
        assert_eq!(calc_checksum(0xF0), 0x0F);
        assert_eq!(calc_checksum(0x80), 0x7F);
        assert_eq!(calc_checksum(0x87), 0x78);
    }

    #[test]
    fn test_packet_builder() {
        let packet = AkkoPacket::with_opcode(AkkoOpcode::Handshake);
        assert_eq!(packet.as_bytes()[0], 0x8F);
        assert_eq!(packet.as_bytes()[7], 0x70);
        assert!(packet.is_checksum_valid());
    }
}
