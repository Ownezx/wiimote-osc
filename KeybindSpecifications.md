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

| OSC address | Keybind / physical control | Arguments                       | Description                |
| ----------- | -------------------------- | ------------------------------- | -------------------------- |
| `/wmX/ba`   | **A**                      | `i` — `1` pressed, `0` released | Main face action button.   |
| `/wmX/bb`   | **B**                      | `i` — `1` pressed, `0` released | Wii Remote trigger button. |
| `/wmX/b1`   | **1**                      | `i` — `1` pressed, `0` released | Numbered button 1.         |
| `/wmX/b2`   | **2**                      | `i` — `1` pressed, `0` released | Numbered button 2.         |
| `/wmX/bp`   | **+**                      | `i` — `1` pressed, `0` released | Plus button.               |
| `/wmX/bm`   | **−**                      | `i` — `1` pressed, `0` released | Minus button.              |
| `/wmX/bh`   | **HOME**                   | `i` — `1` pressed, `0` released | Home button.               |
| `/wmX/bu`   | D-pad **Up**               | `i` — `1` pressed, `0` released | D-pad Up.                  |
| `/wmX/bd`   | D-pad **Down**             | `i` — `1` pressed, `0` released | D-pad Down.                |
| `/wmX/bl`   | D-pad **Left**             | `i` — `1` pressed, `0` released | D-pad Left.                |
| `/wmX/br`   | D-pad **Right**            | `i` — `1` pressed, `0` released | D-pad Right.               |

## Accelerometer hold gesture

The default interaction is: press and hold **A**; after two seconds, start sending accelerometer data. Releasing A stops the stream. The ordinary `/wmX/b/a` state messages are still sent as usual.

| OSC address | Keybind / physical control          | Arguments     | Description                  |
| ----------- | ----------------------------------- | ------------- | ---------------------------- |
| `/wmX/ax`   | Wii Remote accelerometer **X** axis | `f` — G value | Wiimote X-axis acceleration. |
| `/wmX/ay`   | Wii Remote accelerometer **Y** axis | `f` — G value | Wiimote Y-axis acceleration. |
| `/wmX/az`   | Wii Remote accelerometer **Z** axis | `f` — G value | Wiimote Z-axis acceleration. |

Raw and G-force values describe the same sensor sample in different forms. Receivers that need physical acceleration should generally use the G-force addresses; receivers that need device-specific sensor counts can use the raw addresses.

## Nunchuk buttons (optional)

These addresses are active only when a Nunchuk is attached. A controller without a Nunchuk simply does not emit these messages.

| OSC address | Keybind / physical control | Arguments                       | Description             |
| ----------- | -------------------------- | ------------------------------- | ----------------------- |
| `/wmX/n/bc` | Nunchuk **C**              | `i` — `1` pressed, `0` released | Nunchuk C button state. |
| `/wmX/n/bz` | Nunchuk **Z**              | `i` — `1` pressed, `0` released | Nunchuk Z button state. |

## Nunchuk accelerometer (optional)

Nunchuk accelerometer messages are sent only when a Nunchuk is attached and accelerometer streaming is enabled using the same two-second A hold gesture. The Wii Remote and Nunchuk sensors have separate address paths.

| OSC address | Keybind / physical control       | Arguments     | Description                  |
| ----------- | -------------------------------- | ------------- | ---------------------------- |
| `/wmX/n/ax` | Nunchuk accelerometer **X** axis | `f` — G value | Nunchuk X-axis acceleration. |
| `/wmX/n/ay` | Nunchuk accelerometer **Y** axis | `f` — G value | Nunchuk Y-axis acceleration. |
| `/wmX/n/az` | Nunchuk accelerometer **Z** axis | `f` — G value | Nunchuk Z-axis acceleration. |

## Nunchuk joystick OSC mapping

| OSC name    | Keybind / physical control         | Arguments | Description                                         |
| ----------- | ---------------------------------- | --------- | --------------------------------------------------- |
| `/wmX/n/jx` | Nunchuk joystick — horizontal axis | `f`       | Horizontal position: `-1.0` = left, `+1.0` = right. |
| `/wmX/n/jy` | Nunchuk joystick — vertical axis   | `f`       | Vertical position: `-1.0` = down,`+1.0` = up.       |

## Connection and extension status

These messages describe device availability. They should be emitted when the corresponding state changes.

| OSC address | Keybind / physical control | Arguments                             | Description           |
| ----------- | -------------------------- | ------------------------------------- | --------------------- |
| `/wmX/c`    | Wii Remote connection      | `i` — `1` connected, `0` disconnected | Wii Remote connected. |
| `/wmX/n/c`  | Nunchuk attach / detach    | `i` — `1` connected, `0` disconnected | Nunchuk connected.    |
