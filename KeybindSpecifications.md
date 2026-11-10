# Wii Remote OSC Binding Specification

**Status:** Proposed default mapping  
**Namespace:** `/wiimote`  
**Controller:** Nintendo Wii Remote (RVL-003 / RVL-CNT-01)  
**Optional extension:** Nunchuk

This document defines the OSC addresses, physical controls, argument types, and behavior for mapping a Wii Remote to OSC. The mapping is designed around the Rust `wiiuse-rs` state API: button states, accelerometer readings, and optional Nunchuk state.

## 1. OSC conventions

- All addresses begin with `/wiimote`.
- Button messages use one integer argument: `1` means pressed and `0` means released.
- Button messages are sent on state changes, not continuously while a button is held.
- Accelerometer axis values use OSC `i` (32-bit integer) for raw sensor readings and OSC `f` (32-bit float) for G-force values.
- Accelerometer messages are sent only while accelerometer streaming is enabled.
- Nunchuk messages are sent only while a Nunchuk is connected and its data is available.
- Unless otherwise specified, each message has exactly the arguments shown in the table.

### OSC type tags

| Tag | OSC type       | Meaning                                  |
| --- | -------------- | ---------------------------------------- |
| `i` | 32-bit integer | Button state or raw accelerometer sample |
| `f` | 32-bit float   | Calibrated acceleration in G             |

## 2. Wii Remote buttons

Each button has a single state address. Send `1` when the button becomes pressed and `0` when it is released.

