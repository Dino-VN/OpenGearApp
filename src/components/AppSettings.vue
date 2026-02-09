<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { enable, disable, isEnabled } from '@tauri-apps/plugin-autostart';
import { useUpdater } from '../composables/useUpdater';

// defineProps<{ show: boolean }>(); // Removed
// emit('close') Removed

const activeTab = ref<'general' | 'updates'>('general');
const autoStartEnabled = ref(false);

const { 
  updateStatus, 
  currentVersion,
  isChecking, 
  checkForUpdates 
} = useUpdater();

onMounted(async () => {
  try {
    autoStartEnabled.value = await isEnabled();
  } catch (e) {
    console.error('Failed to check autostart status:', e);
  }
});

async function toggleAutoStart() {
  try {
    if (autoStartEnabled.value) {
      await enable();
    } else {
      await disable();
    }
  } catch (e) {
    console.error('Failed to toggle autostart:', e);
    // Revert intent if failed (though v-model updates eagerly)
    autoStartEnabled.value = !autoStartEnabled.value;
  }
}
</script>

<template>
  <!-- Removed transition and overlay wrapper, now just the component content -->
    <div class="app-settings">
      <div class="sidebar">
        <div 
          class="sidebar-item" 
          :class="{ active: activeTab === 'general' }"
          @click="activeTab = 'general'"
        >
          <div class="icon">
              <svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 -960 960 960" width="24px" fill="currentColor"><path d="M480-120q-137 0-240-86.5T116-434l123-52q20 92 93.5 149T480-280q83 0 141.5-58.5T680-480q0-83-58.5-141.5T480-680q-50 0-92 23.5T324-592l-108-72q65-95 163-145.5T480-860q75 0 140.5 28.5t114 77q48.5 48.5 77 114T840-480q0 75-28.5 140.5t-77 114q-48.5 48.5-114 77T480-120Zm-60-264v-192h120v192H420Z"/></svg>
          </div>
          <span>General</span>
        </div>
        <div 
          class="sidebar-item" 
          :class="{ active: activeTab === 'updates' }"
          @click="activeTab = 'updates'"
        >
          <div class="icon">
              <svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 -960 960 960" width="24px" fill="currentColor"><path d="M480-80q-82 0-155-31.5t-127.5-86Q143-252 111.5-325T80-480q0-83 31.5-156t86-127Q252-817 325-848.5T480-880q83 0 156 31.5T763-763l-56 56q-39-39-90.5-61T480-790q-129 0-219.5 90.5T170-480q0 129 90.5 219.5T480-170q129 0 219.5-90.5T790-480q0-61-23-116.5T705-697l66-64q52 50 80.5 117T880-480q0 82-31.5 155t-86 127.5q-54.5 54.5-127 86T480-80Zm-40-240v-280h80v280h-80Z"/></svg>
          </div>
          <span>Updates</span>
        </div>
        
         <!-- Spacer -->
         <div style="flex-grow: 1;"></div>
  
         <!-- Removed Close Button -->
      </div>
  
      <div class="content-area">
            <div v-if="activeTab === 'general'" class="tab-content">
              <h2>General</h2>
              
              <div class="setting-item">
                <div class="setting-info">
                  <div class="setting-title">Start with Windows</div>
                  <div class="setting-desc">Automatically launch the application when you log in</div>
                </div>
                <label class="switch">
                  <input type="checkbox" v-model="autoStartEnabled" @change="toggleAutoStart">
                  <span class="slider round"></span>
                </label>
              </div>
            </div>
      
            <div v-if="activeTab === 'updates'" class="tab-content">
              <h2>Updates</h2>
              
              <div class="update-card">
                <div class="current-version">
                  <div class="label">Current Software version</div>
                  <div class="version-number">{{ currentVersion || 'Loading...' }}</div>
                </div>
                
                <button class="check-update-btn" @click="checkForUpdates" :disabled="isChecking">
                  {{ isChecking ? 'CHECKING...' : 'CHECK FOR UPDATE' }}
                </button>
              </div>
              
              <div class="update-status" v-if="updateStatus">
                  {{ updateStatus }}
              </div>
      
              <div class="setting-item">
                <div class="setting-info">
                  <div class="setting-title">Enable Automatic Updates</div>
                </div>
                <label class="switch">
                  <input type="checkbox" checked>
                  <span class="slider round"></span>
                </label>
              </div>
            </div>
          </div>
        </div>
</template>

<style scoped>
.app-settings {
  display: flex;
  height: 100%;
  width: 100%;
  background-color: #000;
  color: #fff;
}

.sidebar {
  width: 250px;
  background-color: #111;
  display: flex;
  flex-direction: column;
  padding: 20px 0;
  border-right: 1px solid #222;
}

.sidebar-item {
  display: flex;
  align-items: center;
  padding: 15px 24px;
  cursor: pointer;
  color: #aaa;
  transition: all 0.2s;
}

.sidebar-item:hover {
  color: #fff;
  background-color: rgba(255, 255, 255, 0.05);
}

.sidebar-item.active {
  color: #fff;
  background-color: rgba(30, 30, 30, 0.5);
  border-left: 3px solid #00aaff;
}

.icon {
  margin-right: 12px;
  display: flex;
  align-items: center;
}

.content-area {
  flex: 1;
  padding: 40px;
  overflow-y: auto;
}

h2 {
  font-size: 24px;
  margin-bottom: 30px;
  font-weight: 600;
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 0;
  border-bottom: 1px solid #222;
}

.setting-title {
  font-size: 16px;
  font-weight: 500;
  margin-bottom: 5px;
}

.setting-desc {
  font-size: 14px;
  color: #888;
}

/* Toggle Switch */
.switch {
  position: relative;
  display: inline-block;
  width: 50px;
  height: 24px;
}

.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #333;
  transition: .4s;
}

.slider:before {
  position: absolute;
  content: "";
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  transition: .4s;
}

input:checked + .slider {
  background-color: #00aaff;
}

input:checked + .slider:before {
  transform: translateX(26px);
}

.slider.round {
  border-radius: 24px;
}

.slider.round:before {
  border-radius: 50%;
}

/* Update Card */
.update-card {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 30px;
}

.current-version .label {
  font-size: 14px;
  margin-bottom: 8px;
  font-weight: 600;
}

.current-version .version-number {
  font-size: 14px;
  color: #888;
}

.check-update-btn {
  background-color: #00aaff;
  color: #fff;
  border: none;
  padding: 12px 24px;
  font-weight: 600;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.check-update-btn:hover {
  background-color: #0088cc;
}

.check-update-btn:disabled {
  background-color: #444;
  cursor: not-allowed;
}

.update-status {
    margin-bottom: 20px;
    color: #00aaff;
    font-size: 14px;
}

.no-padding {
    padding: 0;
}

/* App Settings Container */
.app-settings {
  display: flex;
  height: 100%;
  width: 100%;
  background: var(--bg-primary);
  color: var(--text-primary);
  overflow: hidden;
}
</style>