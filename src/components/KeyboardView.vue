<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getLayout } from "../layouts/layouts";
import { LED_MODES, type LedMode } from "../layouts/led-modes";
import { useUpdater } from "../composables/useUpdater";

const props = defineProps<{
  modelName: string;
  rgbSettings: { mode: number; brightness: number; speed: number; direction: number; color: { r: number; g: number; b: number } } | null;
  performanceSettings: { debounceDown: number; debounceUp: number } | null;
  profileInfo: { count: number; active: number } | null;
  firmwareVersion: string;
}>();

const emit = defineEmits<{
  (e: 'update:rgbSettings', value: typeof props.rgbSettings): void;
  (e: 'log', type: string, message: string): void;
  (e: 'close'): void;
}>();

// UI State
const activeTab = ref<'lighting' | 'assignments' | 'system'>('lighting');
const isSaving = ref(false);

// Updater
const { 
  updateAvailable, 
  updateVersion, 
  updateStatus, 
  isChecking, 
  checkForUpdates, 
  installUpdate 
} = useUpdater();

// Layout
const layout = computed(() => getLayout(props.modelName));

// RGB Color Computed Property
const rgbColorString = computed(() => {
  if (!props.rgbSettings) return '#B4B4B4';
  const { r, g, b } = props.rgbSettings.color;
  return `rgb(${r}, ${g}, ${b})`;
});

// Get key color based on mode and position
function getKeyColor(keyX: number, maxWidth: number): string {
  // If LED is off or brightness is 0, return gray
  if (props.rgbSettings?.mode === 0 || props.rgbSettings?.brightness === 0) {
    return '#333333';
  }
  
  // Round Dazzle mode (23) or Z-Shape Dazzle (7)
  // Use RGB gradient based on X position
  // Red (0°) → Green (120°) → Blue (240°)
  if (props.rgbSettings?.mode === 7 || props.rgbSettings?.mode === 23) {
    const ratio = keyX / maxWidth;
    return hslToRgb(ratio * 240, 100, 50); // 0-240° only (red to blue)
  }
  
  // Color mode (8) or default - use selected color
  return rgbColorString.value;
}

// Helper: HSL to RGB
function hslToRgb(h: number, s: number, l: number): string {
  s /= 100;
  l /= 100;
  const k = (n: number) => (n + h / 30) % 12;
  const a = s * Math.min(l, 1 - l);
  const f = (n: number) => l - a * Math.max(-1, Math.min(k(n) - 3, Math.min(9 - k(n), 1)));
  const r = Math.round(255 * f(0));
  const g = Math.round(255 * f(8));
  const b = Math.round(255 * f(4));
  return `rgb(${r}, ${g}, ${b})`;
}

// Helper: Check if specific mode is active based on current device settings
const isModeActive = (mode: LedMode) => {
  if (!props.rgbSettings) return false;
  
  if (mode.id === 0) {
    return props.rgbSettings.mode === 0;
  }
  
  // Must be ON for any other mode
  if (props.rgbSettings.mode === 0) return false;
  
  // Static (-1)
  if (mode.id === -1) {
    return props.rgbSettings.speed === 0;
  }
  
  // Drift (4)
  if (mode.id === 4) {
    // Check if direction is 4 (Drift Effect ID) AND speed > 0
    return props.rgbSettings.speed !== 0 && props.rgbSettings.direction === 4;
  }
  
  // Specific Effect Modes (ID > 4 and != 4)
  if (mode.id > 4) {
    return props.rgbSettings.speed !== 0 && props.rgbSettings.direction === mode.id;
  }
  
  return false;
};

// Helper: Check if direction controls should be visible
const showDirectionControls = computed(() => {
  if (!props.rgbSettings) return false;
  const dir = props.rgbSettings.direction || 0;
  
  // Basic Drift (4)
  if (props.rgbSettings.speed !== 0 && dir === 4) return true;
  
  return false;
});



// Current LED mode
const currentLedMode = computed(() => {
  if (!props.rgbSettings) return LED_MODES[0];
  
  if (props.rgbSettings.mode === 0) return LED_MODES.find(m => m.id === 0) || LED_MODES[0];
  if (props.rgbSettings.speed === 0) return LED_MODES.find(m => m.id === -1);
  
  const dir = props.rgbSettings.direction || 1;
  
  // Drift (4)
  if (dir === 4) return LED_MODES.find(m => m.id === 4);
  
  // Specific Mode (ID > 4)
  const match = LED_MODES.find(m => m.id === dir);
  if (match) return match;
  
  return LED_MODES.find(m => m.id === 4); // Default to Drift if unknown
});

// Local State for Edit
const localBrightness = ref(4);
const localSpeed = ref(2);
const localColor = ref({ r: 0, g: 212, b: 255 }); // Default cyan
const colorMode = ref<'color' | 'dazzle' | 'round'>('color'); // 'color' = mode 8, 'dazzle' = mode 7, 'round' = mode 23
const localDirection = ref(1);

