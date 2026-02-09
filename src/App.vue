<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import TitleBar from "./components/TitleBar.vue";
import DeviceList from "./components/DeviceList.vue";
import LogPanel from "./components/LogPanel.vue";
import SettingsDrawer from "./components/SettingsDrawer.vue";
import AppSettings from "./components/AppSettings.vue";

import { useWindow } from "./composables/useWindow";
import { useDevice } from "./composables/useDevice";
import { useAppAutoSwitch } from "./composables/useAppAutoSwitch";

// Composables
const { 
  devices, 
  selectedDevice, 
  isConnected, 
  rgbSettings, 
  performanceSettings, 
  profileInfo, 
  firmwareVersion,
  logEntries, 
  addLog, 
  clearLogs, 
  detectDevices, 
  performHandshake, 
  startDevicePolling 
} = useDevice();

const { updateMaximizeState, appWindow } = useWindow();

const { activeApp } = useAppAutoSwitch();

// UI State
const showSettings = ref(false); // For Device Settings
const showLogs = ref(false);
const viewMode = ref<'grid' | 'list'>('grid');
const activeTab = ref<'devices' | 'profiles' | 'settings'>('devices');
const isLightOn = ref(true);

// Sync isLightOn with actual brightness from device
watch(rgbSettings, (newSettings) => {
  if (newSettings) {
    isLightOn.value = newSettings.brightness > 0;
  }
}, { immediate: true });

// Lifecycle
onMounted(async () => {
  await updateMaximizeState();
  await appWindow.listen('tauri://resize', updateMaximizeState);
  await detectDevices();
  
  if (devices.value.length > 0) {
    // Auto-connect to first device found
    await performHandshake(devices.value[0]);
  }
  
  startDevicePolling();
});

// Bridge functions for UI events
function handleScan() {
  detectDevices();
}

function handleSelectDevice(device: string) {
  if (selectedDevice.value !== device) {
    performHandshake(device);
  }
}

function handleToggleLight() {
  // Logic to toggle light state - sending 0 brightness or restoring
  // This logic was previously in App.vue, we should ideally move it to useDevice or keep it here if simple
  // For now, let's just toggle the UI state as the actual device command implies we have the params
  isLightOn.value = !isLightOn.value;
  // TODO: implement actual toggle using invoke 'akko_set_rgb_settings' with 0 brightness
  // For proper implementation this should probably be in useDevice or a specific action
}

</script>

<style>
.main-content.no-padding {
  padding: 0;
}
</style>

<template>
  <div class="app">
    <!-- Top Navigation Bar -->
    <TitleBar 
      :activeTab="activeTab"
      :activeProfileName="profileInfo ? `Profile ${profileInfo.active}` : undefined"
      :activeApp="activeApp"
      :showLogs="showLogs"
      @update:activeTab="activeTab = $event"
      @toggleLogs="showLogs = !showLogs"
      @openProfileSettings="showSettings = true" 
    />

    <!-- Main Content Area -->
    <main class="main-content" :class="{ 'no-padding': activeTab === 'settings' }">
      
      <!-- Device View -->
      <template v-if="activeTab === 'devices'">
        <!-- View Toggle (Grid/List) -->
        <div class="view-toggle">
          <button 
            class="toggle-btn" 
            :class="{ active: viewMode === 'grid' }" 
            @click="viewMode = 'grid'"
            title="Grid View"
          >
            <svg viewBox="0 0 24 24" fill="currentColor">
              <rect x="3" y="3" width="7" height="7" rx="1"/>
              <rect x="14" y="3" width="7" height="7" rx="1"/>
              <rect x="3" y="14" width="7" height="7" rx="1"/>
              <rect x="14" y="14" width="7" height="7" rx="1"/>
            </svg>
          </button>
          <button 
            class="toggle-btn" 
            :class="{ active: viewMode === 'list' }" 
            @click="viewMode = 'list'"
            title="List View"
          >
            <svg viewBox="0 0 24 24" fill="currentColor">
              <rect x="3" y="4" width="18" height="4" rx="1"/>
              <rect x="3" y="10" width="18" height="4" rx="1"/>
              <rect x="3" y="16" width="18" height="4" rx="1"/>
            </svg>
          </button>
        </div>

        <!-- Device List/Grid -->
        <DeviceList
          :devices="devices"
          :viewMode="viewMode"
          :isConnected="isConnected"
          :selectedDevice="selectedDevice"
          :rgbSettings="rgbSettings"
          :isLightOn="isLightOn"
          @scan="handleScan"
          @select="handleSelectDevice"
          @toggleLight="handleToggleLight"
          @openSettings="showSettings = true"
        />
      </template>

      <!-- App Settings View -->
      <AppSettings 
        v-if="activeTab === 'settings'"
      />


      <!-- Settings Panel -->
      <SettingsDrawer
        :show="showSettings"
        :selectedDevice="selectedDevice"
        :rgbSettings="rgbSettings"
        :performanceSettings="performanceSettings"
        :profileInfo="profileInfo"
        :firmwareVersion="firmwareVersion"
        @close="showSettings = false"
        @update:rgbSettings="(val: any) => rgbSettings = val"
        @log="addLog"
      />

      <!-- Log Panel -->
      <LogPanel
        :show="showLogs"
        :logs="logEntries"
        @close="showLogs = false"
        @clear="clearLogs"
      />
    </main>
  </div>
</template>
