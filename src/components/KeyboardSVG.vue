<script setup lang="ts">
import { computed, watch } from 'vue';

const props = defineProps<{
  modelName: string;
  rgbSettings?: {
    mode: number;
    brightness: number;
    color: { r: number; g: number; b: number };
  } | null;
  isLightOn?: boolean;
}>();

// Debug logging
watch(() => props.rgbSettings, (newVal) => {
  console.log('[KeyboardSVG] RGB Settings changed:', newVal);
}, { immediate: true, deep: true });

watch(() => props.isLightOn, (newVal) => {
  console.log('[KeyboardSVG] isLightOn changed:', newVal);
}, { immediate: true });

// Computed RGB color string
const rgbColorString = computed(() => {
  if (!props.rgbSettings) return '#B4B4B4';
  const { r, g, b } = props.rgbSettings.color;
  return `rgb(${r}, ${g}, ${b})`;
});

// Determine keyboard layout based on model
const keyboardLayout = computed(() => {
  const model = props.modelName.toLowerCase();
  
  // MOD007B-HE: 75% layout with knob
  if (model.includes('mod007b')) {
    return 'mod007b';
  }
  
  // Default to standard layout
  return 'standard';
});
</script>

<template>
  <div 
    class="keyboard-preview" 
    :class="{ 
      dazzle: rgbSettings?.mode === 7 && rgbSettings?.brightness > 0, 
      off: !isLightOn 
    }"
    :style="{ '--rgb-glow': rgbColorString }"
  >
    <!-- MOD007B-HE: 75% layout with knob -->
    <svg v-if="keyboardLayout === 'mod007b'" viewBox="0 0 620 256" class="keyboard-svg">
      <defs>
        <linearGradient id="dazzle-gradient" x1="0%" y1="0%" x2="100%" y2="0%">
          <stop offset="0%" style="stop-color:#ff0000"/>
          <stop offset="25%" style="stop-color:#ffff00"/>
          <stop offset="50%" style="stop-color:#00ff00"/>
          <stop offset="75%" style="stop-color:#00ffff"/>
          <stop offset="100%" style="stop-color:#0000ff"/>
        </linearGradient>
      </defs>
      <rect x="0" y="0" width="620" height="256" rx="12" class="keyboard-base"/>
      
      <g transform="translate(10, 10)">
        <!-- Function row -->
        <rect class="key glow" x="0" y="0" width="36" height="32" rx="4"/>
        <rect class="key glow" x="46" y="0" width="32" height="32" rx="4"/>
        <rect class="key glow" x="82" y="0" width="32" height="32" rx="4"/>
        <rect class="key glow" x="118" y="0" width="32" height="32" rx="4"/>
        <rect class="key glow" x="154" y="0" width="32" height="32" rx="4"/>
        <rect class="key glow" x="196" y="0" width="32" height="32" rx="4"/>
        <rect class="key glow" x="232" y="0" width="32" height="32" rx="4"/>
        <rect class="key glow" x="268" y="0" width="32" height="32" rx="4"/>
        <rect class="key glow" x="304" y="0" width="32" height="32" rx="4"/>
        <rect class="key glow" x="346" y="0" width="32" height="32" rx="4"/>
        <rect class="key glow" x="382" y="0" width="32" height="32" rx="4"/>
        <rect class="key glow" x="418" y="0" width="32" height="32" rx="4"/>
        <rect class="key glow" x="454" y="0" width="32" height="32" rx="4"/>
        <rect class="key glow" x="496" y="0" width="36" height="32" rx="4"/>
        
        <!-- Knob (distinctive feature of MOD007B) -->
        <circle cx="570" cy="16" r="20" class="knob"/>
        
        <!-- Number row -->
        <rect class="key glow" x="0" y="40" width="36" height="36" rx="4"/>
        <rect class="key glow" x="40" y="40" width="500" height="36" rx="4"/>
        <rect class="key glow" x="544" y="40" width="32" height="36" rx="4"/>
        <rect class="key glow" x="580" y="40" width="20" height="36" rx="4"/>
        
        <!-- QWERTY row -->
        <rect class="key glow" x="0" y="80" width="52" height="36" rx="4"/>
        <rect class="key glow" x="56" y="80" width="484" height="36" rx="4"/>
        <rect class="key glow" x="544" y="80" width="56" height="36" rx="4"/>
        
        <!-- ASDF row -->
        <rect class="key glow" x="0" y="120" width="60" height="36" rx="4"/>
        <rect class="key glow" x="64" y="120" width="436" height="36" rx="4"/>
        <rect class="key glow" x="504" y="120" width="72" height="36" rx="4"/>
        <rect class="key glow" x="580" y="120" width="20" height="36" rx="4"/>
        
        <!-- ZXCV row -->
        <rect class="key glow" x="0" y="160" width="76" height="36" rx="4"/>
        <rect class="key glow" x="80" y="160" width="396" height="36" rx="4"/>
        <rect class="key glow" x="480" y="160" width="60" height="36" rx="4"/>
        <rect class="key glow" x="544" y="160" width="32" height="36" rx="4"/>
        <rect class="key glow" x="580" y="160" width="20" height="36" rx="4"/>
        
        <!-- Bottom row -->
        <rect class="key glow" x="0" y="200" width="44" height="36" rx="4"/>
        <rect class="key glow" x="48" y="200" width="44" height="36" rx="4"/>
        <rect class="key glow" x="96" y="200" width="44" height="36" rx="4"/>
        <rect class="key glow" x="144" y="200" width="224" height="36" rx="4"/>
        <rect class="key glow" x="372" y="200" width="44" height="36" rx="4"/>
        <rect class="key glow" x="420" y="200" width="36" height="36" rx="4"/>
        <rect class="key glow" x="460" y="200" width="44" height="36" rx="4"/>
        <rect class="key glow" x="508" y="200" width="32" height="36" rx="4"/>
        <rect class="key glow" x="544" y="200" width="32" height="36" rx="4"/>
        <rect class="key glow" x="580" y="200" width="20" height="36" rx="4"/>
      </g>
    </svg>

    <!-- Standard layout (fallback) -->
    <svg v-else viewBox="0 0 600 200" class="keyboard-svg">
      <defs>
        <linearGradient id="dazzle-gradient-std" x1="0%" y1="0%" x2="100%" y2="0%">
          <stop offset="0%" style="stop-color:#ff0000"/>
          <stop offset="25%" style="stop-color:#ffff00"/>
          <stop offset="50%" style="stop-color:#00ff00"/>
          <stop offset="75%" style="stop-color:#00ffff"/>
          <stop offset="100%" style="stop-color:#0000ff"/>
        </linearGradient>
      </defs>
      <rect x="0" y="0" width="600" height="200" rx="12" class="keyboard-base"/>
      
      <g transform="translate(10, 10)">
        <!-- Simplified standard keyboard -->
        <rect class="key glow" x="0" y="0" width="580" height="30" rx="4"/>
        <rect class="key glow" x="0" y="35" width="580" height="30" rx="4"/>
        <rect class="key glow" x="0" y="70" width="580" height="30" rx="4"/>
        <rect class="key glow" x="0" y="105" width="580" height="30" rx="4"/>
        <rect class="key glow" x="0" y="140" width="580" height="30" rx="4"/>
      </g>
    </svg>
  </div>
