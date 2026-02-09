<script setup lang="ts">
import { useWindow } from '../composables/useWindow';

const { isMaximized, minimizeWindow, toggleMaximize, closeWindow } = useWindow();

const props = defineProps<{
  activeTab: 'devices' | 'profiles' | 'settings';
  activeProfileName?: string;
  activeApp?: string;
  showLogs: boolean;
}>();

const emit = defineEmits<{
  (e: 'update:activeTab', value: 'devices' | 'profiles' | 'settings'): void;
  (e: 'toggleLogs'): void;
  (e: 'openProfileSettings'): void;
}>();
</script>

<template>
  <nav class="top-nav" data-tauri-drag-region>
    <div class="nav-left">
      <div class="logo">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="2" y="6" width="20" height="12" rx="2" />
          <line x1="6" y1="10" x2="6" y2="10" />
          <line x1="10" y1="10" x2="10" y2="10" />
          <line x1="14" y1="10" x2="14" y2="10" />
          <line x1="18" y1="10" x2="18" y2="10" />
          <line x1="8" y1="14" x2="16" y2="14" />
        </svg>
      </div>
      <a 
        class="nav-link" 
        :class="{ active: activeTab === 'devices' }" 
        @click="emit('update:activeTab', 'devices')"
        @mousedown.stop
        href="#"
      >Devices</a>
      <a 
        class="nav-link" 
        :class="{ active: activeTab === 'profiles' }" 
        @click="emit('update:activeTab', 'profiles')"
        @mousedown.stop
        href="#"
      >Profiles</a>
    </div>
    
    <div class="nav-right">
      <!-- Profile / Active App Dropdown -->
      <div class="profile-dropdown" @mousedown.stop @click="emit('openProfileSettings')">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="2" y="3" width="20" height="14" rx="2"/>
          <path d="M8 21h8M12 17v4"/>
        </svg>
        <div class="profile-info">
          <span class="active-app" v-if="activeApp">{{ activeApp }}</span>
          <span class="profile-name">{{ activeProfileName || 'Default Profile' }}</span>
        </div>
        <svg class="chevron" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M6 9l6 6 6-6"/>
        </svg>
      </div>

      <button 
        class="nav-icon-btn" 
        :class="{ active: activeTab === 'settings' }"
        @click="emit('update:activeTab', 'settings')" 
        @mousedown.stop
      >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="3"/>
          <path d="M19.4 15a1.65 1.65 0 00.33 1.82l.06.06a2 2 0 010 2.83 2 2 0 01-2.83 0l-.06-.06a1.65 1.65 0 00-1.82-.33 1.65 1.65 0 00-1 1.51V21a2 2 0 01-4 0v-.09A1.65 1.65 0 009 19.4a1.65 1.65 0 00-1.82.33l-.06.06a2 2 0 01-2.83 0 2 2 0 010-2.83l.06-.06a1.65 1.65 0 00.33-1.82 1.65 1.65 0 00-1.51-1H3a2 2 0 010-4h.09A1.65 1.65 0 004.6 9a1.65 1.65 0 00-.33-1.82l-.06-.06a2 2 0 010-2.83 2 2 0 012.83 0l.06.06a1.65 1.65 0 001.82.33H9a1.65 1.65 0 001-1.51V3a2 2 0 114 0v.09a1.65 1.65 0 001 1.51 1.65 1.65 0 001.82-.33l.06-.06a2 2 0 012.83 0 2 2 0 010 2.83l-.06.06a1.65 1.65 0 00-.33 1.82V9a1.65 1.65 0 001.51 1H21a2 2 0 010 4h-.09a1.65 1.65 0 00-1.51 1z"/>
        </svg>
      </button>

      <button 
        class="nav-icon-btn" 
        :class="{ active: showLogs }" 
        @click="emit('toggleLogs')" 
        @mousedown.stop 
        title="View Logs"
      >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
          <polyline points="14 2 14 8 20 8"/>
          <line x1="16" y1="13" x2="8" y2="13"/>
          <line x1="16" y1="17" x2="8" y2="17"/>
          <polyline points="10 9 9 9 8 9"/>
        </svg>
      </button>
      
      <!-- Window Controls -->
      <div class="window-controls">
        <button class="window-btn minimize" @click="minimizeWindow" @mousedown.stop title="Minimize">
          <svg viewBox="0 0 12 12" fill="currentColor">
            <rect x="2" y="5.5" width="8" height="1"/>
          </svg>
        </button>
        <button class="window-btn maximize" @click="toggleMaximize" @mousedown.stop :title="isMaximized ? 'Restore' : 'Maximize'">
          <svg v-if="!isMaximized" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1">
            <rect x="2.5" y="2.5" width="7" height="7"/>
          </svg>
          <svg v-else viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1">
            <path d="M3.5 3.5V1.5h7v7h-2" />
            <rect x="1.5" y="3.5" width="7" height="7" fill="none" />
          </svg>
        </button>
        <button class="window-btn close" @click="closeWindow" @mousedown.stop title="Close">
          <svg viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M2 2l8 8M10 2l-8 8"/>
          </svg>
        </button>
      </div>
    </div>
  </nav>
</template>

<style scoped>
.profile-dropdown {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 6px 12px;
  background: rgba(255, 255, 255, 0.05); /* Very subtle background */
  border-radius: 6px;
  cursor: pointer;
  border: 1px solid rgba(255, 255, 255, 0.1);
  transition: all 0.2s;
  min-width: 160px;
  justify-content: space-between;
}

.profile-dropdown:hover {
  background: rgba(255, 255, 255, 0.08);
  border-color: rgba(255, 255, 255, 0.2);
}

.profile-info {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  line-height: 1.2;
}

.active-app {
  font-size: 0.75rem;
  color: var(--accent-blue);
  font-weight: 600;
  text-transform: uppercase;
}

.profile-name {
  font-size: 0.9rem;
  font-weight: 500;
}

.chevron {
  width: 14px;
  height: 14px;
  opacity: 0.6;
}
</style>