// Sync local state when props change
watch(() => props.rgbSettings, (newVal) => {
  if (newVal) {
    localBrightness.value = newVal.brightness;
    localSpeed.value = newVal.speed;
    localColor.value = { ...newVal.color };
    // Determine if dazzle or color based on mode
    // Mode 7 = Dazzle, Mode 8 = Color
    // Mode 23 = Round Dazzle, Mode 24 = Round Color
    // Drift Modes:
    // Right: 7/8
    // Left: 23/24
    // Down: 39/40
    // Up: 55/56
    const m = newVal.mode;
    if (m === 7 || m === 23 || m === 39 || m === 55) {
      colorMode.value = 'dazzle';
    } else {
      colorMode.value = 'color';
    }
    
    // Map Drift Modes to Local Direction (1=Left, 2=Right, 3=Up, 4=Down)
    // Right (7/8), Left (23/24), Down (39/40), Up (55/56)
    if (newVal.direction === 4) {
        if (m === 23 || m === 24) localDirection.value = 1; // Left
        else if (m === 7 || m === 8) localDirection.value = 2; // Right
        else if (m === 55 || m === 56) localDirection.value = 3; // Up
        else if (m === 39 || m === 40) localDirection.value = 4; // Down
    } else {
        localDirection.value = newVal.direction;
    }
  }
}, { immediate: true });

// Set Color Mode (Color vs Dazzle)
async function setColorMode(mode: 'color' | 'dazzle') {
  if (!props.rgbSettings || !props.modelName) return;
  
  colorMode.value = mode;
  
  // Default logic (Z-Shape)
  let protocolMode = mode === 'dazzle' ? 7 : 8;
  
  // Special handling for Steady Stream (7), Flowing Spring (11), Flowers Blooming (12), and Peak Turn (15)
  // All use the same sub-modes (7,8,23,24) for patterns
  if (currentLedMode.value?.id === 7 || currentLedMode.value?.id === 11 || currentLedMode.value?.id === 12 || currentLedMode.value?.id === 15) {
      // Check if we are currently in Round/Inward/Left/Clockwise mode (23 or 24)
      // Note: props.rgbSettings.mode might be 23 or 24
      const isRoundOrInward = props.rgbSettings.mode === 23 || props.rgbSettings.mode === 24;
      
      if (mode === 'dazzle') {
          // If Round, go to 23. If Z-Shape, go to 7.
          protocolMode = isRoundOrInward ? 23 : 7;
      } else {
          // If Round, go to 24. If Z-Shape, go to 8.
          protocolMode = isRoundOrInward ? 24 : 8;
      }
  }
  // Special handling for Drift (ID 4)
  else if (currentLedMode.value?.id === 4) {
       // Check current direction from mode
       const m = props.rgbSettings.mode;
       // Right (7/8), Left (23/24), Down (39/40), Up (55/56)
       let baseMode = 7; // Right Dazzle default
       if (m === 23 || m === 24) baseMode = 23; // Left
       else if (m === 39 || m === 40) baseMode = 39; // Down
       else if (m === 55 || m === 56) baseMode = 55; // Up
       else baseMode = 7; // Right
       
       // If Color, add 1. If Dazzle, use base.
       protocolMode = (mode === 'color') ? baseMode + 1 : baseMode;
  }
  
  emit('log', 'info', `Setting color mode: ${mode} (protocol=${protocolMode})`);
  
  try {
    await invoke('akko_set_rgb_with_mode', {
      model: props.modelName.toLowerCase(),
      brightness: localBrightness.value,
      speed: localSpeed.value,
      direction: props.rgbSettings.direction, // Use current encoded direction
      r: localColor.value.r,
      g: localColor.value.g,
      b: localColor.value.b,
      mode: protocolMode,
    });
    
    emit('update:rgbSettings', {
      ...props.rgbSettings,
      mode: protocolMode,
    });
    emit('log', 'success', `Color mode set to: ${mode}`);
  } catch (e: any) {
    emit('log', 'error', `Failed to set color mode: ${e}`);
  }
}

// Set Pattern for Steady Stream (7) or Flowing Spring (11)
// Pattern A: Z-Shape / Outward (Modes 7/8)
// Pattern B: Round / Inward (Modes 23/24)
async function setPattern(effectId: number, patternType: 'A' | 'B') {
  if (!props.rgbSettings || !props.modelName) return;
  
  let targetMode = 7;
  
  // Check if we are currently in a "Color" mode (8 or 24)
  // or if the UI state says 'color'
  const isColor = props.rgbSettings.mode === 8 || props.rgbSettings.mode === 24 || colorMode.value === 'color';

  if (patternType === 'A') {
      // Z-Shape / Outward
      // If currently Color, go to 8. Else Dazzle 7.
      targetMode = isColor ? 8 : 7;
  } else {
      // Round / Inward
      // If currently Color, go to 24. Else Dazzle 23.
      targetMode = isColor ? 24 : 23;
  }
  
  emit('log', 'info', `Setting pattern for effect ${effectId}: ${patternType} (mode=${targetMode})`);
  
  try {
    await invoke('akko_set_rgb_with_mode', {
      model: props.modelName.toLowerCase(),
      brightness: localBrightness.value,
      speed: localSpeed.value,
      direction: effectId, // 7 or 11
      r: localColor.value.r,
      g: localColor.value.g,
      b: localColor.value.b,
      mode: targetMode,
    });
    
    emit('update:rgbSettings', {
      ...props.rgbSettings,
      direction: effectId,
      mode: targetMode,
    });
    
    // Update colorMode ref manually
    if (targetMode === 8 || targetMode === 24) colorMode.value = 'color';
    else colorMode.value = 'dazzle'; // 7 and 23
    
    emit('log', 'success', `Pattern set to: ${patternType}`);
  } catch (e: any) {
    emit('log', 'error', `Failed to set pattern: ${e}`);
  }
}

