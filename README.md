# Wow! Another CHIP-8 Emulator!

Rust implementation of the [CHIP-8](https://chip-8.github.io/links/) virtual machine.

NOTE: This emulator uses the original [CHIP-8 instruction set](https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set). Games that use the [S-CHIP 1.1 specification](http://devernay.free.fr/hacks/chip8/schip.txt) may not work correctly.

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

## Keyboard Shortcuts

| Key | Action                                   |
| --- | ---------------------------------------- |
| T   | Suspend execution                        |
| .   | Step through next cycle (when suspended) |
| Esc | Quit the emulator                        |

## Logging

Set the `RUST_LOG` env var to `debug` to see log messages in the console.

## Compatibility

The following is a list of games that have been tested with this emulator (far more to come):

| Game                  | Year  | Author            | Status | Notes                               |
| --------------------- | ----- | ----------------- | ------ | ----------------------------------- |
| 8CE Attorney - Disc 1 | 2016  | SystemLogoff      | ✅     | Haven't tested full game            |
| 8CE Attorney - Disc 2 | 2016  | SystemLogoff      | ✅     | Haven't tested full game            |
| 8CE Attorney - Disc 3 | 2016  | SystemLogoff      | ✅     | Haven't tested full game            |
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

## Roadmap

- Eliminate flickering
- Add compiler/decompiler
- Improve debugging
- Add SCHIP support
