//! High-level API for Akko keyboard operations
//! Orchestrates HID layer with commands

use super::commands::{self, CommandResult, ProbeResult};
use super::detector::AkkoModel;
use super::hid::AkkoHidDevice;

use log::info;

/// Perform handshake with an Akko keyboard
pub fn akko_handshake(model: AkkoModel) -> Result<Vec<u8>, String> {
    info!("Starting handshake with Akko {}", model.name());

    let device = AkkoHidDevice::open(model.vid(), model.pid())?;
    let result = commands::cmd_handshake(&device)?;

    info!("Handshake complete: {}", result.hex_short);
    Ok(result.response)
}

/// Send arbitrary packet to Akko keyboard
pub fn akko_send_packet(model: AkkoModel, packet: [u8; 64]) -> Result<Vec<u8>, String> {
    info!("Sending packet to Akko {}", model.name());

    let device = AkkoHidDevice::open(model.vid(), model.pid())?;
    let response = device.send_feature_report(&packet)?;

    Ok(response)
}

/// Probe a single opcode (handshake first)
pub fn akko_probe_opcode(model: AkkoModel, opcode: u8) -> Result<ProbeResult, String> {
    info!("Probing opcode 0x{:02X} on Akko {}", opcode, model.name());

    let device = AkkoHidDevice::open(model.vid(), model.pid())?;

    // Handshake first
    commands::cmd_handshake(&device)?;

    // Probe the opcode
    commands::probe_opcode(&device, opcode)
}

/// Probe a range of opcodes (handshake first)
pub fn akko_probe_range(model: AkkoModel, start: u8, end: u8) -> Result<Vec<ProbeResult>, String> {
    info!(
        "Probing opcodes 0x{:02X}-0x{:02X} on Akko {}",
        start,
        end,
        model.name()
    );

    let device = AkkoHidDevice::open(model.vid(), model.pid())?;

    // Handshake first
    commands::cmd_handshake(&device)?;

    // Probe range
    commands::probe_opcode_range(&device, start, end)
}

/// Run all known commands
pub fn akko_run_all(model: AkkoModel) -> Result<Vec<CommandResult>, String> {
    info!("Running all commands on Akko {}", model.name());

    let device = AkkoHidDevice::open(model.vid(), model.pid())?;
    commands::run_all_commands(&device)
}

/// Get profile count and active profile
pub fn akko_get_profile_count(model: AkkoModel) -> Result<CommandResult, String> {
    let device = AkkoHidDevice::open(model.vid(), model.pid())?;
    commands::cmd_handshake(&device)?;
    commands::cmd_get_profile_count(&device)
}

/// Get RGB settings
pub fn akko_get_rgb_settings(model: AkkoModel) -> Result<CommandResult, String> {
    let device = AkkoHidDevice::open(model.vid(), model.pid())?;
    commands::cmd_handshake(&device)?;
    commands::cmd_get_rgb_settings(&device)
}

/// Get RGB mode
pub fn akko_get_rgb_mode(model: AkkoModel) -> Result<CommandResult, String> {
    let device = AkkoHidDevice::open(model.vid(), model.pid())?;
    commands::cmd_handshake(&device)?;
    commands::cmd_get_rgb_mode(&device)
}

/// Get performance settings (debounce)
pub fn akko_get_performance(model: AkkoModel) -> Result<CommandResult, String> {
    let device = AkkoHidDevice::open(model.vid(), model.pid())?;
    commands::cmd_handshake(&device)?;
    commands::cmd_get_performance(&device)
}

/// Set RGB settings (brightness, speed, direction, color)
pub fn akko_set_rgb_settings(
    model: AkkoModel,
    brightness: u8,
    speed: u8,
    direction: u8,
    color: (u8, u8, u8),
) -> Result<CommandResult, String> {
    let device = AkkoHidDevice::open(model.vid(), model.pid())?;
    commands::cmd_handshake(&device)?;
    commands::cmd_set_rgb_settings(&device, brightness, speed, direction, color)
}

/// Set RGB settings with specific mode
/// mode: 0x07 = Dazzle, 0x08 = Static Color
pub fn akko_set_rgb_settings_with_mode(
    model: AkkoModel,
    brightness: u8,
    speed: u8,
    direction: u8,
    color: (u8, u8, u8),
    mode: u8,
) -> Result<CommandResult, String> {
    let device = AkkoHidDevice::open(model.vid(), model.pid())?;
    commands::cmd_handshake(&device)?;
    commands::cmd_set_rgb_settings_with_mode(&device, brightness, speed, direction, color, mode)
}