| OSC address             | Keybind / physical control | Arguments                       | Description                                                                                                                                                                                  |
| ----------------------- | -------------------------- | ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/wiimote/button/a`     | **A**                      | `i` — `1` pressed, `0` released | Main face action button. A press is reported immediately. Holding A for two seconds also activates accelerometer streaming; see [Accelerometer hold gesture](#4-accelerometer-hold-gesture). |
| `/wiimote/button/b`     | **B**                      | `i` — `1` pressed, `0` released | Trigger button on the underside of the Wii Remote.                                                                                                                                           |
| `/wiimote/button/1`     | **1**                      | `i` — `1` pressed, `0` released | Numbered button 1.                                                                                                                                                                           |
| `/wiimote/button/2`     | **2**                      | `i` — `1` pressed, `0` released | Numbered button 2.                                                                                                                                                                           |
| `/wiimote/button/plus`  | **+**                      | `i` — `1` pressed, `0` released | Plus button.                                                                                                                                                                                 |
| `/wiimote/button/minus` | **−**                      | `i` — `1` pressed, `0` released | Minus button.                                                                                                                                                                                |
| `/wiimote/button/home`  | **HOME**                   | `i` — `1` pressed, `0` released | Home button.                                                                                                                                                                                 |
| `/wiimote/button/up`    | D-pad **Up**               | `i` — `1` pressed, `0` released | Up direction on the D-pad.                                                                                                                                                                   |
| `/wiimote/button/down`  | D-pad **Down**             | `i` — `1` pressed, `0` released | Down direction on the D-pad.                                                                                                                                                                 |
| `/wiimote/button/left`  | D-pad **Left**             | `i` — `1` pressed, `0` released | Left direction on the D-pad.                                                                                                                                                                 |
| `/wiimote/button/right` | D-pad **Right**            | `i` — `1` pressed, `0` released | Right direction on the D-pad.                                                                                                                                                                |

## 3. Button gesture events

Gesture events are additional messages; they do not replace the normal button state messages above.

| OSC address              | Keybind / physical control        | Arguments                           | Description                                                                                                                   |
| ------------------------ | --------------------------------- | ----------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `/wiimote/button/a/long` | Hold **A** for at least 2 seconds | `i` — `1` when threshold is reached | One-shot event emitted once per continuous A hold when the hold reaches two seconds. It is not repeated while A remains held. |

## 4. Accelerometer hold gesture

The default interaction is: press and hold **A**; after two seconds, start sending accelerometer data. Releasing A stops the stream. The ordinary `/wiimote/button/a` state messages are still sent as usual.

| OSC address                       | Keybind / physical control                      | Arguments                       | Description                                                                                        |
| --------------------------------- | ----------------------------------------------- | ------------------------------- | -------------------------------------------------------------------------------------------------- |
| `/wiimote/accelerometer/stream`   | Hold **A** for 2 seconds; release **A** to stop | `i` — `1` enabled, `0` disabled | Stream state. Send `1` once when the two-second threshold is reached. Send `0` when A is released. |
| `/wiimote/accelerometer/raw/x`    | Wii Remote accelerometer **X** axis             | `i` — raw sample                | Raw X-axis reading from the Wii Remote accelerometer.                                              |
| `/wiimote/accelerometer/raw/y`    | Wii Remote accelerometer **Y** axis             | `i` — raw sample                | Raw Y-axis reading from the Wii Remote accelerometer.                                              |
| `/wiimote/accelerometer/raw/z`    | Wii Remote accelerometer **Z** axis             | `i` — raw sample                | Raw Z-axis reading from the Wii Remote accelerometer.                                              |
| `/wiimote/accelerometer/gforce/x` | Wii Remote accelerometer **X** axis             | `f` — G value                   | Calibrated X-axis acceleration, expressed in G.                                                    |
| `/wiimote/accelerometer/gforce/y` | Wii Remote accelerometer **Y** axis             | `f` — G value                   | Calibrated Y-axis acceleration, expressed in G.                                                    |
| `/wiimote/accelerometer/gforce/z` | Wii Remote accelerometer **Z** axis             | `f` — G value                   | Calibrated Z-axis acceleration, expressed in G.                                                    |

Raw and G-force values describe the same sensor sample in different forms. Receivers that need physical acceleration should generally use the G-force addresses; receivers that need device-specific sensor counts can use the raw addresses.

## 5. Nunchuk buttons (optional)

These addresses are active only when a Nunchuk is attached. A controller without a Nunchuk simply does not emit these messages.

| OSC address                 | Keybind / physical control | Arguments                       | Description             |
| --------------------------- | -------------------------- | ------------------------------- | ----------------------- |
| `/wiimote/nunchuk/button/c` | Nunchuk **C**              | `i` — `1` pressed, `0` released | Nunchuk C button state. |
| `/wiimote/nunchuk/button/z` | Nunchuk **Z**              | `i` — `1` pressed, `0` released | Nunchuk Z button state. |

## 6. Nunchuk accelerometer (optional)

Nunchuk accelerometer messages are sent only when a Nunchuk is attached and accelerometer streaming is enabled using the same two-second A hold gesture. The Wii Remote and Nunchuk sensors have separate address paths.

| OSC address                               | Keybind / physical control       | Arguments        | Description                                             |
| ----------------------------------------- | -------------------------------- | ---------------- | ------------------------------------------------------- |
| `/wiimote/nunchuk/accelerometer/raw/x`    | Nunchuk accelerometer **X** axis | `i` — raw sample | Raw Nunchuk X-axis reading.                             |
| `/wiimote/nunchuk/accelerometer/raw/y`    | Nunchuk accelerometer **Y** axis | `i` — raw sample | Raw Nunchuk Y-axis reading.                             |
| `/wiimote/nunchuk/accelerometer/raw/z`    | Nunchuk accelerometer **Z** axis | `i` — raw sample | Raw Nunchuk Z-axis reading.                             |
| `/wiimote/nunchuk/accelerometer/gforce/x` | Nunchuk accelerometer **X** axis | `f` — G value    | Calibrated Nunchuk X-axis acceleration, expressed in G. |
| `/wiimote/nunchuk/accelerometer/gforce/y` | Nunchuk accelerometer **Y** axis | `f` — G value    | Calibrated Nunchuk Y-axis acceleration, expressed in G. |
| `/wiimote/nunchuk/accelerometer/gforce/z` | Nunchuk accelerometer **Z** axis | `f` — G value    | Calibrated Nunchuk Z-axis acceleration, expressed in G. |

## Nunchuk joystick OSC mapping

| OSC name                              | Keybind / physical control         | Description                                                                                |
| ------------------------------------- | ---------------------------------- | ------------------------------------------------------------------------------------------ |
| `/wiimote/nunchuk/joystick/raw/x`     | Nunchuk joystick — horizontal axis | Raw horizontal axis value. Increases to the right.                                         |
| `/wiimote/nunchuk/joystick/raw/y`     | Nunchuk joystick — vertical axis   | Raw vertical axis value. Increases upward.                                                 |
| `/wiimote/nunchuk/joystick/x`         | Nunchuk joystick — horizontal axis | Normalized horizontal position: `-1.0` = left, `0.0` = center, `+1.0` = right.             |
| `/wiimote/nunchuk/joystick/y`         | Nunchuk joystick — vertical axis   | Normalized vertical position: `-1.0` = down, `0.0` = center, `+1.0` = up.                  |
| `/wiimote/nunchuk/joystick/magnitude` | Nunchuk joystick — displacement    | Distance from center, normalized from `0.0` to `1.0`.                                      |
| `/wiimote/nunchuk/joystick/angle`     | Nunchuk joystick — direction       | Direction in radians, counter-clockwise from the positive X axis. Undefined when centered. |

## 7. Connection and extension status

These messages describe device availability. They should be emitted when the corresponding state changes.

| OSC address                  | Keybind / physical control | Arguments                             | Description                                                            |
| ---------------------------- | -------------------------- | ------------------------------------- | ---------------------------------------------------------------------- |
| `/wiimote/connected`         | Wii Remote connection      | `i` — `1` connected, `0` disconnected | Indicates whether the Wii Remote is connected and available for input. |
| `/wiimote/nunchuk/connected` | Nunchuk attach / detach    | `i` — `1` connected, `0` disconnected | Indicates whether a Nunchuk is currently attached and recognized.      |

## 8. Recommended message behavior

1. On successful connection, emit `/wiimote/connected 1` and send the current button states so receivers start synchronized.
2. When a button changes state, emit its button address once with `1` or `0`.
3. When A has remained pressed continuously for two seconds, emit `/wiimote/button/a/long 1` and `/wiimote/accelerometer/stream 1`.
4. While streaming is enabled, emit the six Wii Remote accelerometer axis messages for each sample. If a Nunchuk is attached, emit its six axis messages too.
5. When A is released, emit `/wiimote/accelerometer/stream 0` and stop sending accelerometer samples. Also emit `/wiimote/button/a 0`.
6. On disconnect, emit `/wiimote/connected 0` and stop all input streams.

## 9. Example OSC messages

```text
/wiimote/button/a                 1
/wiimote/button/a/long            1
/wiimote/accelerometer/stream     1
/wiimote/accelerometer/raw/x      512
/wiimote/accelerometer/gforce/x   0.03
/wiimote/button/a                 0
/wiimote/accelerometer/stream     0
/wiimote/connected                0
```

## 10. Implementation notes

- This document specifies the OSC contract; it does not itself implement OSC networking.
- The core Wii Remote button and accelerometer paths map to the current `wiiuse-rs` state API.
- Nunchuk paths are conditional on extension detection and availability of Nunchuk state.
- Connection-status messages and the OSC long-press event should be emitted by the application layer when those transitions are detected.
- Use the same OSC address spelling and argument types consistently. Do not alternate between string booleans, integer booleans, and float booleans.
