# Wii Remote OSC Binding Specification

Status: Proposed default mapping  
Namespace: `/wmX`with `X` the wiimote number
Controller: Nintendo Wii Remote (RVL-003 / RVL-CNT-01)  
Optional extension: Nunchuk

This document defines the OSC addresses, physical controls, argument types, and behavior for mapping a Wii Remote to OSC. The mapping is designed around the Rust `wiiuse-rs` state API: button states, accelerometer readings, and optional Nunchuk state.

## OSC conventions

- All addresses begin with `/wmX` with `X` the wiimote number.
- For now all osc messages should be in UDP until we have bandwidth problems
- Button messages use one integer argument: `1` means pressed and `0` means released
- Accelerometer axis values use OSC `f` (32-bit float) for G-force values.
- Nunchuk messages are sent only while a Nunchuk is connected and its data is available.

## Function keys

Because we wish to have function keys to create specific actions, certain buttons are not used in the OSC scheme and instead used to create other key combinations as presented in the key [key combination section](#key-combinations) .

- Nunchuk C button is considered as a shift key
- D-pad is used for color actions and toggles

## Wii Remote buttons

Each button has a single state address. Send `1` when the button becomes pressed and `0` when it is released.

| OSC address | Keybind / physical control | Arguments                       | Description                |
| ----------- | -------------------------- | ------------------------------- | -------------------------- |
| `/wmX/ba`   | A                          | `i` — `1` pressed, `0` released | Main face action button.   |
| `/wmX/bb`   | B                          | `i` — `1` pressed, `0` released | Wii Remote trigger button. |
| `/wmX/b1`   | 1                          | `i` — `1` pressed, `0` released | Numbered button 1.         |
| `/wmX/b2`   | 2                          | `i` — `1` pressed, `0` released | Numbered button 2.         |
| `/wmX/bp`   | +                          | `i` — `1` pressed, `0` released | Plus button.               |
| `/wmX/bm`   | −                          | `i` — `1` pressed, `0` released | Minus button.              |
| `/wmX/bh`   | HOME                       | `i` — `1` pressed, `0` released | Home button.               |

## Accelerometer hold gesture

| OSC address | Keybind / physical control      | Arguments     | Description                  |
| ----------- | ------------------------------- | ------------- | ---------------------------- |
| `/wmX/ax`   | Wii Remote accelerometer X axis | `f` — G value | Wiimote X-axis acceleration. |
| `/wmX/ay`   | Wii Remote accelerometer Y axis | `f` — G value | Wiimote Y-axis acceleration. |
| `/wmX/az`   | Wii Remote accelerometer Z axis | `f` — G value | Wiimote Z-axis acceleration. |

## IR sensor

| OSC address | Keybind / physical control | Arguments | Description                 |
| ----------- | -------------------------- | --------- | --------------------------- |
| `/wmX/irx`  | IR horizontal position     | `f`       | Wiimote X-axis IR position. |
| `/wmX/iry`  | IR vertical position       | `f`       | Wiimote Y-axis IR position. |
| `/wmX/irv`  | IR visible                 | `i`       | 1 if visible.               |

## Nunchuk buttons (optional)

These addresses are active only when a Nunchuk is attached. A controller without a Nunchuk simply does not emit these messages.

| OSC address | Keybind / physical control | Arguments                       | Description             |
| ----------- | -------------------------- | ------------------------------- | ----------------------- |
| `/wmX/n/bz` | Nunchuk Z                  | `i` — `1` pressed, `0` released | Nunchuk Z button state. |

## Nunchuk accelerometer (optional)

| OSC address | Keybind / physical control   | Arguments     | Description                  |
| ----------- | ---------------------------- | ------------- | ---------------------------- |
| `/wmX/n/ax` | Nunchuk accelerometer X axis | `f` — G value | Nunchuk X-axis acceleration. |
| `/wmX/n/ay` | Nunchuk accelerometer Y axis | `f` — G value | Nunchuk Y-axis acceleration. |
| `/wmX/n/az` | Nunchuk accelerometer Z axis | `f` — G value | Nunchuk Z-axis acceleration. |

## Nunchuk joystick OSC mapping

| OSC name    | Keybind / physical control         | Arguments | Description                                         |
| ----------- | ---------------------------------- | --------- | --------------------------------------------------- |
| `/wmX/n/jx` | Nunchuk joystick — horizontal axis | `f`       | Horizontal position: `-1.0` = left, `+1.0` = right. |
| `/wmX/n/jy` | Nunchuk joystick — vertical axis   | `f`       | Vertical position: `-1.0` = down,`+1.0` = up.       |

Note: Deadzone continuously sends 0.

## Connection and extension status

These messages describe device availability. They should be emitted when the corresponding state changes.

| OSC address | Keybind / physical control | Arguments                             | Description           |
| ----------- | -------------------------- | ------------------------------------- | --------------------- |
| `/wmX/c`    | Wii Remote connection      | `i` — `1` connected, `0` disconnected | Wii Remote connected. |
| `/wmX/n/c`  | Nunchuk attach / detach    | `i` — `1` connected, `0` disconnected | Nunchuk connected.    |

## Key combinations

| OSC address | Keybind / physical control                                              | Arguments | Description                                          |
| ----------- | ----------------------------------------------------------------------- | --------- | ---------------------------------------------------- |
| `/wmX/c`    | Color being modified                                                    | `i`       | A color currently being selected {1,2,3,4}           |
| `/wmX/c1r`  | D-pad UP + Nunchuk joystick (hue) + IR position (saturation + value)    | `f`       | Red component of color 1                             |
| `/wmX/c1g`  | D-pad UP + Nunchuk joystick (hue) + IR position (saturation + value)    | `f`       | Green component of color 1                           |
| `/wmX/c1b`  | D-pad UP + Nunchuk joystick (hue) + IR position (saturation + value)    | `f`       | Blue component of color 1                            |
| `/wmX/c2r`  | D-pad RIGHT + Nunchuk joystick (hue) + IR position (saturation + value) | `f`       | Red component of color 2                             |
| `/wmX/c2g`  | D-pad RIGHT + Nunchuk joystick (hue) + IR position (saturation + value) | `f`       | Green component of color 2                           |
| `/wmX/c2b`  | D-pad RIGHT + Nunchuk joystick (hue) + IR position (saturation + value) | `f`       | Blue component of color 2                            |
| `/wmX/c3r`  | D-pad DOWN + Nunchuk joystick (hue) + IR position (saturation + value)  | `f`       | Red component of color 3                             |
| `/wmX/c3g`  | D-pad DOWN + Nunchuk joystick (hue) + IR position (saturation + value)  | `f`       | Green component of color 3                           |
| `/wmX/c3b`  | D-pad DOWN + Nunchuk joystick (hue) + IR position (saturation + value)  | `f`       | Blue component of color 3                            |
| `/wmX/c4r`  | D-pad LEFT + Nunchuk joystick (hue) + IR position (saturation + value)  | `f`       | Red component of color 4                             |
| `/wmX/c4g`  | D-pad LEFT + Nunchuk joystick (hue) + IR position (saturation + value)  | `f`       | Green component of color 4                           |
| `/wmX/c4b`  | D-pad LEFT + Nunchuk joystick (hue) + IR position (saturation + value)  | `f`       | Blue component of color 4                            |
| `/wmX/cx`   | Nunchuk C button + Nunchuk joystick X                                   | `f`       | Linear choice between color 2 and 4, middle is black |
| `/wmX/cy`   | Nunchuk C button + Nunchuk joystick Y                                   | `f`       | Linear choice between color 1 and 3, middle is black |
| `/wmX/f1`   | Nunchuk C button + D-pad UP long press                                  | `i`       | Toggle function 1                                    |
| `/wmX/f2`   | Nunchuk C button + D-pad RIGHT long press                               | `i`       | Toggle function 2                                    |
| `/wmX/f3`   | Nunchuk C button + D-pad DOWN long press                                | `i`       | Toggle function 3                                    |
| `/wmX/f4`   | Nunchuk C button + D-pad LEFT long press                                | `i`       | Toggle function 4                                    |

Notes:

- Color output values between 0 and 1
- To select the color, the nunchuk is used as a 360º control for hue (if in much bigger deadzone don't modify color) and the IRx for saturation and IRy for value
- long presses are 1 second long
