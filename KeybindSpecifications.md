# Wii Remote OSC Binding Specification

**Status:** Proposed default mapping  
**Namespace:** `/wmX`with `X` the wiimote number
**Controller:** Nintendo Wii Remote (RVL-003 / RVL-CNT-01)  
**Optional extension:** Nunchuk

This document defines the OSC addresses, physical controls, argument types, and behavior for mapping a Wii Remote to OSC. The mapping is designed around the Rust `wiiuse-rs` state API: button states, accelerometer readings, and optional Nunchuk state.

## OSC conventions

- All addresses begin with `/wmX` with `X` the wiimote number.
- For now all osc messages should be in UDP until we have bandwidth problems
- Button messages use one integer argument: `1` means pressed and `0` means released
- Accelerometer axis values use OSC `f` (32-bit float) for G-force values.
- Nunchuk messages are sent only while a Nunchuk is connected and its data is available.

## Wii Remote buttons

Each button has a single state address. Send `1` when the button becomes pressed and `0` when it is released.

| OSC address    | Keybind / physical control | Arguments                       | Description                                                                                                                                                                                  |
| -------------- | -------------------------- | ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/wmX/b/a`     | **A**                      | `i` — `1` pressed, `0` released | Main face action button. A press is reported immediately. Holding A for two seconds also activates accelerometer streaming; see [Accelerometer hold gesture](#4-accelerometer-hold-gesture). |
| `/wmX/b/b`     | **B**                      | `i` — `1` pressed, `0` released | Trigger button on the underside of the Wii Remote.                                                                                                                                           |
| `/wmX/b/1`     | **1**                      | `i` — `1` pressed, `0` released | Numbered button 1.                                                                                                                                                                           |
| `/wmX/b/2`     | **2**                      | `i` — `1` pressed, `0` released | Numbered button 2.                                                                                                                                                                           |
| `/wmX/b/plus`  | **+**                      | `i` — `1` pressed, `0` released | Plus button.                                                                                                                                                                                 |
| `/wmX/b/minus` | **−**                      | `i` — `1` pressed, `0` released | Minus button.                                                                                                                                                                                |
| `/wmX/b/home`  | **HOME**                   | `i` — `1` pressed, `0` released | Home button.                                                                                                                                                                                 |
| `/wmX/b/up`    | D-pad **Up**               | `i` — `1` pressed, `0` released | Up direction on the D-pad.                                                                                                                                                                   |
| `/wmX/b/down`  | D-pad **Down**             | `i` — `1` pressed, `0` released | Down direction on the D-pad.                                                                                                                                                                 |
| `/wmX/b/left`  | D-pad **Left**             | `i` — `1` pressed, `0` released | Left direction on the D-pad.                                                                                                                                                                 |
| `/wmX/b/right` | D-pad **Right**            | `i` — `1` pressed, `0` released | Right direction on the D-pad.                                                                                                                                                                |

## Accelerometer hold gesture

The default interaction is: press and hold **A**; after two seconds, start sending accelerometer data. Releasing A stops the stream. The ordinary `/wmX/b/a` state messages are still sent as usual.

| OSC address     | Keybind / physical control                      | Arguments                       | Description                                                                                        |
| --------------- | ----------------------------------------------- | ------------------------------- | -------------------------------------------------------------------------------------------------- |
| `/wmX/a/stream` | Hold **A** for 2 seconds; release **A** to stop | `i` — `1` enabled, `0` disabled | Stream state. Send `1` once when the two-second threshold is reached. Send `0` when A is released. |
| `/wmX/ax`       | Wii Remote accelerometer **X** axis             | `f` — G value                   | Calibrated X-axis acceleration, expressed in G.                                                    |
| `/wmX/ay`       | Wii Remote accelerometer **Y** axis             | `f` — G value                   | Calibrated Y-axis acceleration, expressed in G.                                                    |
| `/wmX/az`       | Wii Remote accelerometer **Z** axis             | `f` — G value                   | Calibrated Z-axis acceleration, expressed in G.                                                    |

Raw and G-force values describe the same sensor sample in different forms. Receivers that need physical acceleration should generally use the G-force addresses; receivers that need device-specific sensor counts can use the raw addresses.

## Nunchuk buttons (optional)

These addresses are active only when a Nunchuk is attached. A controller without a Nunchuk simply does not emit these messages.

| OSC address | Keybind / physical control | Arguments                       | Description             |
| ----------- | -------------------------- | ------------------------------- | ----------------------- |
| `/wmX/n/bc` | Nunchuk **C**              | `i` — `1` pressed, `0` released | Nunchuk C button state. |
| `/wmX/n/bz` | Nunchuk **Z**              | `i` — `1` pressed, `0` released | Nunchuk Z button state. |

## Nunchuk accelerometer (optional)

Nunchuk accelerometer messages are sent only when a Nunchuk is attached and accelerometer streaming is enabled using the same two-second A hold gesture. The Wii Remote and Nunchuk sensors have separate address paths.

| OSC address | Keybind / physical control       | Arguments     | Description                                             |
| ----------- | -------------------------------- | ------------- | ------------------------------------------------------- |
| `/wmX/n/ax` | Nunchuk accelerometer **X** axis | `f` — G value | Calibrated Nunchuk X-axis acceleration, expressed in G. |
| `/wmX/n/ay` | Nunchuk accelerometer **Y** axis | `f` — G value | Calibrated Nunchuk Y-axis acceleration, expressed in G. |
| `/wmX/n/az` | Nunchuk accelerometer **Z** axis | `f` — G value | Calibrated Nunchuk Z-axis acceleration, expressed in G. |

## Nunchuk joystick OSC mapping

| OSC name    | Keybind / physical control         | Description                                                                                |
| ----------- | ---------------------------------- | ------------------------------------------------------------------------------------------ |
| `/wmX/n/jx` | Nunchuk joystick — horizontal axis | Normalized horizontal position: `-1.0` = left, `0.0` = center, `+1.0` = right.             |
| `/wmX/n/jy` | Nunchuk joystick — vertical axis   | Normalized vertical position: `-1.0` = down, `0.0` = center, `+1.0` = up.                  |
| `/wmX/n/jm` | Nunchuk joystick — displacement    | Distance from center, normalized from `0.0` to `1.0`.                                      |
| `/wmX/n/ja` | Nunchuk joystick — direction       | Direction in radians, counter-clockwise from the positive X axis. Undefined when centered. |

## Connection and extension status

These messages describe device availability. They should be emitted when the corresponding state changes.

| OSC address | Keybind / physical control | Arguments                             | Description                                                            |
| ----------- | -------------------------- | ------------------------------------- | ---------------------------------------------------------------------- |
| `/wmX/c`    | Wii Remote connection      | `i` — `1` connected, `0` disconnected | Indicates whether the Wii Remote is connected and available for input. |
| `/wmX/n/c`  | Nunchuk attach / detach    | `i` — `1` connected, `0` disconnected | Indicates whether a Nunchuk is currently attached and recognized.      |