// Actions
async function selectLedMode(mode: LedMode) {
  if (!props.rgbSettings || !props.modelName) return;
  isSaving.value = true;
  emit('log', 'info', `Setting LED mode: ${mode.name} (id=${mode.id})`);

  try {
    // Determine Protocol Mode & Speed based on UI Mode selection (Static vs Drift)
    let protocolMode = 8; // Default to Color
    let speed = 0;
    
    // Check if we are currently in Dazzle mode (7), stay in Dazzle if switching between Static/Drift
    // otherwise default to Color (8)
    if (props.rgbSettings.mode === 7) {
      protocolMode = 7;
    }

    if (mode.id === 0) {
      // OFF
      protocolMode = 8; // Always On but Brightness 0
    } else if (mode.id === -1) {
      // STATIC
      speed = 0;
      // Use current colorMode preference (Dazzle or Color)
      // If we are switching TO Static, we utilize the current colorMode
      if (colorMode.value === 'dazzle') {
          protocolMode = 7;
      } else {
          protocolMode = 8;
      }
    } else if (mode.id === 4) {
      // DRIFT (ID 4)
      // Default to speed 2 if currently 0
      speed = props.rgbSettings.speed === 0 ? 2 : props.rgbSettings.speed;
      // Default direction Right (Mode 7/8)
      // If color, 8. If dazzle, 7.
      protocolMode = colorMode.value === 'color' ? 8 : 7;
    } else if (mode.id > 4) {
      // PREDEFINED EFFECTS (Waves, Stars, etc.)
      // ID is the Effect ID directly
      speed = props.rgbSettings.speed === 0 ? 2 : props.rgbSettings.speed;
    }

    const brightness = mode.id === 0 ? 0 : (localBrightness.value || 4);
    
    // Determine direction / effect ID:
    let direction = (props.rgbSettings.direction || 1);
    
    // Static (-1) always uses Direction 1
    if (mode.id === -1) {
        direction = 1;
    }
    // Drift (4)
    else if (mode.id === 4) {
        direction = 4;
    }
    // If mode has a specific Effect ID (>4), use it
    else if (mode.id > 4) {
      direction = mode.id;
    }
    
    // Update local speed just in case
    localSpeed.value = speed;

    await invoke('akko_set_rgb_with_mode', {
      model: props.modelName.toLowerCase(),
      brightness: brightness,
      speed: speed,
      direction: direction,
      r: localColor.value.r,
      g: localColor.value.g,
      b: localColor.value.b,
      mode: protocolMode,
    });
    
    // Update local brightness
    if (mode.id === 0) {
      localBrightness.value = 0;
    } else if (localBrightness.value === 0) {
      localBrightness.value = 4;
    }
    
    emit('update:rgbSettings', {
      ...props.rgbSettings,
      mode: protocolMode,
      brightness: brightness,
      speed: speed,
      direction: direction,
    });
    emit('log', 'success', `LED mode set to: ${mode.name}`);
  } catch (e: any) {
    emit('log', 'error', `Failed to set LED mode: ${e}`);
  } finally {
    isSaving.value = false;
  }
}

// Debounce timers
let brightnessTimer: number | null = null;
let speedTimer: number | null = null;

function updateBrightness(val: number) {
  localBrightness.value = val;
  
  // Debounce: wait 500ms after last change before sending
  if (brightnessTimer) clearTimeout(brightnessTimer);
  
  brightnessTimer = setTimeout(async () => {
    if (!props.rgbSettings || !props.modelName) return;
    
    try {
      emit('log', 'info', `Setting brightness: ${val}`);
      
      await invoke('akko_set_rgb_settings', {
        model: props.modelName.toLowerCase(),
        brightness: val,
        speed: props.rgbSettings.speed,
        direction: props.rgbSettings.direction,
        r: props.rgbSettings.color.r,
        g: props.rgbSettings.color.g,
        b: props.rgbSettings.color.b,
      });
      
      emit('log', 'success', `Brightness set to: ${val}`);
    } catch (e: any) {
      emit('log', 'error', `Failed to set brightness: ${e}`);
    }
  }, 500);
}



function updateSpeed(val: number) {
  localSpeed.value = val;
  
  // Debounce: wait 500ms after last change before sending
  if (speedTimer) clearTimeout(speedTimer);
  
  speedTimer = setTimeout(async () => {
    if (!props.rgbSettings || !props.modelName) return;
    
    try {
      emit('log', 'info', `Setting speed: ${val}`);
      
      await invoke('akko_set_rgb_settings', {
        model: props.modelName.toLowerCase(),
        brightness: localBrightness.value,
        speed: val,
        direction: props.rgbSettings.direction,
        r: props.rgbSettings.color.r,
        g: props.rgbSettings.color.g,
        b: props.rgbSettings.color.b,
      });
      
      emit('log', 'success', `Speed set to: ${val}`);
    } catch (e: any) {
      emit('log', 'error', `Failed to set speed: ${e}`);
    }
  }, 500);
}

