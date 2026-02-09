// Keyboard layout definitions for different models
// Each layout defines key positions, sizes, and labels

export interface KeyDef {
    id: string;
    x: number;
    y: number;
    w: number;        // width
    h: number;        // height
    label: string;
    label2?: string;  // secondary label (e.g., shift layer)
}

export interface KeyboardLayout {
    name: string;
    model: string;
    width: number;
    height: number;
    rows: KeyDef[][];
}

// Standard key unit size (for reference)
// const U = 40;  // 1 unit = 40px
// const GAP = 4; // gap between keys

export const MOD007B_LAYOUT: KeyboardLayout = {
    name: "Akko MOD007B",
    model: "mod007b",
    width: 620,
    height: 256,
    rows: [
        // Row 0: Function row
        [
            { id: "esc", x: 0, y: 0, w: 36, h: 32, label: "Esc" },
            { id: "f1", x: 46, y: 0, w: 32, h: 32, label: "F1" },
            { id: "f2", x: 82, y: 0, w: 32, h: 32, label: "F2" },
            { id: "f3", x: 118, y: 0, w: 32, h: 32, label: "F3" },
            { id: "f4", x: 154, y: 0, w: 32, h: 32, label: "F4" },
            { id: "f5", x: 196, y: 0, w: 32, h: 32, label: "F5" },
            { id: "f6", x: 232, y: 0, w: 32, h: 32, label: "F6" },
            { id: "f7", x: 268, y: 0, w: 32, h: 32, label: "F7" },
            { id: "f8", x: 304, y: 0, w: 32, h: 32, label: "F8" },
            { id: "f9", x: 346, y: 0, w: 32, h: 32, label: "F9" },
            { id: "f10", x: 382, y: 0, w: 32, h: 32, label: "F10" },
            { id: "f11", x: 418, y: 0, w: 32, h: 32, label: "F11" },
            { id: "f12", x: 454, y: 0, w: 32, h: 32, label: "F12" },
            { id: "del", x: 496, y: 0, w: 36, h: 32, label: "Del" },
        ],
        // Row 1: Number row
        [
            { id: "grave", x: 0, y: 40, w: 36, h: 36, label: "~", label2: "`" },
            { id: "1", x: 40, y: 40, w: 36, h: 36, label: "1", label2: "!" },
            { id: "2", x: 80, y: 40, w: 36, h: 36, label: "2", label2: "@" },
            { id: "3", x: 120, y: 40, w: 36, h: 36, label: "3", label2: "#" },
            { id: "4", x: 160, y: 40, w: 36, h: 36, label: "4", label2: "$" },
            { id: "5", x: 200, y: 40, w: 36, h: 36, label: "5", label2: "%" },
            { id: "6", x: 240, y: 40, w: 36, h: 36, label: "6", label2: "^" },
            { id: "7", x: 280, y: 40, w: 36, h: 36, label: "7", label2: "&" },
            { id: "8", x: 320, y: 40, w: 36, h: 36, label: "8", label2: "*" },
            { id: "9", x: 360, y: 40, w: 36, h: 36, label: "9", label2: "(" },
            { id: "0", x: 400, y: 40, w: 36, h: 36, label: "0", label2: ")" },
            { id: "minus", x: 440, y: 40, w: 36, h: 36, label: "-", label2: "_" },
            { id: "equal", x: 480, y: 40, w: 36, h: 36, label: "=", label2: "+" },
            { id: "backspace", x: 520, y: 40, w: 56, h: 36, label: "←" },
            { id: "home", x: 580, y: 40, w: 36, h: 36, label: "Home" },
        ],
        // Row 2: QWERTY row
        [
            { id: "tab", x: 0, y: 80, w: 52, h: 36, label: "Tab" },
            { id: "q", x: 56, y: 80, w: 36, h: 36, label: "Q" },
            { id: "w", x: 96, y: 80, w: 36, h: 36, label: "W" },
            { id: "e", x: 136, y: 80, w: 36, h: 36, label: "E" },
            { id: "r", x: 176, y: 80, w: 36, h: 36, label: "R" },
            { id: "t", x: 216, y: 80, w: 36, h: 36, label: "T" },
            { id: "y", x: 256, y: 80, w: 36, h: 36, label: "Y" },
            { id: "u", x: 296, y: 80, w: 36, h: 36, label: "U" },
            { id: "i", x: 336, y: 80, w: 36, h: 36, label: "I" },
            { id: "o", x: 376, y: 80, w: 36, h: 36, label: "O" },
            { id: "p", x: 416, y: 80, w: 36, h: 36, label: "P" },
            { id: "lbracket", x: 456, y: 80, w: 36, h: 36, label: "[", label2: "{" },
            { id: "rbracket", x: 496, y: 80, w: 36, h: 36, label: "]", label2: "}" },
            { id: "backslash", x: 536, y: 80, w: 40, h: 36, label: "\\", label2: "|" },
            { id: "pgup", x: 580, y: 80, w: 36, h: 36, label: "PgUp" },
        ],
        // Row 3: Home row
        [
            { id: "caps", x: 0, y: 120, w: 60, h: 36, label: "Caps" },
            { id: "a", x: 64, y: 120, w: 36, h: 36, label: "A" },
            { id: "s", x: 104, y: 120, w: 36, h: 36, label: "S" },
            { id: "d", x: 144, y: 120, w: 36, h: 36, label: "D" },
            { id: "f", x: 184, y: 120, w: 36, h: 36, label: "F" },
            { id: "g", x: 224, y: 120, w: 36, h: 36, label: "G" },
            { id: "h", x: 264, y: 120, w: 36, h: 36, label: "H" },
            { id: "j", x: 304, y: 120, w: 36, h: 36, label: "J" },
            { id: "k", x: 344, y: 120, w: 36, h: 36, label: "K" },
            { id: "l", x: 384, y: 120, w: 36, h: 36, label: "L" },
            { id: "semicolon", x: 424, y: 120, w: 36, h: 36, label: ";", label2: ":" },
            { id: "quote", x: 464, y: 120, w: 36, h: 36, label: "'", label2: "\"" },
            { id: "enter", x: 504, y: 120, w: 72, h: 36, label: "Enter" },
            { id: "pgdn", x: 580, y: 120, w: 36, h: 36, label: "PgDn" },
        ],
        // Row 4: Shift row
        [
            { id: "lshift", x: 0, y: 160, w: 76, h: 36, label: "Shift" },
            { id: "z", x: 80, y: 160, w: 36, h: 36, label: "Z" },
            { id: "x", x: 120, y: 160, w: 36, h: 36, label: "X" },
            { id: "c", x: 160, y: 160, w: 36, h: 36, label: "C" },
            { id: "v", x: 200, y: 160, w: 36, h: 36, label: "V" },
            { id: "b", x: 240, y: 160, w: 36, h: 36, label: "B" },
            { id: "n", x: 280, y: 160, w: 36, h: 36, label: "N" },
            { id: "m", x: 320, y: 160, w: 36, h: 36, label: "M" },
            { id: "comma", x: 360, y: 160, w: 36, h: 36, label: ",", label2: "<" },
            { id: "period", x: 400, y: 160, w: 36, h: 36, label: ".", label2: ">" },
            { id: "slash", x: 440, y: 160, w: 36, h: 36, label: "/", label2: "?" },
            { id: "rshift", x: 480, y: 160, w: 60, h: 36, label: "Shift" },
            { id: "up", x: 544, y: 160, w: 32, h: 36, label: "↑" },
            { id: "end", x: 580, y: 160, w: 36, h: 36, label: "End" },
        ],
        // Row 5: Bottom row
        [
            { id: "lctrl", x: 0, y: 200, w: 44, h: 36, label: "Ctrl" },
            { id: "lwin", x: 48, y: 200, w: 44, h: 36, label: "Win" },
            { id: "lalt", x: 96, y: 200, w: 44, h: 36, label: "Alt" },
            { id: "space", x: 144, y: 200, w: 224, h: 36, label: "" },
            { id: "ralt", x: 372, y: 200, w: 44, h: 36, label: "Alt" },
            { id: "fn", x: 420, y: 200, w: 36, h: 36, label: "Fn" },
            { id: "rctrl", x: 460, y: 200, w: 44, h: 36, label: "Ctrl" },
            { id: "left", x: 508, y: 200, w: 32, h: 36, label: "←" },
            { id: "down", x: 544, y: 200, w: 32, h: 36, label: "↓" },
            { id: "right", x: 580, y: 200, w: 32, h: 36, label: "→" },
        ],
    ],
};

// Layout registry
export const LAYOUTS: Record<string, KeyboardLayout> = {
    mod007b: MOD007B_LAYOUT,
};

export function getLayout(model: string): KeyboardLayout | undefined {
    return LAYOUTS[model.toLowerCase()];
}
