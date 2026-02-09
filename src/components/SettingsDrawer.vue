<script setup lang="ts">
import KeyboardView from './KeyboardView.vue';

defineProps<{
  show: boolean;
  selectedDevice: string | null;
  rgbSettings: any;
  performanceSettings: any;
  profileInfo: any;
  firmwareVersion: string;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'update:rgbSettings', val: any): void;
  (e: 'log', type: string, data: string): void;
}>();
</script>

<template>
  <transition name="slide-up">
    <div v-if="show" class="settings-overlay" @click="emit('close')">
      <div class="settings-drawer" @click.stop>
        <!-- <div class="drawer-handle" @click="emit('close')"></div> -->
        <KeyboardView
          v-if="selectedDevice"
          :modelName="selectedDevice"
          :rgbSettings="rgbSettings"
          :performanceSettings="performanceSettings"
          :profileInfo="profileInfo"
          :firmwareVersion="firmwareVersion"
          @update:rgbSettings="(val) => emit('update:rgbSettings', val)"
          @log="(type, data) => emit('log', type, data)"
          @close="emit('close')"
        />
      </div>
    </div>
  </transition>
</template>

<style scoped>
.drawer-handle {
  width: 40px;
  height: 4px;
  background: rgba(255, 255, 255, 0.2);
  border-radius: 2px;
  margin: 10px auto;
  cursor: pointer;
}
</style>
