//! High-level Akko commands
//! Based on real Akko Cloud protocol capture

use super::hid::AkkoHidDevice;
use super::protocol::{AkkoOpcode, AkkoPacket};
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};

/// Command result with parsed data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub success: bool,
    pub opcode: u8,
    pub opcode_name: String,
    pub response: Vec<u8>,
    pub hex_short: String,
}

impl CommandResult {
    pub fn from_response(opcode: AkkoOpcode, response: Vec<u8>) -> Self {
        let packet = AkkoPacket::from_bytes(&response);
        Self {
            success: packet.has_data() || response[0] == u8::from(opcode),
            opcode: opcode.into(),
            opcode_name: opcode.name().to_string(),
            hex_short: packet.to_hex_short(),
            response,
        }
    }
}

/// Probe result for opcode discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeResult {
    pub opcode: u8,
    pub opcode_name: String,
    pub responded: bool,
    pub has_data: bool,
    pub response_hex: String,
}

/// Execute a command with auto-checksum
pub fn execute_command(
    device: &AkkoHidDevice,
    opcode: AkkoOpcode,
) -> Result<CommandResult, String> {
    let packet = AkkoPacket::with_opcode(opcode);

    info!("Executing: {} (0x{:02X})", opcode.name(), u8::from(opcode));
    debug!("TX: {}", packet.to_hex_string());

    let response = device.send_feature_report(packet.as_bytes())?;

    Ok(CommandResult::from_response(opcode, response))
}

/// Execute command with parameters
pub fn execute_command_with_params(
    device: &AkkoHidDevice,
    opcode: AkkoOpcode,
    param1: u8,
    param2: u8,
) -> Result<CommandResult, String> {
    let packet = AkkoPacket::with_opcode_params(opcode, param1, param2);

    info!(
        "Executing: {} (0x{:02X}) params=({}, {})",
        opcode.name(),
        u8::from(opcode),
        param1,
        param2
    );

    let response = device.send_feature_report(packet.as_bytes())?;

    Ok(CommandResult::from_response(opcode, response))
}

// ============ High-Level Commands ============

/// Handshake with keyboard
pub fn cmd_handshake(device: &AkkoHidDevice) -> Result<CommandResult, String> {
    execute_command(device, AkkoOpcode::Handshake)
}

/// Get profile count and active profile
pub fn cmd_get_profile_count(device: &AkkoHidDevice) -> Result<CommandResult, String> {
    execute_command(device, AkkoOpcode::GetProfileCount)
}

/// Get device info
pub fn cmd_get_device_info(device: &AkkoHidDevice) -> Result<CommandResult, String> {
    execute_command(device, AkkoOpcode::GetDeviceInfo)
}

/// Get RGB settings (mode, speed, direction, color)
pub fn cmd_get_rgb_settings(device: &AkkoHidDevice) -> Result<CommandResult, String> {
    execute_command(device, AkkoOpcode::GetRgbSettings)
}

/// Get RGB mode details
pub fn cmd_get_rgb_mode(device: &AkkoHidDevice) -> Result<CommandResult, String> {
    execute_command(device, AkkoOpcode::GetRgbMode)
}

/// Get performance settings (debounce)
pub fn cmd_get_performance(device: &AkkoHidDevice) -> Result<CommandResult, String> {
    execute_command(device, AkkoOpcode::GetPerformance)
}

/// Get FN lock status
pub fn cmd_get_fn_lock(device: &AkkoHidDevice) -> Result<CommandResult, String> {
    execute_command(device, AkkoOpcode::GetFnLockStatus)
}

/// Get indicator LED status
pub fn cmd_get_indicator_led(device: &AkkoHidDevice) -> Result<CommandResult, String> {
    execute_command(device, AkkoOpcode::GetIndicatorLed)
}

/// Set RGB settings (brightness, speed, direction, and mode with color)
/// Based on web capture analysis:
/// - Byte 0: Opcode (0x07)
/// - Byte 1: Sub-command (0x04)
/// - Byte 2: Speed (INVERTED: UI 4 → protocol 1, UI 0 → protocol 5)
/// - Byte 3: Brightness (DIRECT: UI value = protocol value)
/// - Byte 4: Mode (0x07 = Dazzle, 0x08 = Static Color, etc.)
/// - Bytes 5-7: R, G, B
/// - Byte 8: Checksum
pub fn cmd_set_rgb_settings(
    device: &AkkoHidDevice,
    brightness: u8,
    speed: u8,
    direction: u8,
    color: (u8, u8, u8),
) -> Result<CommandResult, String> {
    cmd_set_rgb_settings_with_mode(device, brightness, speed, direction, color, 0x07)
    // Default: Dazzle
}