// Direction update
async function updateDirection(dir: number) {
  if (!props.rgbSettings || !props.modelName) return;
  localDirection.value = dir;
  
  emit('log', 'info', `Setting direction: ${dir}`);
  
  try {
    // Determine mode to use
    let mode = props.rgbSettings.mode;
    let direction = props.rgbSettings.direction;

    // For Drift (4), direction determines the Mode
    if (direction === 4) {
        const isColor = (colorMode.value === 'color');
        // Right (7/8), Left (23/24), Down (39/40), Up (55/56)
        if (dir === 1) mode = isColor ? 24 : 23; // Left
        else if (dir === 2) mode = isColor ? 8 : 7; // Right
        else if (dir === 3) mode = isColor ? 56 : 55; // Up
        else if (dir === 4) mode = isColor ? 40 : 39; // Down
    } else {
        // For other modes, we might just update valid directions if supported
        // But mainly this function is for Drift control
        // If we enter here and it's not Drift, we should probably ignore or default.
        // Assuming this is only called when direction controls are visible
    }

    await invoke('akko_set_rgb_with_mode', {
      model: props.modelName.toLowerCase(),
      brightness: localBrightness.value,
      speed: localSpeed.value,
      direction: direction,
      r: localColor.value.r,
      g: localColor.value.g,
      b: localColor.value.b,
      mode: mode,
    });
    
    emit('update:rgbSettings', {
      ...props.rgbSettings,
      direction: direction,
      mode: mode,
    });
    
    emit('log', 'success', `Direction set to: ${dir}`);
  } catch (e: any) {
    emit('log', 'error', `Failed to set direction: ${e}`);
  }
}

// Key Flattening
const allKeys = computed(() => {
  if (!layout.value) return [];
  return layout.value.rows.flat();
});

// Color picker helpers
function rgbToHex(r: number, g: number, b: number): string {
  const toHex = (n: number) => n.toString(16).padStart(2, '0');
  return `#${toHex(r)}${toHex(g)}${toHex(b)}`;
}

function hexToRgb(hex: string): { r: number; g: number; b: number } {
  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  return result ? {
    r: parseInt(result[1], 16),
    g: parseInt(result[2], 16),
    b: parseInt(result[3], 16)
  } : { r: 0, g: 0, b: 0 };
}

function handleColorPicker(event: Event) {
  const hex = (event.target as HTMLInputElement).value;
  const { r, g, b } = hexToRgb(hex);
  updateColor(r, g, b);
}

// Color update with debounce
let colorTimer: number | null = null;

function updateColor(r: number, g: number, b: number) {
  localColor.value = { r, g, b };
  
  // Debounce: wait 500ms after last change before sending
  if (colorTimer) clearTimeout(colorTimer);
  
  colorTimer = setTimeout(async () => {
    if (!props.rgbSettings || !props.modelName) return;
    
    try {
      emit('log', 'info', `Setting color: RGB(${r}, ${g}, ${b})`);
      
      // Use mode 8 (Static Color) when setting color
      await invoke('akko_set_rgb_with_mode', {
        model: props.modelName.toLowerCase(),
        brightness: localBrightness.value,
        speed: localSpeed.value,
        direction: props.rgbSettings.direction,
        r: r,
        g: g,
        b: b,
        mode: 8, // Static Color mode
      });
      
      // Update parent state
      emit('update:rgbSettings', {
        ...props.rgbSettings,
        color: { r, g, b },
        mode: 8, // Now in color mode
      });
      
      emit('log', 'success', `Color set to: RGB(${r}, ${g}, ${b})`);
    } catch (e: any) {
      emit('log', 'error', `Failed to set color: ${e}`);
    }
  }, 500);
}

const boardPadding = 12;
const svgWidth = computed(() => (layout.value?.width || 0) + boardPadding * 2);
const svgHeight = computed(() => (layout.value?.height || 0) + boardPadding * 2);

</script>

