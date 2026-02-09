mod devices;

use active_win_pos_rs::get_active_window;
use devices::akko::{self, AkkoModel, CommandResult, ProbeResult};
use log::{error, info};

/// Tauri command: Get the active application name/process
#[tauri::command]
fn get_active_app() -> Result<String, String> {
    match get_active_window() {
        Ok(active_window) => {
            // Return process name (e.g., "code" or "chrome")
            // On Windows this is usually the executable name without .exe, but let's check
            Ok(active_window.app_name)
        }
        Err(e) => {
            error!("Failed to get active window: {:?}", e);
            Err("Failed to get active window".to_string())
        }
    }
}

/// Tauri command: Perform handshake with Akko keyboard
#[tauri::command]
fn akko_handshake(model: String) -> Result<Vec<u8>, String> {
    let akko_model =
        AkkoModel::from_str(&model).ok_or_else(|| format!("Unknown Akko model: {}", model))?;

    info!("Tauri command: akko_handshake({})", model);
    akko::akko_handshake(akko_model)
}

/// Tauri command: Detect connected Akko devices
#[tauri::command]
fn detect_akko_devices() -> Vec<String> {
    info!("Tauri command: detect_akko_devices");
    akko::detector::detect_akko_devices()
        .iter()
        .map(|m| m.name().to_string())
        .collect()
}

/// Tauri command: Probe single opcode
#[tauri::command]
fn akko_probe_opcode(model: String, opcode: u8) -> Result<ProbeResult, String> {
    let akko_model =
        AkkoModel::from_str(&model).ok_or_else(|| format!("Unknown Akko model: {}", model))?;

    info!(
        "Tauri command: akko_probe_opcode({}, 0x{:02X})",
        model, opcode
    );
    akko::api::akko_probe_opcode(akko_model, opcode)
}

/// Tauri command: Probe opcode range
#[tauri::command]
fn akko_probe_range(model: String, start: u8, end: u8) -> Result<Vec<ProbeResult>, String> {
    let akko_model =
        AkkoModel::from_str(&model).ok_or_else(|| format!("Unknown Akko model: {}", model))?;

    info!(
        "Tauri command: akko_probe_range({}, 0x{:02X}-0x{:02X})",
        model, start, end
    );
    akko::api::akko_probe_range(akko_model, start, end)
}

/// Tauri command: Run all known commands
#[tauri::command]
fn akko_run_all(model: String) -> Result<Vec<CommandResult>, String> {
    let akko_model =
        AkkoModel::from_str(&model).ok_or_else(|| format!("Unknown Akko model: {}", model))?;

    info!("Tauri command: akko_run_all({})", model);
    akko::api::akko_run_all(akko_model)
}

/// Tauri command: Get profile count
#[tauri::command]
fn akko_get_profile_count(model: String) -> Result<CommandResult, String> {
    let akko_model =
        AkkoModel::from_str(&model).ok_or_else(|| format!("Unknown Akko model: {}", model))?;

    akko::api::akko_get_profile_count(akko_model)
}

/// Tauri command: Get RGB settings
#[tauri::command]
fn akko_get_rgb_settings(model: String) -> Result<CommandResult, String> {
    let akko_model =
        AkkoModel::from_str(&model).ok_or_else(|| format!("Unknown Akko model: {}", model))?;

    akko::api::akko_get_rgb_settings(akko_model)
}

/// Tauri command: Get RGB mode (0x88) - contains brightness
#[tauri::command]
fn akko_get_rgb_mode(model: String) -> Result<CommandResult, String> {
    let akko_model =
        AkkoModel::from_str(&model).ok_or_else(|| format!("Unknown Akko model: {}", model))?;

    akko::api::akko_get_rgb_mode(akko_model)
}

/// Tauri command: Get performance settings
#[tauri::command]
fn akko_get_performance(model: String) -> Result<CommandResult, String> {
    let akko_model =
        AkkoModel::from_str(&model).ok_or_else(|| format!("Unknown Akko model: {}", model))?;

    akko::api::akko_get_performance(akko_model)
}

/// Tauri command: Send raw packet
#[tauri::command]
fn akko_send_raw(model: String, packet: Vec<u8>) -> Result<Vec<u8>, String> {
    let akko_model =
        AkkoModel::from_str(&model).ok_or_else(|| format!("Unknown Akko model: {}", model))?;

    if packet.len() != 64 {
        return Err(format!("Packet must be 64 bytes, got {}", packet.len()));
    }

    let mut arr = [0u8; 64];
    arr.copy_from_slice(&packet);

    info!(
        "Tauri command: akko_send_raw({}, {} bytes)",
        model,
        packet.len()
    );
    akko::akko_send_packet(akko_model, arr)
}

/// Tauri command: Set RGB settings
#[tauri::command]
fn akko_set_rgb_settings(
    model: String,
    brightness: u8,
    speed: u8,
    direction: u8,
    r: u8,
    g: u8,
    b: u8,
) -> Result<CommandResult, String> {
    let akko_model =
        AkkoModel::from_str(&model).ok_or_else(|| format!("Unknown Akko model: {}", model))?;

    info!("Tauri command: akko_set_rgb_settings({}, brightness={}, speed={}, direction={}, color=({},{},{}))", 
          model, brightness, speed, direction, r, g, b);
    akko::api::akko_set_rgb_settings(akko_model, brightness, speed, direction, (r, g, b))
}

/// Tauri command: Set RGB settings with specific mode
/// mode: 7 = Dazzle, 8 = Static Color
#[tauri::command]
fn akko_set_rgb_with_mode(
    model: String,
    brightness: u8,
    speed: u8,
    direction: u8,
    r: u8,
    g: u8,
    b: u8,
    mode: u8,
) -> Result<CommandResult, String> {
    let akko_model =
        AkkoModel::from_str(&model).ok_or_else(|| format!("Unknown Akko model: {}", model))?;

    info!("Tauri command: akko_set_rgb_with_mode({}, brightness={}, speed={}, mode=0x{:02X}, color=({},{},{}))", 
          model, brightness, speed, mode, r, g, b);
    akko::api::akko_set_rgb_settings_with_mode(
        akko_model,
        brightness,
        speed,
        direction,
        (r, g, b),
        mode,
    )
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, Some(vec!["--flag1", "--flag2"])))
        .invoke_handler(tauri::generate_handler![
            akko_handshake,
            detect_akko_devices,
            akko_probe_opcode,
            akko_probe_range,
            akko_run_all,
            akko_get_profile_count,
            akko_get_rgb_settings,
            akko_get_rgb_mode,
            akko_get_performance,
            akko_set_rgb_settings,
            akko_set_rgb_with_mode,
            akko_set_rgb_with_mode,
            akko_send_raw,
            get_active_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
