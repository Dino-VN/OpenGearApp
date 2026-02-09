// LED effect modes for Akko keyboards
// Based on real Akko Cloud web interface

export interface LedMode {
    id: number;
    name: string;
    hasColor: boolean;      // supports custom color
    hasSpeed: boolean;      // supports speed adjustment
    hasDirection: boolean;  // supports direction
}

export const LED_MODES: LedMode[] = [
    { id: 0, name: "Off", hasColor: false, hasSpeed: false, hasDirection: false },
    { id: -1, name: "Static", hasColor: true, hasSpeed: false, hasDirection: false },
    { id: 4, name: "Drift", hasColor: true, hasSpeed: true, hasDirection: true },
    { id: 5, name: "Waves Ripple", hasColor: true, hasSpeed: true, hasDirection: false },
    { id: 6, name: "Stars Twinkle", hasColor: true, hasSpeed: true, hasDirection: false },
    { id: 7, name: "Steady Stream", hasColor: true, hasSpeed: true, hasDirection: false },
    { id: 8, name: "Like Shadows", hasColor: true, hasSpeed: true, hasDirection: false },
    { id: 9, name: "Peaks rising one after another", hasColor: true, hasSpeed: true, hasDirection: false },
    { id: 10, name: "Sine Wave", hasColor: true, hasSpeed: true, hasDirection: false },
    { id: 11, name: "Flowing Spring", hasColor: true, hasSpeed: true, hasDirection: false },
    { id: 12, name: "Flowers Blooming", hasColor: true, hasSpeed: true, hasDirection: false },
    { id: 14, name: "Laser", hasColor: true, hasSpeed: true, hasDirection: false },
    { id: 15, name: "Peak Turn", hasColor: true, hasSpeed: true, hasDirection: false },
    { id: 16, name: "Colorful vertical and horizontal", hasColor: true, hasSpeed: true, hasDirection: false },
    { id: 17, name: "Snow", hasColor: true, hasSpeed: true, hasDirection: false },
    { id: 18, name: "Meteor", hasColor: true, hasSpeed: true, hasDirection: false },
    { id: 19, name: "Light Trace", hasColor: true, hasSpeed: true, hasDirection: false },
    { id: 20, name: "Dynamic Breathing", hasColor: true, hasSpeed: true, hasDirection: false },
    { id: 21, name: "Spectrum Cycle", hasColor: true, hasSpeed: true, hasDirection: false },
];

export function getLedMode(id: number): LedMode | undefined {
    return LED_MODES.find(m => m.id === id);
}

export function getLedModeName(id: number): string {
    const mode = getLedMode(id);
    return mode?.name ?? `Mode ${id}`;
}
