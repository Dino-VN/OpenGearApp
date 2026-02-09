//! MOD007B-specific packet definitions
//! Only defines packets - does NOT send HID directly

/// Packet definitions for Akko MOD007B
pub struct Mod007bPackets;

impl Mod007bPackets {
    /// Handshake packet (64 bytes)
    /// Used to initiate communication with the keyboard
    pub const HANDSHAKE: [u8; 64] = [
        143, 0, 0, 0, 0, 0, 0, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ];

    /// Get handshake packet
    pub fn handshake() -> [u8; 64] {
        Self::HANDSHAKE
    }
}
