# Chippy

Chippy is a Rust implementation of the [CHIP-8](https://chip-8.github.io/links/) interpreter. It supports both the [original CHIP-8](https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set) and the [SCHIP-1.x](http://devernay.free.fr/hacks/chip8/schip.txt) instruction sets.

No AI was used at any point in Chippy's development.

## Usage

Games are added to the `roms/` folder and referenced by filename. Filenames should include extensions.

Via `cargo run`:

```bash
$ cargo run ROM_FILENAME
```

Via binary:

```bash
$ cargo build
$ ./target/debug/rust-chip8 ROM_FILENAME
```

## Input

CHIP-8 was originally written for the COSMAC VIP computer, which used a hexadecimal keypad with the following layout:

<img src="assets/images/COSMAC-VIP-keypad-layout.gif" alt="COSMAC VIP hex keypad" height="256" />

Chippy approximates this using the left side of the keyboard:

| Key                | Effect                          |
| ------------------ | ------------------------------- |
| `1`, `2`, `3`, `4` | Hex keypad 1st row (1, 2, 3, C) |
| `Q`, `W`, `E`, `R` | Hex keypad 2nd row (4, 5, 6, D) |
| `A`, `S`, `D`, `F` | Hex keypad 3rd row (7, 8, 9, E) |
| `Z`, `X`, `C`, `V` | Hex keypad 4th row (A, 0, B, F) |

### Keyboard Shortcuts

| Key           | Effect                                    |
| ------------- | ----------------------------------------- |
| `Esc`         | Quit program                              |
| `Backspace`   | Reset                                     |
| `\`           | Show/hide debug panel                     |
| `T`           | Toggle step-by-step execution (step mode) |
| `.`           | Execute next cycle (step mode only)       |
| `J`           | Toggle CRT fade effect                    |
| `7`, `8`, `9` | CRT fade rate (slow/med/fast)             |
| `P`           | Cycle color palettes                      |
| `[`           | Zoom display out                          |
| `]`           | Zoom display in                           |
| `H`           | Toggle hi-res mode (use to debug)         |

## Logging

Set the `RUST_LOG` env var to `debug` to see log messages in the console.

## Variants

Chippy fully supports the following CHIP-8 instruction sets:

- [CHIP-8](https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set) (1977)
- [SCHIP-1.x](http://devernay.free.fr/hacks/chip8/schip.txt) (1991)

As well as partial support for CHIP-48, SCHIPC, and others (more to come!)

Use the `DEFAULT_INSTRUCTION_SET` const to select a variant before loading the program.

### Quirks

| Status | #   | Variants                                    | Instruction                    | Description                                                                                                                                                                                      |
| ------ | --- | ------------------------------------------- | ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
|        | 1   | SCHIP-1.x, SCHIPC                           | `00Cn`, `00Dn`, `00FB`, `00FC` | On the HP48 (SCHIP/SCHIPC) scrolling in lores mode only scrolls half the pixels                                                                                                                  |
| ✅     | 2   | SCHIP-1.x, SCHIPC                           | `00C0`                         | On the HP48 (SCHIP/SCHPC) opcode 00C0, so scrolling zero pixels, is not a valid opcode                                                                                                           |
| ✅     | 3   | SCHIP-1.x                                   | `00FE`, `00FF`                 | The original SCHIP-1.x did not clean the screen, leading to artifacts                                                                                                                            |
|        | 4   | CHIP-8E, XO-CHIP                            | `5xy2`, `5xy3`                 | CHIP-8E only supports X<=Y, on XO-CHIP registers are written/read in the order of X to Y                                                                                                         |
| ✅     | 5   | COSMAC variants                             | `8xy1`, `8xy2`, `8xy3`         | COSMAC based variants will reset VF                                                                                                                                                              |
| ✅     | 6   | CHIP-48, SCHIP-1.x                          | `8xy6`, `8xyE`                 | CHIP-48/SCHIP-1.x don't set vX to vY, so only shift vX                                                                                                                                           |
|        | 7   | XO-CHIP                                     | `Dxyn`, `Dxy0`                 | XO-CHIP wraps pixels instead of clipping them                                                                                                                                                    |
|        | 8   | COSMAC, HP48 variants                       | `Dxyn`, `Dxy0`                 | Original COSMAC VIP based systems (like original CHIP-8), and the HP48 based interpreters in 64x32 mode wait for the start of the next frame, the VIP sometimes even needs two screens to finish |
|        | 9   | CHIP-10                                     | `Dxyn`                         | CHIP-10 only has a hires mode                                                                                                                                                                    |
|        | 10  | CHIP-1.1                                    | `Dxyn`, `Dxy0`                 | The original SCHIP-1.1 in hires mode set VF to the number of sprite rows with collisions plus the number of rows clipped at the bottom border                                                    |
|        | 11  | SCHIP-1.x, SCHIPC                           | `Dxy0`                         | SCHIP-1.x/SCHIPC only draws 8x16 on lores                                                                                                                                                        |
|   ✅    | 12  | CHIP-48, SCHIP-1.0, SCHIP-1.1, SCHIP-MODERN | `Fx55`, `Fx65`                 | CHIP-48/SCHIP1.0 increment I only by X, SCHIP1.1/SCHIP-MODERN not at all                                                                                                                         |
|        | 13  | SCHIP-1.x, SCHIPC                           | `Fx75`, `Fx85`                 | SCHIP-1.x and SCHIPC only support v0-v7 on a real HP48                                                                                                                                           |

## ROM Compatibility

The following is a list of games that have been tested with this interpreter:

| Game                  | Year  | Author            | Status | Notes                               |
| --------------------- | ----- | ----------------- | ------ | ----------------------------------- |
| 8CE Attorney - Disc 1 | 2016  | SystemLogoff      | ✅     | Haven't tested full game            |
| 8CE Attorney - Disc 2 | 2016  | SystemLogoff      | ✅     | Haven't tested full game            |
| 8CE Attorney - Disc 3 | 2016  | SystemLogoff      | ✅     | Haven't tested full game            |
| Airplane              | ?     | Unknown           | ✅     |                                     |
| Br8kout               | 2014  | SharpenedSpoon    | ❌     | Stuck in draw loop                  |
| Breakout              | 1979  | Carmelo Cortez    | ✅     |                                     |
| Coin Flipping         | 1978  | Carmelo Cortez    | ✅     |                                     |
| Craps                 | 1978  | Carmelo Cortez    | ✅     |                                     |
| Flight Runner         | 2014  | TodPunk           | ❌     | Plays for a while then freezes      |
| Hi-Lo                 | 1978  | Jef Winsor        | ✅     |                                     |
| Kaleidoscope          | 1978  | Joseph Weisbecker | ❌     | Starts, but nothing else happens    |
| Lunar Lander          | 1979  | Udo Pernisz       | ✅     |                                     |
| Mastermind FourRow    | 1978  | Robert Lindley    | ✅     |                                     |
| Nim                   | 1978  | Carmelo Cortez    | ✅     |                                     |
| Outlaw                | 2014  | John Earnest      | ✅     |                                     |
| Pong                  | 1990  | Paul Vervalin     | ✅     |                                     |
| Pong (1-player)       | 1990? | Paul Vervalin?    | ✅     | Paul Vervalin's Pong w/CPU opponent |
| RPS                   | 2015  | SystemLogoff      | ❌     | Can't get past title screen         |
| Snek                  | 2021  | John Earnest      | ✅     |                                     |
| Tetris                | 1991  | Fran Dachille     | ✅     |                                     |

## Test Suites

| Test ROM                         | CHIP-8 | SCHIP-1.x | Notes                                  |
| -------------------------------- | ------ | --------- | -------------------------------------- |
| CHIP-8 Logo (`1-chip8-logo.ch8`) | ✅     | ✅        |                                        |
| IBM Logo (`2-ibm-logo.ch8`)      | ✅     | ✅        |                                        |
| Corax+ (`3-corax+.ch8`)          | ✅     | ✅        |                                        |
| Flags (`4-flags.ch8`)            | ✅     | ✅        |                                        |
| Quirks (`5-quirks.ch8`)          | ⚠️     | ❌        | Optional disp. wait test fail (CHIP-8) |
| Keypad (`6-keypad.ch8`)          | ✅     | ✅        |                                        |
| Beep (`7-beep.ch8`)              | ✅     | ✅        |                                        |
| Scrolling (`8-scrolling.ch8`)    |        | ❌        | SCHIP mode only                        |

## Roadmap

- ~~CRT fade effect~~
- ~~Improve debugging~~
- ~~Adjustable video scaling~~
- Add SCHIP support (In Progress)
- Move config to `settings.ini`
- Settings and flags visible in UI
- Adjustable clock speed
- ROM hotswapping
- Add compiler/decompiler
- Alert text overlay with fadeout
- Simulated timings

## License

This software is released under the MIT license. See [`LICENSE.md`](./LICENSE.md) for the full text.