/// Set RGB settings with specific mode
pub fn cmd_set_rgb_settings_with_mode(
    device: &AkkoHidDevice,
    brightness: u8,
    speed: u8,
    direction: u8,
    color: (u8, u8, u8),
    mode: u8,
) -> Result<CommandResult, String> {
    let mut packet = AkkoPacket::with_opcode(AkkoOpcode::SetRgbSettings);

    // Set parameters based on ACTUAL web capture:
    // [7, 1, 5, 4, 7, 255, 0, 0, 232] for Dazzle
    // [7, 1, 5, 4, 8, 255, 0, 0, 231] for Color
    let data = packet.as_bytes_mut();
    data[0] = 0x07; // Opcode
    data[1] = direction; // Direction (Byte 1)
    data[2] = 5_u8.saturating_sub(speed.min(4)); // Speed INVERTED (UI 4→1, UI 0→5)
    data[3] = brightness.min(4); // Brightness DIRECT (0-4)
    data[4] = mode; // Mode: 0x07=Dazzle, 0x08=Static Color
    data[5] = color.0; // R
    data[6] = color.1; // G
    data[7] = color.2; // B

    // Calculate checksum at byte 8: 0xFF - (sum of bytes 0-7 mod 256)
    let sum: u16 = data[0..8].iter().map(|&b| b as u16).sum();
    data[8] = (0xFF_u16.wrapping_sub(sum % 256)) as u8;

    println!(
        "[SET_RGB] brightness={}, speed={} (protocol={}), dir={}, mode=0x{:02X}, color=({},{},{})",
        brightness, speed, data[2], direction, mode, color.0, color.1, color.2
    );
    println!("[SET_RGB] Packet bytes: {:?}", &data[0..16]);

    let response = device.send_feature_report(packet.as_bytes())?;
    println!(
        "[SET_RGB] Response: {:?}",
        &response[0..16.min(response.len())]
    );

    Ok(CommandResult::from_response(
        AkkoOpcode::SetRgbSettings,
        response,
    ))
}

// ============ Probing ============

/// Probe a single opcode
pub fn probe_opcode(device: &AkkoHidDevice, opcode: u8) -> Result<ProbeResult, String> {
    let akko_opcode = AkkoOpcode::from(opcode);
    let packet = AkkoPacket::with_opcode(akko_opcode);

    info!("Probing: 0x{:02X} ({})", opcode, akko_opcode.name());

    match device.send_feature_report(packet.as_bytes()) {
        Ok(response) => {
            let resp_packet = AkkoPacket::from_bytes(&response);
            let has_data = resp_packet.has_data();

            if has_data {
                info!(
                    "  -> 0x{:02X}: DATA - {}",
                    opcode,
                    resp_packet.to_hex_short()
                );
            } else {
                debug!("  -> 0x{:02X}: EMPTY", opcode);
            }

            Ok(ProbeResult {
                opcode,
                opcode_name: akko_opcode.name().to_string(),
                responded: true,
                has_data,
                response_hex: resp_packet.to_hex_short(),
            })
        }
        Err(e) => {
            warn!("  -> 0x{:02X}: ERROR - {}", opcode, e);
            Ok(ProbeResult {
                opcode,
                opcode_name: akko_opcode.name().to_string(),
                responded: false,
                has_data: false,
                response_hex: String::new(),
            })
        }
    }
}

/// Probe a range of opcodes
pub fn probe_opcode_range(
    device: &AkkoHidDevice,
    start: u8,
    end: u8,
) -> Result<Vec<ProbeResult>, String> {
    info!("Probing range: 0x{:02X} - 0x{:02X}", start, end);

    let mut results = Vec::new();

    for opcode in start..=end {
        match probe_opcode(device, opcode) {
            Ok(result) => results.push(result),
            Err(e) => warn!("Probe 0x{:02X} failed: {}", opcode, e),
        }
    }

    let valid_count = results.iter().filter(|r| r.has_data).count();
    info!(
        "Probe complete: {}/{} with data",
        valid_count,
        results.len()
    );

    Ok(results)
}

/// Run all known commands and return results
pub fn run_all_commands(device: &AkkoHidDevice) -> Result<Vec<CommandResult>, String> {
    let commands = [
        AkkoOpcode::Handshake,
        AkkoOpcode::GetProfileCount,
        AkkoOpcode::GetDeviceInfo,
        AkkoOpcode::GetRgbSettings,
        AkkoOpcode::GetRgbMode,
        AkkoOpcode::GetPerformance,
        AkkoOpcode::GetFnLockStatus,
        AkkoOpcode::GetIndicatorLed,
        AkkoOpcode::GetSleepSettings,
        AkkoOpcode::GetMacroStatus,
    ];

    let mut results = Vec::new();

    for cmd in commands {
        match execute_command(device, cmd) {
            Ok(result) => results.push(result),
            Err(e) => warn!("{} failed: {}", cmd.name(), e),
        }
    }

    Ok(results)
}