<template>
  <div class="keyboard-detail-view">
    <!-- Sidebar Navigation -->
    <aside class="sidebar">
      <div class="sidebar-top">
        <button class="btn-back-sidebar" @click="emit('close')" title="Back to Devices">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M19 12H5M12 19l-7-7 7-7"/>
          </svg>
        </button>
        <!-- <div class="device-thumb">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <rect x="2" y="6" width="20" height="12" rx="2" />
            <line x1="6" y1="10" x2="6" y2="10" />
            <line x1="10" y1="10" x2="10" y2="10" />
            <line x1="14" y1="10" x2="14" y2="10" />
            <line x1="18" y1="10" x2="18" y2="10" />
            <line x1="8" y1="14" x2="16" y2="14" />
          </svg>
        </div> -->
        <div class="device-meta">
          <span class="model-name">{{ modelName }}</span>
          <span class="connection-status">
            <span class="status-dot"></span> Connected
          </span>
        </div>
      </div>

      <nav class="sidebar-nav">
        <button 
          class="nav-item" 
          :class="{ active: activeTab === 'lighting' }"
          @click="activeTab = 'lighting'"
          title="LIGHTSYNC"
        >
          <div class="icon-box">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="5"/>
              <path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42"/>
            </svg>
          </div>
          <span class="nav-label">LIGHTSYNC</span>
        </button>

        <button 
          class="nav-item" 
          :class="{ active: activeTab === 'assignments' }"
          @click="activeTab = 'assignments'"
          title="Assignments"
        >
          <div class="icon-box">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
              <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
            </svg>
          </div>
          <span class="nav-label">Assignments</span>
        </button>

        <button 
          class="nav-item" 
          :class="{ active: activeTab === 'system' }"
          @click="activeTab = 'system'"
          title="Settings"
        >
          <div class="icon-box">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="3"/>
              <path d="M19.4 15a1.65 1.65 0 00.33 1.82l.06.06a2 2 0 010 2.83 2 2 0 01-2.83 0l-.06-.06a1.65 1.65 0 00-1.82-.33 1.65 1.65 0 00-1 1.51V21a2 2 0 01-4 0v-.09A1.65 1.65 0 009 19.4a1.65 1.65 0 00-1.82.33l-.06.06a2 2 0 01-2.83 0 2 2 0 010-2.83l.06-.06a1.65 1.65 0 00.33-1.82 1.65 1.65 0 00-1.51-1H3a2 2 0 010-4h.09A1.65 1.65 0 004.6 9a1.65 1.65 0 00-.33-1.82l-.06-.06a2 2 0 010-2.83 2 2 0 012.83 0l.06.06a1.65 1.65 0 001.82.33H9a1.65 1.65 0 001-1.51V3a2 2 0 114 0v.09a1.65 1.65 0 001 1.51 1.65 1.65 0 001.82-.33l.06-.06a2 2 0 012.83 0 2 2 0 010 2.83l-.06.06a1.65 1.65 0 00-.33 1.82V9a1.65 1.65 0 001.51 1H21a2 2 0 010 4h-.09a1.65 1.65 0 00-1.51 1z"/>
            </svg>
          </div>
          <span class="nav-label">Settings</span>
        </button>
      </nav>
    </aside>

    <!-- Main Content Area -->
    <main class="content-area">
      <!-- Lighting Settings Panel (Left side overlay or bottom) -->
      <transition name="fade">
        <div v-if="activeTab === 'lighting'" class="settings-panel left">
          <div class="panel-header">
            <h3>Effect</h3>
          </div>
          <div class="panel-body">
            <!-- Preset List -->
            <div class="preset-list">
              <button 
                v-for="mode in LED_MODES" 
                :key="mode.id"
                class="preset-item"
                :class="{ active: isModeActive(mode) }"
                @click="selectLedMode(mode)"
              >
                {{ mode.name }}
              </button>
            </div>

            <div class="divider"></div>

            <!-- Color Mode Toggle - Hidden when Off -->
            <div class="color-mode-section" v-if="props.rgbSettings?.mode !== 0">
              <label>Effect Type</label>
              <div class="color-mode-toggle">
                <button 
                  class="mode-btn" 
                  :class="{ active: colorMode === 'color' }"
                  @click="setColorMode('color')"
                >
                  <span class="mode-icon">🎨</span>
                  Color
                </button>
                <button 
                  class="mode-btn" 
                  :class="{ active: colorMode === 'dazzle' }"
                  @click="setColorMode('dazzle')"
                >
                  <span class="mode-icon">✨</span>
                  Dazzle
                </button>
              </div>
              
              <!-- Dazzle preview gradient -->
              <div class="dazzle-preview" v-if="colorMode === 'dazzle'">
                <div class="dazzle-gradient"></div>
                <span class="dazzle-label">Red → Green → Blue gradient</span>
              </div>
            </div>

            <!-- Controls - Hidden when Off -->
            <div class="controls-section" v-if="props.rgbSettings?.mode !== 0">
              <div class="control-group">
                <label>Brightness</label>
                <input 
                  type="range" 
                  min="0" 
                  max="4" 
                  :value="localBrightness"
                  @input="updateBrightness(Number(($event.target as HTMLInputElement).value))"
                  class="slider"
                />
              </div>

              <div class="control-group" v-if="props.rgbSettings?.speed !== 0">
                <label>Speed</label>
                <input 
                  type="range" 
                  min="1" 
                  max="4" 
                  :value="localSpeed"
                  @input="updateSpeed(Number(($event.target as HTMLInputElement).value))"
                  class="slider"
                />
              </div>
              
              <!-- Pattern Select for Steady Stream (ID 7) -->
              <div class="control-group" v-if="currentLedMode?.id === 7">
                <label>Pattern</label>
                <div class="direction-controls">
                   <button 
                    class="dir-btn wide" 
                    :class="{ active: props.rgbSettings?.mode === 7 || props.rgbSettings?.mode === 8 }"
                    @click="setPattern(7, 'A')"
                    title="Z-Shape"
                  >Z-Shape</button>
                  <button 
                    class="dir-btn wide" 
                    :class="{ active: props.rgbSettings?.mode === 23 || props.rgbSettings?.mode === 24 }"
                    @click="setPattern(7, 'B')"
                    title="Round"
                  >Round</button>
                </div>
              </div>

              <!-- Pattern Select for Flowing Spring (ID 11) -->
              <div class="control-group" v-if="currentLedMode?.id === 11">
                <label>Pattern</label>
                <div class="direction-controls">
                   <button 
                    class="dir-btn wide" 
                    :class="{ active: props.rgbSettings?.mode === 7 || props.rgbSettings?.mode === 8 }"
                    @click="setPattern(11, 'A')"
                    title="Outward"
                  >Outward</button>
                  <button 
                    class="dir-btn wide" 
                    :class="{ active: props.rgbSettings?.mode === 23 || props.rgbSettings?.mode === 24 }"
                    @click="setPattern(11, 'B')"
                    title="Inward"
                  >Inward</button>
                </div>
              </div>

              <!-- Pattern Select for Flowers Blooming (ID 12) -->
              <div class="control-group" v-if="currentLedMode?.id === 12">
                <label>Pattern</label>
                <div class="direction-controls">
                   <button 
                    class="dir-btn wide" 
                    :class="{ active: props.rgbSettings?.mode === 7 || props.rgbSettings?.mode === 8 }"
                    @click="setPattern(12, 'A')"
                    title="Right"
                  >Right</button>
                  <button 
                    class="dir-btn wide" 
                    :class="{ active: props.rgbSettings?.mode === 23 || props.rgbSettings?.mode === 24 }"
                    @click="setPattern(12, 'B')"
                    title="Left"
                  >Left</button>
                </div>
              </div>

              <!-- Pattern Select for Peak Turn (ID 15) -->
              <div class="control-group" v-if="currentLedMode?.id === 15">
                <label>Pattern</label>
                <div class="direction-controls">
                   <button 
                    class="dir-btn wide" 
                    :class="{ active: props.rgbSettings?.mode === 7 || props.rgbSettings?.mode === 8 }"
                    @click="setPattern(15, 'A')"
                    title="Counterclockwise"
                  >CCW</button>
                  <button 
                    class="dir-btn wide" 
                    :class="{ active: props.rgbSettings?.mode === 23 || props.rgbSettings?.mode === 24 }"
                    @click="setPattern(15, 'B')"
                    title="Clockwise"
                  >CW</button>
                </div>
              </div>

              <!-- Direction: Show only if in Drift mode (Effect ID 1-4) or Meteor (18) -->
              <div class="control-group" v-if="showDirectionControls">
                <label>Direction</label>
                <div class="direction-controls">
                  <button 
                    class="dir-btn" 
                    :class="{ active: localDirection === 1 }"
                    @click="updateDirection(1)"
                    title="Left"
                  >←</button>
                  <button 
                    class="dir-btn" 
                    :class="{ active: localDirection === 2 }"
                    @click="updateDirection(2)"
                    title="Right"
                  >→</button>
                  <button 
                    class="dir-btn" 
                    :class="{ active: localDirection === 3 }"
                    @click="updateDirection(3)"
                    title="Up"
                  >↑</button>
                  <button 
                    class="dir-btn" 
                    :class="{ active: localDirection === 4 }"
                    @click="updateDirection(4)"
                    title="Down"
                  >↓</button>
                </div>
              </div>
            </div>
            
            <!-- Color Picker - Show if Mode 8 (Color) OR Mode 23 (Round) OR other color-supported modes -->
            <div class="color-section" v-if="props.rgbSettings?.mode !== 0 && colorMode === 'color'">
              <label>Color</label>
              <div class="color-picker-row">
                <input 
                  type="color" 
                  :value="rgbToHex(localColor.r, localColor.g, localColor.b)"
                  @input="handleColorPicker($event)"
                  class="color-picker-input"
                />
                <div class="color-preview-large" :style="{ background: rgbColorString }"></div>
              </div>
              <div class="rgb-display">
                RGB({{ localColor.r }}, {{ localColor.g }}, {{ localColor.b }})
              </div>
            </div>
          </div>
        </div>
      </transition>
      
      <!-- System Settings Panel (Left side overlay) -->
      <transition name="fade">
        <div v-if="activeTab === 'system'" class="settings-panel left">
          <div class="panel-header">
            <h3>Device Settings</h3>
          </div>
          <div class="panel-body">
            <div class="info-group">
              <label>Firmware Version</label>
              <div class="info-value">{{ firmwareVersion || 'Unknown' }}</div>
            </div>
            
            <div class="info-group" v-if="performanceSettings">
              <label>Debounce Time</label>
              <div class="info-value">{{ (performanceSettings.debounceDown / 100).toFixed(1) }} ms</div>
            </div>
            
            <div class="info-group">
              <label>Active Profile</label>
              <div class="info-value">{{ profileInfo?.active ?? 1 }} / {{ profileInfo?.count ?? 4 }}</div>
            </div>

            <div class="divider"></div>
            
            <div class="info-group">
              <label>App Updates</label>
              <button class="action-button" @click="checkForUpdates" :disabled="isChecking">
                {{ isChecking ? 'Checking...' : 'Check for Updates' }}
              </button>
              <div class="update-status-text" v-if="updateStatus && !isChecking && !updateAvailable">{{ updateStatus }}</div>
              
              <div v-if="updateAvailable" class="update-available-box">
                <div class="update-info">New version {{ updateVersion }} available!</div>
                <button class="action-button primary" @click="installUpdate">
                  Download & Install
                </button>
              </div>
              <div v-if="!updateAvailable && updateStatus && updateStatus.includes('Error')" class="error-text">
                {{ updateStatus }}
              </div>
            </div>

            <div class="divider"></div>
            
            <button class="action-button danger">Reset to Default</button>
          </div>
        </div>
      </transition>

      <!-- Center Visualization -->
      <div class="visualization-area" :style="{ '--rgb-glow': rgbColorString }">
        <svg v-if="layout" :viewBox="`0 0 ${svgWidth} ${svgHeight}`" class="keyboard-svg-large">
          <!-- Base -->
          <rect x="0" y="0" :width="svgWidth" :height="svgHeight" rx="12" class="keyboard-base"/>
          
          <!-- Keys -->
          <g v-for="key in allKeys" :key="key.id" class="key-group">
            <rect 
              :x="key.x + boardPadding" 
              :y="key.y + boardPadding" 
              :width="key.w" 
              :height="key.h" 
              rx="4" 
              class="key"
              :style="{ 
                fill: getKeyColor(key.x, svgWidth), 
                filter: `drop-shadow(0 0 3px ${getKeyColor(key.x, svgWidth)})` 
              }"
            />
            <text 
              :x="key.x + boardPadding + key.w / 2" 
              :y="key.y + boardPadding + key.h / 2 + 4" 
              class="key-label"
            >{{ key.label }}</text>
          </g>
          
          <!-- Knob -->
          <g :transform="`translate(${598 + boardPadding}, ${16 + boardPadding})`">
            <circle cx="0" cy="0" r="16" class="knob-outer"/>
            <circle cx="0" cy="0" r="12" class="knob-inner"/>
          </g>
        </svg>
      </div>
    </main>
  </div>
