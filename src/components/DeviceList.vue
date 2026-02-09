<script setup lang="ts">
import KeyboardSVG from './KeyboardSVG.vue';

defineProps<{
  devices: string[];
  viewMode: 'grid' | 'list';
  isConnected: boolean;
  selectedDevice: string | null;
  rgbSettings: any;  // Using any for flexibility or import the type if available
  isLightOn: boolean;
}>();

const emit = defineEmits<{
  (e: 'scan'): void;
  (e: 'select', device: string): void;
  (e: 'toggleLight'): void;
  (e: 'openSettings'): void;
}>();
</script>

<template>
  <div class="device-showcase" :class="viewMode">
    <!-- No device state -->
    <div v-if="devices.length === 0" class="no-device-card">
      <div class="scanning-indicator" v-if="!isConnected"> <!-- Assuming isConnecting logic handled in parent passed as !isConnected effectively or separate prop -->
         <!-- Simplify for now, logic passed down -->
        <p>Scanning...</p>
      </div>
      <div v-else>
        <svg class="no-device-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
          <rect x="2" y="6" width="20" height="12" rx="2" />
          <path d="M6 10h.01M10 10h.01M14 10h.01M18 10h.01" stroke-linecap="round" />
          <line x1="8" y1="14" x2="16" y2="14" stroke-linecap="round" />
        </svg>
        <p>No device connected</p>
        <button class="btn-scan-large" @click="emit('scan')">Scan Devices</button>
      </div>
    </div>

    <!-- Device List/Grid -->
    <div 
      v-else 
      v-for="device in devices" 
      :key="device" 
      class="device-card-ghub"
      :class="{ 'active': selectedDevice === device }"
      @click="emit('select', device); emit('openSettings')"
    >
      <div class="card-header">
        <h2 class="device-name">{{ device }}</h2>
        <div class="connection-icon" title="Wired Connection">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M10 7.5v-2a2 2 0 0 1 2-2h0a2 2 0 0 1 2 2v2" />
            <path d="M8.5 7.5h7" />
            <path d="M12 7.5v6" />
            <path d="M7 13.5h10v4h-10z" />
            <path d="M10 17.5v4" />
            <path d="M14 17.5v4" />
          </svg>
        </div>
      </div>

      <!-- Keyboard Visualization -->
      <KeyboardSVG 
        :modelName="device"
        :rgbSettings="rgbSettings"
        :isLightOn="isLightOn"
      />

      <!-- Quick Actions -->
      <div class="card-actions">
        <button class="action-btn" :class="{ active: isLightOn }" @click.stop="emit('toggleLight')" title="Toggle LEDs">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="5"/>
            <path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42"/>
          </svg>
        </button>
        <button class="action-btn" @click.stop="emit('openSettings')" title="Open Settings">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/>
          </svg>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.device-showcase.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 20px;
}

.device-showcase.list {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

/* List view: make cards horizontal and compact */
.device-showcase.list .device-card-ghub {
  display: flex;
  flex-direction: row;
  align-items: center;
  padding: 15px 20px;
  max-width: 100%;
}

.device-showcase.list .card-header {
  flex-shrink: 0;
  width: 200px;
}

.device-showcase.list .connection-icon {
  color: var(--text-muted);
}

.connection-icon svg {
  width: 24px;
  height: 24px;
}

.device-showcase.list .keyboard-preview {
  flex: 1;
  max-width: 400px;
  margin: 0 20px;
}

.device-showcase.list .keyboard-preview svg {
  height: 80px;
}

.device-showcase.list .card-actions {
  flex-shrink: 0;
  display: flex;
  gap: 10px;
}

/* Grid view: keep cards vertical (default) */
.device-showcase.grid .device-card-ghub {
  display: flex;
  flex-direction: column;
}
</style>
