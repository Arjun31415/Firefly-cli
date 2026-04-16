# Firefly-cli

CLI for managing colors and effects for CosmicByte Firefly keyboard.

## About 
This project was done by reverse engineering the firefly software on windows. On Windows, The packets were sniffed using WireShark while running the  firefly GUI software. Then packets were analysed and the logic written in rust using `rusb` crate.  

## Features 
- [x] 12 different Effects
    - [x] Setting loop/individual color of effects

- [x] 7 different colors

- [ ] Individual key paint
- [ ] Brightness control (This can already be modified via keybind Fn + {up,down} arrow)
- [ ] Speed control (This can already be modified via keybind Fn + {left,right} arrow)

## Usage 

1. Requires user to have permission to access usb devices, otherwise sudo can be used (not recommended).
Ref: this stackoverflow [link](https://stackoverflow.com/questions/22713834/libusb-cannot-open-usb-device-permission-isse-netbeans-ubuntu)
2. git clone 
3. Run `cargo run` one time to build and start the binary
4. After that, use the binary directly from `./target/debug/Firefly-cli` or `./Firefly-cli` if you are in `/target/debug/`

### Commands

The CLI currently uses flags instead of subcommands.

Basic format:

```bash
./target/debug/Firefly-cli --effect <effect-name> [--colors <7 hex colors>] [--ci <0-7>]
```

### Flags

- `--effect`, `-e`
  Required. Selects the lighting effect.

- `--colors`, `-c`
  Optional. Comma-separated list of exactly 7 hex colors.
  Example:
  `--colors "#ff0000,#00ff00,#ffff00,#0000ff,#00ffff,#ff00ff,#ffffff"`

- `--ci`
  Optional. Selects which palette color to use for the effect.
  - `0` to `6` picks one of the 7 colors
  - `7` loops through all 7 colors
    Default value is `7`

### Default colors

If `--colors` is not provided, these 7 colors are used:

```text
#ff0000, #00ff00, #ffff00, #0000ff, #00ffff, #ff00ff, #ffffff
```

### Available effects

- `static`
- `breathe`
- `fade`
- `getting-off`
- `little-stars`
- `laser`
- `wave`
- `neon`
- `raindrop`
- `ripple`
- `wave2`
- `swirl`

### Example commands

Solid color using the first palette entry:

```bash
./target/debug/Firefly-cli --effect static --ci 0
```

Without lights:
```bash
./target/debug/Firefly-cli --effect getting-off
```

Static mode with color loop:

```bash
./target/debug/Firefly-cli --effect static --ci 7
```

Breathing effect:

```bash
./target/debug/Firefly-cli --effect breathe
```

Fade effect:

```bash
./target/debug/Firefly-cli --effect fade
```

Little stars effect:

```bash
./target/debug/Firefly-cli --effect little-stars
```

Laser effect:

```bash
./target/debug/Firefly-cli --effect laser
```

Wave effect:

```bash
./target/debug/Firefly-cli --effect wave
```

Neon effect:

```bash
./target/debug/Firefly-cli --effect neon
```

Raindrop effect:

```bash
./target/debug/Firefly-cli --effect raindrop
```

Ripple effect using the third palette color:

```bash
./target/debug/Firefly-cli --effect ripple --ci 2
```

Wave2 effect:

```bash
./target/debug/Firefly-cli --effect wave2
```

Swirl effect:

```bash
./target/debug/Firefly-cli --effect swirl
```

Custom 7-color palette:

```bash
./target/debug/Firefly-cli --effect wave --colors "#ff0000,#00ff00,#ffff00,#0000ff,#00ffff,#ff00ff,#ffffff"
```

Show built-in CLI help:

```bash
./target/debug/Firefly-cli --help
```

### Notes

- `--effect` is required.
- `--colors` must contain exactly 7 valid 6-digit hex colors.
- `--ci` must be between `0` and `7`.
- If USB permissions are not configured on Linux, the program may fail to access the keyboard.


## Acknowledgements 
- This stackoverflow [answer](https://stackoverflow.com/questions/37943825/send-hid-report-with-pyusb/52368526#52368526) which came in clutch, while I was breaking my head over why HIDAPI was not working.
- Low Byte Productions [Video](https://www.youtube.com/watch?v=is9wVOKeIjQ)