</template>

<style scoped>
.keyboard-preview {
  position: relative;
  width: 100%;
  padding: 20px;
  background: linear-gradient(145deg, #1a1a1a, #0d0d0d);
  border-radius: 16px;
  overflow: hidden;
}

.keyboard-svg {
  width: 100%;
  height: auto;
  display: block;
}

.keyboard-base {
  fill: #1a1a1a;
  stroke: #333;
  stroke-width: 2;
}

.key {
  fill: #2a2a2a;
  stroke: #444;
  stroke-width: 1;
  transition: all 0.3s ease;
}

.key.glow {
  filter: drop-shadow(0 0 3px var(--rgb-glow, #B4B4B4));
}

.knob {
  fill: #333;
  stroke: #555;
  stroke-width: 2;
  filter: drop-shadow(0 0 5px var(--rgb-glow, #B4B4B4));
}

/* Dazzle mode animation */
/* .keyboard-preview.dazzle .key.glow {
  animation: dazzle-pulse 2s ease-in-out infinite;
  filter: drop-shadow(0 0 8px var(--rgb-glow)) 
          drop-shadow(0 0 15px var(--rgb-glow))
          drop-shadow(0 0 20px var(--rgb-glow));
} */

.keyboard-preview.dazzle .knob {
  animation: dazzle-pulse 2s ease-in-out infinite;
  filter: drop-shadow(0 0 10px var(--rgb-glow)) 
          drop-shadow(0 0 20px var(--rgb-glow));
}

/* Off state */
.keyboard-preview.off .key.glow,
.keyboard-preview.off .knob {
  filter: none;
  opacity: 0.5;
}

@keyframes dazzle-pulse {
  0%, 100% { opacity: 0.8; }
  50% { opacity: 1; }
}
</style>
