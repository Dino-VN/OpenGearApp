import { ref, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export interface ProbeResult {
    opcode: number;
    responded: boolean;
    has_data: boolean;
    response_hex: string;
}

export interface CommandResult {
    success: boolean;
    opcode: number;
    opcode_name: string;
    response: number[];
    hex_short: string;
}

export interface RgbSettings {
    mode: number;
    brightness: number;
    speed: number;
    direction: number;
    color: { r: number; g: number; b: number };
}

export interface PerformanceSettings {
    debounceDown: number;
    debounceUp: number;
}

export interface ProfileInfo {
    count: number;
    active: number;
}

export function useDevice() {
    // Device State
    const devices = ref<string[]>([]);
    const selectedDevice = ref<string | null>(null);
    const isConnecting = ref(false);
    const isConnected = ref(false);
    const firmwareVersion = ref<string>("");

    // Keyboard Data State
    const profileInfo = ref<ProfileInfo | null>(null);
    const rgbSettings = ref<RgbSettings | null>(null);
    const performanceSettings = ref<PerformanceSettings | null>(null);
    const isLoadingData = ref(false);

    // Logging
    const logEntries = ref<{ type: string; data: string; time: string }[]>([]);

    function addLog(type: string, data: string) {
        const now = new Date();
        const time = now.toLocaleTimeString("en-US", { hour12: false }) + "." + now.getMilliseconds().toString().padStart(3, "0");
        logEntries.value.unshift({ type, data, time });
        if (logEntries.value.length > 100) logEntries.value.pop();
    }

    function clearLogs() {
        logEntries.value = [];
    }

    function formatBytes(bytes: number[]): string {
        return bytes.map(b => b.toString(16).padStart(2, "0").toUpperCase()).join(" ");
    }

    async function detectDevices() {
        try {
            addLog("info", "Scanning for devices...");
            devices.value = await invoke("detect_akko_devices");
            if (devices.value.length > 0) {
                addLog("success", `Found ${devices.value.length} device(s): ${devices.value.join(", ")}`);
                // Don't auto-select here to allow list view to show
            } else {
                addLog("warning", "No devices found. Connect your keyboard and try again.");
            }
        } catch (e: any) {
            addLog("error", `Detection failed: ${e}`);
        }
    }

    async function performHandshake(modelName: string) {
        isConnecting.value = true;
        try {
            addLog("info", `Initiating handshake with ${modelName}...`);
            const response = await invoke<number[]>("akko_handshake", { model: modelName.toLowerCase() });

            isConnected.value = true;
            selectedDevice.value = modelName;

            // Parse firmware version from response
            if (response.length >= 3) {
                firmwareVersion.value = `${response[1]}.${response[2]}`;
            }

            addLog("success", `Connected! Firmware: v${firmwareVersion.value}`);
            addLog("data", formatBytes(response));

            // Automatically fetch all data after handshake
            await fetchAllData(modelName);
        } catch (e: any) {
            addLog("error", `Handshake failed: ${e}`);
            isConnected.value = false;
            selectedDevice.value = null;
        } finally {
            isConnecting.value = false;
        }
    }

    async function fetchAllData(modelName: string) {
        isLoadingData.value = true;
        addLog("info", "Fetching keyboard configuration...");

        try {
            // Get profile count
            const profileResult = await invoke<CommandResult>("akko_get_profile_count", {
                model: modelName.toLowerCase()
            });
            if (profileResult.response.length >= 3) {
                profileInfo.value = {
                    count: profileResult.response[1],
                    active: profileResult.response[2]
                };
                addLog("data", `Profiles: ${profileInfo.value.count} total, active: ${profileInfo.value.active}`);
            }

            // Get RGB settings
            const rgbResult = await invoke<CommandResult>("akko_get_rgb_settings", {
                model: modelName.toLowerCase()
            });

            console.log("[DEBUG] RGB Response:", rgbResult.response);

            if (rgbResult.response.length >= 8) {
                // Based on HID logs:
                // [135, 1, 5, 4, 8, 255, 0, 0, ...]
                //       ^  ^  ^  ^  ^RGB
                //       |  |  |  |  mode (byte[4])
                //       |  |  |  brightness (byte[3])
                //       |  |  speed protocol (byte[2])
                //       |  unknown (byte[1])
                //       opcode
                const speedProtocol = rgbResult.response[2];
                const speed = 5 - speedProtocol; // Convert back to UI value
                const brightness = rgbResult.response[3];
                const mode = rgbResult.response[4]; // Mode is at byte[4], not byte[1]!

                rgbSettings.value = {
                    mode: mode,
                    brightness: brightness,
                    speed: speed,
                    direction: 0,
                    color: {
                        r: rgbResult.response[5],
                        g: rgbResult.response[6],
                        b: rgbResult.response[7]
                    }
                };

                console.log("[DEBUG] Parsed RGB Settings:", {
                    mode: mode,
                    brightness: brightness,
                    speed: speed,
                    color: rgbSettings.value.color
                });

                addLog("data", `RGB: mode=${mode}, brightness=${brightness}, speed=${speed}, color=(${rgbSettings.value.color.r},${rgbSettings.value.color.g},${rgbSettings.value.color.b})`);
            }

            // Get performance settings
            const perfResult = await invoke<CommandResult>("akko_get_performance", {
                model: modelName.toLowerCase()
            });
            if (perfResult.response.length >= 4) {
                performanceSettings.value = {
                    debounceDown: perfResult.response[1],
                    debounceUp: perfResult.response[3]
                };
                addLog("data", `Debounce: ${performanceSettings.value.debounceDown / 100}ms down`);
            }

            addLog("success", "Configuration loaded!");
        } catch (e: any) {
            addLog("error", `Failed to fetch data: ${e}`);
        } finally {
            isLoadingData.value = false;
        }
    }

    // Polling logic
    let pollingInterval: number | null = null;
    function startDevicePolling() {
        if (pollingInterval) return;
        pollingInterval = window.setInterval(async () => {
            try {
                const newDevices = await invoke<string[]>("detect_akko_devices");

                const devicesChanged =
                    newDevices.length !== devices.value.length ||
                    !newDevices.every((d, i) => d === devices.value[i]);

                if (devicesChanged) {
                    const oldCount = devices.value.length;
                    const newCount = newDevices.length;
                    devices.value = newDevices;

                    if (newCount > oldCount) {
                        addLog("success", `New device detected: ${newDevices[newCount - 1]}`);
                        if (!selectedDevice.value) {
                            // Auto connect if no device selected? optional
                        }
                    } else if (newCount < oldCount) {
                        addLog("warning", "Device disconnected");
                        // If the selected device is gone
                        if (selectedDevice.value && !newDevices.includes(selectedDevice.value)) {
                            isConnected.value = false;
                            selectedDevice.value = null;
                        }
                    }
                }
            } catch (e) {
                console.error("Polling error:", e);
            }
        }, 2000);
        addLog("info", "Auto-detection enabled (polling every 2s)");
    }

    onUnmounted(() => {
        if (pollingInterval) clearInterval(pollingInterval);
    });

    return {
        devices,
        selectedDevice,
        isConnecting,
        isConnected,
        firmwareVersion,
        profileInfo,
        rgbSettings,
        performanceSettings,
        isLoadingData,
        logEntries,
        addLog,
        clearLogs,
        detectDevices,
        performHandshake,
        fetchAllData,
        startDevicePolling
    };
}
