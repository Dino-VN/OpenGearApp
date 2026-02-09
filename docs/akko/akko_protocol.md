# Akko Keyboard HID Protocol Documentation

## Overview

- **Interface**: HID Feature Reports
- **Packet Size**: 64 bytes  
- **Report ID**: 0 (Windows requires 65-byte buffer with prepended 0x00)

---

## Checksum Algorithm

**Standard (GET commands)**: `Byte 7 = 0xFF - Opcode`

**SET RGB (0x07)**: 
```rust
let sum: u16 = data[0..8].iter().map(|&b| b as u16).sum();
data[8] = (0xFF - (sum % 256)) as u8;
```

---

## Command Reference

| Opcode | Checksum | Name | Response |
|--------|----------|------|----------|
| `0x07` | Special | **SetRgbSettings** | Write command |
| `0x8F` | `0x70` | Handshake | `[8F, major, minor]` FW version |
| `0xF0` | `0x0F` | GetProfileCount | `[F0, count, active]` |
| `0x80` | `0x7F` | GetDeviceInfo | `[80, type, status]` |
| `0x87` | `0x78` | GetRgbSettings | `[87, mode, speed, brightness, 0, R, G, B]` |
| `0x88` | `0x77` | GetRgbMode | `[88, mode, p1, p2, brightness, R, G, B]` |
| `0x92` | `0x6D` | GetPerformance | `[92, debounce_dn, 0, debounce_up]` |
| `0x84` | `0x7B` | GetFnLockStatus | `[84, 0, enabled]` |
| `0x91` | `0x6E` | GetIndicatorLed | `[91, 0, enabled]` |
| `0x97` | `0x68` | GetSleepSettings | `[97, 0, 0]` |
| `0xAE` | `0x51` | GetMacroStatus | `[AE, 0, enabled]` |

---

## SET RGB Settings (0x07) - CRITICAL

### Packet Format

| Byte | Value | Description |
|------|-------|-------------|
| 0 | 0x07 | Opcode |
| 1 | 0x04 | Sub-command |
| 2 | 1-5 | **Speed (INVERTED!)** |
| 3 | 0-4 | Brightness (direct) |
| 4 | 0x07 | Fixed value |
| 5 | R | Red (0-255) |
| 6 | G | Green (0-255) |
| 7 | B | Blue (0-255) |
| 8 | CS | Checksum |

### Speed Conversion (INVERTED!)

```
UI Value → Protocol Value
   0     →     5      (slowest)
   1     →     4
   2     →     3
   3     →     2
   4     →     1      (fastest)

Formula: protocol = 5 - ui_value
```

### Brightness (DIRECT)

```
0 = OFF (LEDs disabled) ← Use for light toggle
1-4 = Brightness levels
```

### Example: Turn OFF LEDs
```
[07, 04, 01, 00, 07, FA, FF, FA, checksum, 00, ...]
           ↑    ↑
        speed  brightness=0 (OFF!)
```

---

## GET RGB Settings (0x87)

### Response Format (UPDATED)

| Byte | Description |
|------|-------------|
| 0 | 0x87 (opcode) |
| 1 | Mode ID |
| 2 | Speed (protocol, needs inversion!) |
| 3 | **Brightness** |
| 4 | Reserved |
| 5-7 | R, G, B |

### Reading Speed
```typescript
const speedUI = 5 - response[2]; // Convert from protocol
```

---

## Device IDs

| Model | VID | PID |
|-------|-----|-----|
| MOD007B | 0x3151 | 0x5009 |
| Akko 2.4G Wireless | 0x3151 | 0x4011 |

---

## Implementation Notes

1. **Handshake first** before other commands
2. **Speed is INVERTED** - UI ↔ protocol opposite
3. **Brightness 0 = OFF** - for light toggle
4. **Windows quirk**: 65-byte buffer with leading 0x00
