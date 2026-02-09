# Akko Keyboard RGB Protocol Documentation

## RGB Settings Packet Structure

The `SetRgbSettings` command (Opcode `0x07`) uses the following structure:

```
Byte Index:  [0]   [1]        [2]    [3]         [4]    [5] [6] [7]
Packet:      [07,  EFFECT,    SPD,   BRI,        C_MODE, R,  G,  B]
             └┬┘   │          │      │           │       └──┬──┘
              │    │          │      │           │          └─ RGB Color
              │    │          │      │           └─ Color Mode (7=Dazzle, 8=Color)
              │    │          │      └─ Brightness (0-4)
              │    │          └─ Speed (inverted: 0=Fast, 4=Slow/Stop)
              │    └─ Effect ID / Animation Mode
              └─ Opcode (0x07)
```

## Field Definitions

### 1. Color Mode (Byte 4)

This defines whether the effect uses the custom RGB values or a preset rainbow gradient.

- **`0x08` (Color)**: Uses the RGB values in bytes 5-7.
- **`0x07` (Dazzle)**: Uses a predetermined rainbow gradient (RGB bytes are ignored).

### 2. Speed (Byte 2)

Speed is inverted from the UI representation.

- **Protocol 5**: Speed 0 (Static/Stopped)
- **Protocol 4**: Speed 1 (Slow)
- **Protocol 3**: Speed 2 (Medium)
- **Protocol 2**: Speed 3 (Fast)
- **Protocol 1**: Speed 4 (Very Fast)

> **Note**: An implementation detail in `commands.rs` maps UI Speed (0-4) to `5 - speed`.

### 3. Effect ID (Byte 1)

Previously referred to as "Direction", this byte actually selects the specific lighting animation or effect.

| Value | Effect Name | Description |
|-------|-------------|-------------|
| **4** | **Drift** | Directional drift effect (Right/Left/Down/Up) |
| **5** | **Waves Ripple** | Ripple animation |
| **6** | **Stars Twinkle** | Random twinkling stars |
| **7** | **Steady Stream** | Linear stream effect |
| **8** | **Like Shadows** | Shadow/fading effect |
| **9** | **Peaks Rising** | Mountain peak animation |
| **10** | **Sine Wave** | Sine wave animation |
| **11** | **Flowing Spring** | Spring animation (Outward/Inward) |
| **12** | **Flowers Blooming** | Blooming animation (Right/Left) |
| **14** | **Laser** | Laser effect |
| **15** | **Peak Turn** | Turning effect (CCW/CW) |
| **16** | **Colorful V&H** | Vertical & horizontal colors |
| **17** | **Snow** | Snow effect |
| **18** | **Meteor** | Meteor effect (Directional) |
| **19** | **Light Trace** | Light trace effect |
| **20** | **Dynamic Breathing** | Breathing effect |
| **21** | **Spectrum Cycle** | Spectrum cycle |

## Direction & Sub-mode Patterns

Several modes use a shared pattern of sub-modes to represent different directions or patterns. These are set via **Byte 4 (Mode)** and **Byte 1 (Effect ID)**.

| Logical Direction | Dazzle Mode (Byte 4) | Color Mode (Byte 4) | Used By |
|-------------------|---------------------|---------------------|---------|
| **Right / Outward / Z-Shape / CCW** | `0x07` (7) | `0x08` (8) | Drift, Steady Stream, Flowing Spring, Flowers Blooming, Peak Turn |
| **Left / Inward / Round / CW** | `0x17` (23) | `0x18` (24) | Drift, Steady Stream, Flowing Spring, Flowers Blooming, Peak Turn |
| **Down** | `0x27` (39) | `0x28` (40) | Drift |
| **Up** | `0x37` (55) | `0x38` (56) | Drift |

## UI to Protocol Mapping

| UI Mode | UI Control | Effect ID (Byte 1) | Mode (Byte 4) |
|---------|------------|--------------------|---------------|
| **Static** | Color/Dazzle | Any | `8` / `7` (Speed 0) |
| **Drift** | Right / Left / Down / Up | `4` | `7/8`, `23/24`, `39/40`, `55/56` |
| **Steady Stream** | Z-Shape / Round | `7` | `7/8` (Z), `23/24` (Round) |
| **Flowing Spring** | Outward / Inward | `11` | `7/8` (Out), `23/24` (In) |
| **Flowers Blooming**| Right / Left | `12` | `7/8` (Right), `23/24` (Left) |
| **Peak Turn** | CCW / CW | `15` | `7/8` (CCW), `23/24` (CW) |

## Discovery Notes

- **"Waves Ripple"** identified via logs showing `direction=5`.
- **"Stars Twinkle"** identified via logs showing `direction=6`.
- **"Steady Stream"** identified via logs showing `direction=7`.
- **"Like Shadows"** identified via logs showing `direction=8`.
- **"Peaks Rising"** identified via logs showing `direction=9`.

Traffic capture logs confirmed that `Brightness` is Byte 3 (value 0-4) and `Speed` is Byte 2.