</template>

<style scoped>
.keyboard-detail-view {
  display: flex;
  height: 100%;
  width: 100%;
  background: var(--bg-primary);
  color: var(--text-primary);
  overflow: hidden;
}

/* Sidebar */
.sidebar {
  width: 100px;
  background: var(--bg-secondary);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 24px 0;
  z-index: 10;
}

.sidebar-top {
  margin-bottom: 40px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
}

.btn-back-sidebar {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: 1px solid var(--border);
  border-radius: 8px;
  color: var(--text-muted);
  cursor: pointer;
  transition: all 0.2s;
}

.btn-back-sidebar:hover {
  border-color: var(--accent-primary);
  color: var(--text-primary);
  background: var(--bg-hover);
}

.btn-back-sidebar svg {
  width: 16px;
  height: 16px;
}

.device-thumb {
  width: 48px;
  height: 48px;
  color: var(--accent-primary);
  opacity: 0.8;
}

.device-meta {
  display: none; /* Hide for narrow sidebar, maybe show on hover */
}

.sidebar-nav {
  display: flex;
  flex-direction: column;
  gap: 16px;
  width: 100%;
}

.nav-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 12px 0;
  background: transparent;
  border: none;
  cursor: pointer;
  color: var(--text-muted);
  transition: all 0.2s;
  position: relative;
}

