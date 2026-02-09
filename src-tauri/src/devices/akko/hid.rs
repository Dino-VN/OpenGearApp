//! HID core layer for Akko keyboards
//! Handles low-level HID operations: open, send, receive feature reports
//!
//! IMPORTANT (Windows HID API):
//! - Buffer must be 65 bytes: [report_id (1 byte)] + [payload (64 bytes)]
//! - report_id = 0 when device doesn't use report IDs
//! - hidapi does NOT auto-prepend report_id (unlike WebHID)

use hidapi::{DeviceInfo, HidApi, HidDevice};
use log::{debug, info, warn};

const PAYLOAD_SIZE: usize = 64;
const BUFFER_SIZE: usize = 65; // report_id (1) + payload (64)
const REPORT_ID: u8 = 0;

/// Wrapper around HidDevice for Akko keyboards
pub struct AkkoHidDevice {
    device: HidDevice,
}

impl AkkoHidDevice {
    /// Open an Akko HID device by VID/PID
    /// Attempts to find the correct interface that supports Feature Reports
    pub fn open(vid: u16, pid: u16) -> Result<Self, String> {
        info!("Opening Akko device VID: 0x{:04X}, PID: 0x{:04X}", vid, pid);

        let api = HidApi::new().map_err(|e| format!("Failed to init HID API: {}", e))?;

        // Find the correct interface - keyboards often have multiple HID interfaces
        // We need the one that supports Feature Reports
        let device = Self::find_feature_report_interface(&api, vid, pid)?;

        info!("Device opened successfully");
        Ok(Self { device })
    }

    /// Find and open the HID interface that supports Feature Reports
    fn find_feature_report_interface(
        api: &HidApi,
        vid: u16,
        pid: u16,
    ) -> Result<HidDevice, String> {
        let devices: Vec<&DeviceInfo> = api
            .device_list()
            .filter(|d| d.vendor_id() == vid && d.product_id() == pid)
            .collect();

        if devices.is_empty() {
            return Err(format!(
                "No device found with VID: 0x{:04X}, PID: 0x{:04X}",
                vid, pid
            ));
        }

        info!("Found {} interface(s) for device", devices.len());

        // Try each interface until we find one that works with Feature Reports

        for (idx, dev_info) in devices.iter().enumerate() {
            info!(
                "Trying interface {}: path={:?}, usage_page=0x{:04X}, usage=0x{:04X}",
                idx,
                dev_info.path().to_str().unwrap_or("unknown"),
                dev_info.usage_page(),
                dev_info.usage()
            );

            match dev_info.open_device(api) {
                Ok(device) => {
                    // Test if this interface supports Feature Reports
                    let mut test_buf = [0u8; BUFFER_SIZE];
                    test_buf[0] = REPORT_ID;

                    // Try to get feature report - if it fails, try next interface
                    if device.get_feature_report(&mut test_buf).is_ok() {
                        info!("Interface {} supports Feature Reports", idx);
                        return Ok(device);
                    } else {
                        warn!(
                            "Interface {} does not support Feature Reports, trying next...",
                            idx
                        );
                    }
                }
                Err(e) => {
                    let err_msg = format!("Failed to open interface {}: {}", idx, e);
                    warn!("{}", err_msg);
                }
            }
        }

        // If no interface worked, try opening the first one anyway
        devices
            .first()
            .ok_or_else(|| "No device found".to_string())?
            .open_device(api)
            .map_err(|e| format!("Failed to open device: {}", e))
    }

    /// Send a 64-byte feature report and receive response
    ///
    /// Windows HID API requires:
    /// - Buffer size = 65 bytes (report_id + 64 bytes payload)
    /// - byte[0] = report_id (0 if not used)
    /// - byte[1..65] = payload
    pub fn send_feature_report(&self, data: &[u8; PAYLOAD_SIZE]) -> Result<Vec<u8>, String> {
        // Prepare 65-byte buffer: [report_id | payload]
        let mut out_buf = [0u8; BUFFER_SIZE];
        out_buf[0] = REPORT_ID;
        out_buf[1..].copy_from_slice(data);

        // Log outgoing data (payload only)
        self.log_packet("SEND", data);

        // Send feature report (must be 65 bytes on Windows)
        self.device
            .send_feature_report(&out_buf)
            .map_err(|e| format!("send_feature_report failed: {} (ensure 65-byte buffer)", e))?;

        info!("Feature report sent successfully");

        // Receive response
        let response = self.receive_feature_report()?;

        Ok(response)
    }

    /// Receive a feature report
    /// Returns 64-byte payload (strips report_id byte)
    fn receive_feature_report(&self) -> Result<Vec<u8>, String> {
        // Prepare 65-byte buffer for receiving
        let mut in_buf = [0u8; BUFFER_SIZE];
        in_buf[0] = REPORT_ID; // Must set report_id before calling get_feature_report

        let bytes_read = self
            .device
            .get_feature_report(&mut in_buf)
            .map_err(|e| format!("get_feature_report failed: {}", e))?;

        info!("Received {} bytes (including report_id)", bytes_read);

        // Extract payload (skip report_id at byte[0])
        // Return exactly 64 bytes
        let payload = if bytes_read > 1 {
            in_buf[1..BUFFER_SIZE].to_vec()
        } else {
            vec![0u8; PAYLOAD_SIZE]
        };

        // Log incoming data (payload only)
        self.log_packet("RECV", &payload);

        Ok(payload)
    }

    /// Log packet data in hex and decimal format
    fn log_packet(&self, direction: &str, data: &[u8]) {
        let hex: Vec<String> = data.iter().map(|b| format!("{:02X}", b)).collect();
        let dec: Vec<String> = data.iter().map(|b| format!("{}", b)).collect();

        info!("[{}] Hex: [{}]", direction, hex.join(" "));
        debug!("[{}] Dec: [{}]", direction, dec.join(", "));
        info!("[{}] Length: {} bytes", direction, data.len());
    }
}
