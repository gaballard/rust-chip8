# WyAce - Wow, Yet Another CHIP-8 Emulator!

Rust implementation of the [CHIP-8 platform](https://chip-8.github.io/links/).

## Usage

Via `cargo run`:

```bash
$ cargo run PATH_TO_ROM
```

Via binary:

```bash
$ cargo build
$ ./target/debug/rust-chip8 PATH_TO_ROM
```

## Logging

Set the `RUST_LOG` env var to `debug` to see log messages in the console.

## ToDo

- [ ] Toggle logs on/off while emulator is running