.nav-item::before {
  content: '';
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 3px;
  height: 0;
  background: var(--accent-primary);
  transition: height 0.2s;
  border-radius: 0 4px 4px 0;
}

.nav-item:hover {
  color: var(--text-primary);
}

.nav-item.active {
  color: var(--accent-primary);
}

.nav-item.active::before {
  height: 70%;
}

.icon-box {
  width: 24px;
  height: 24px;
}

.icon-box svg {
  width: 100%;
  height: 100%;
}

.nav-label {
  font-size: 10px;
  text-transform: uppercase;
  font-weight: 600;
  letter-spacing: 0.5px;
  text-align: center;
  padding: 0 4px;
}

/* Main Content Area */
.content-area {
  flex: 1;
  position: relative;
  display: flex;
  background: var(--bg-primary);
}

/* Settings Panel (Float over left side of content) */
.settings-panel {
  width: 300px;
  background: rgba(22, 22, 31, 0.95);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  z-index: 5;
  backdrop-filter: blur(10px);
}

.panel-header {
  height: 60px;
  display: flex;
  align-items: center;
  padding: 0 24px;
  border-bottom: 1px solid var(--border);
}

.panel-header h3 {
  font-size: 14px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 1px;
  color: var(--text-primary);
}

.panel-body {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 24px;
}

/* Preset List */
.preset-list {
  display: flex;
  flex-direction: column;
  gap: 4px;
  max-height: 300px;
  overflow-y: auto;
  padding-right: 8px;
}

.preset-item {
  text-align: left;
  padding: 10px 16px;
  background: transparent;
  border: 1px solid transparent;
  border-radius: 4px;
  color: var(--text-secondary);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.preset-item:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.preset-item.active {
  background: var(--bg-tertiary);
  border-color: var(--accent-primary);
  color: var(--accent-primary);
  font-weight: 500;
}

.divider {
  height: 1px;
  background: var(--border);
  width: 100%;
}

/* Controls */
.control-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.control-group label {
  font-size: 11px;
  text-transform: uppercase;
  color: var(--text-muted);
  font-weight: 600;
  letter-spacing: 0.5px;
}

.slider {
  -webkit-appearance: none;
  width: 100%;
  height: 4px;
  background: var(--bg-tertiary);
  border-radius: 2px;
  outline: none;
}

.slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: var(--accent-primary);
  cursor: pointer;
  border: 2px solid var(--bg-card);
  box-shadow: 0 0 0 1px var(--accent-primary);
}

.value-display {
  font-family: var(--font-mono);
  font-size: 12px;
  color: var(--text-primary);
  text-align: right;
}

/* Color Mode Toggle */
.color-mode-section {
  margin-bottom: 16px;
}

.color-mode-section > label {
  display: block;
  font-size: 11px;
  text-transform: uppercase;
  color: var(--text-muted);
  font-weight: 600;
  letter-spacing: 0.5px;
  margin-bottom: 12px;
}

