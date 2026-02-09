<script setup lang="ts">
defineProps<{
  show: boolean;
  logs: { type: string; data: string; time: string }[];
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'clear'): void;
}>();
</script>

<template>
  <transition name="slide-left">
    <div v-if="show" class="log-panel-overlay" @click="emit('close')">
      <div class="log-panel" @click.stop>
        <div class="log-header">
          <h3>System Logs</h3>
          <div class="log-actions">
            <button class="btn-clear-logs" @click="emit('clear')" title="Clear Logs">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="3 6 5 6 21 6"/>
                <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
              </svg>
            </button>
            <button class="btn-close-panel" @click="emit('close')" title="Close">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <line x1="18" y1="6" x2="6" y2="18"/>
                <line x1="6" y1="6" x2="18" y2="18"/>
              </svg>
            </button>
          </div>
        </div>
        <div class="log-body">
          <div v-if="logs.length === 0" class="no-logs">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
              <polyline points="14 2 14 8 20 8"/>
            </svg>
            <p>No logs yet</p>
          </div>
          <div v-else class="log-entries">
            <div 
              v-for="(entry, index) in logs" 
              :key="index" 
              class="log-entry"
              :class="`log-${entry.type}`"
            >
              <span class="log-time">{{ entry.time }}</span>
              <span class="log-type">{{ entry.type.toUpperCase() }}</span>
              <span class="log-data">{{ entry.data }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </transition>
</template>