.color-mode-toggle {
  display: flex;
  gap: 8px;
}

.mode-btn {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 12px 16px;
  background: var(--bg-tertiary);
  border: 2px solid var(--border);
  border-radius: 8px;
  color: var(--text-secondary);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.mode-btn:hover {
  background: var(--bg-hover);
  border-color: var(--text-muted);
}

.mode-btn.active {
  background: rgba(0, 212, 255, 0.1);
  border-color: var(--accent-primary);
  color: var(--accent-primary);
}

.mode-icon {
  font-size: 16px;
}

.dazzle-preview {
  margin-top: 12px;
  text-align: center;
}

.dazzle-gradient {
  height: 24px;
  border-radius: 6px;
  background: linear-gradient(to right, #ff0000, #ffff00, #00ff00, #00ffff, #0000ff);
  margin-bottom: 8px;
}

.dazzle-label {
  font-size: 11px;
  color: var(--text-muted);
}

/* Color Picker */
.color-section label {
  display: block;
  font-size: 11px;
  text-transform: uppercase;
  color: var(--text-muted);
  font-weight: 600;
  letter-spacing: 0.5px;
  margin-bottom: 12px;
}

.color-picker-row {
  display: flex;
  gap: 12px;
  align-items: center;
}

.color-picker-input {
  width: 60px;
  height: 60px;
  padding: 0;
  border: 2px solid var(--border);
  border-radius: 8px;
  cursor: pointer;
  background: transparent;
  appearance: none;
  -webkit-appearance: none;
}

.color-picker-input::-webkit-color-swatch-wrapper {
  padding: 4px;
}

.color-picker-input::-webkit-color-swatch {
  border-radius: 4px;
  border: none;
}

.color-picker-input:hover {
  border-color: var(--accent-primary);
}

.color-preview-large {
  flex: 1;
  height: 60px;
  border-radius: 8px;
  border: 1px solid var(--border);
  box-shadow: inset 0 0 20px rgba(0,0,0,0.5), 0 0 20px currentColor;
}

.rgb-display {
  margin-top: 8px;
  font-family: var(--font-mono);
  font-size: 12px;
  color: var(--text-muted);
  text-align: center;
}

/* Info Value */
.info-value {
  font-family: var(--font-mono);
  font-size: 13px;
  color: var(--text-primary);
  padding: 8px 12px;
  background: var(--bg-tertiary);
  border-radius: 4px;
  border: 1px solid var(--border);
}

.action-button {
  padding: 10px;
  border-radius: 4px;
  border: 1px solid var(--border);
  cursor: pointer;
  font-weight: 500;
  transition: all 0.2s;
}

.action-button.danger {
  background: rgba(239, 68, 68, 0.1);
  color: var(--error);
  border-color: rgba(239, 68, 68, 0.3);
}

.action-button.danger:hover {
  background: rgba(239, 68, 68, 0.2);
}

/* Visualization */
.visualization-area {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 40px;
  background: radial-gradient(circle at center, var(--bg-secondary) 0%, var(--bg-primary) 100%);
}

.keyboard-svg-large {
  width: 100%;
  max-width: 900px;
  height: auto;
  filter: drop-shadow(0 0 40px color-mix(in srgb, var(--rgb-glow) 20%, transparent));
  transition: filter 0.3s;
}

.keyboard-base {
  fill: #16161f;
  stroke: #2a2a3a;
  stroke-width: 2;
}

.key {
  stroke: #2a2a3a;
  stroke-width: 1;
  transition: all 0.3s;
}

.key:hover {
  stroke: var(--accent-primary);
  filter: brightness(1.2) !important;
}

.key-label {
  fill: #8a8a9a;
  font-family: var(--font-primary);
  font-size: 10px; /* Slightly larger for detail view */
  text-anchor: middle;
  pointer-events: none;
}

.knob-outer {
  fill: #2a2a3a;
  stroke: #3a3a4a;
  stroke-width: 2;
}

.knob-inner {
  fill: #16161f;
}

/* Direction Controls */
.direction-controls {
  display: flex;
  gap: 8px;
  justify-content: space-between;
}

.dir-btn {
  flex: 1;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-tertiary);
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--text-secondary);
  font-size: 16px;
  cursor: pointer;
  transition: all 0.2s;
}

.dir-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.dir-btn.active {
  background: rgba(0, 212, 255, 0.15);
  border-color: var(--accent-primary);
  color: var(--accent-primary);
}

/* Animations */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
  transform: translateX(-10px);
}

.update-status-text {
  font-size: 12px;
  color: var(--text-secondary);
  margin-top: 8px;
}

.update-available-box {
  margin-top: 12px;
  padding: 12px;
  background: rgba(0, 212, 255, 0.1);
  border: 1px solid var(--accent-primary);
  border-radius: 4px;
}

.update-info {
  font-size: 12px;
  color: var(--accent-primary);
  margin-bottom: 8px;
  font-weight: 500;
}

.error-text {
  font-size: 12px;
  color: var(--error);
  margin-top: 8px;
}

.action-button.primary {
  background: var(--accent-primary);
  color: var(--bg-primary);
  border: none;
}

.action-button.primary:hover {
  background: color-mix(in srgb, var(--accent-primary), white 10%);
}
</style>
