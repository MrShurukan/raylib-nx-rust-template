# raylib-nx-rust-template

A small Rust + raylib-nx template for Nintendo Switch homebrew games.

Game code uses `core` and `alloc`. A tiny C entrypoint and devkitPro link the Rust static library with [raylib-nx](https://github.com/luizpestana/raylib-nx) and produce a `.nro` executable.

The starter displays **Hello World**. It provides a game loop, a focused Rust wrapper for drawing and input, resource cleanup through `Drop`, a custom allocator, and a graphical panic handler. Add raylib functions as your game needs them.

## Build and test

Run in WSL from the project root, with Cargo and devkitPro on PATH:

```bash
./build.sh          # Build the Switch executable
./test.sh           # Test platform-independent rules on the host
./build.sh clean    # Remove Switch build outputs
```

The output is `<project-folder-name>.nro` in the project root. Copy it to your Switch using your usual homebrew workflow. Regular builds track changes; cleaning before every build is unnecessary.

The build expects raylib-nx at `~/dev/raylib-nx`. To use another location:

```bash
./build.sh RAYLIB_DIR=/your/path/to/raylib-nx
```

The empty starter has no tests yet. Host tests do not exercise graphics, input, or Switch integration.

## Documentation

| Guide | Contents |
| --- | --- |
| [Setup](SETUP.md) | WSL toolchain installation, IDE configuration, and troubleshooting |
| [Development](DEVELOPMENT.md) | Module responsibilities, a first game, resources, and tests |

## Project layout

| Path | Purpose |
| --- | --- |
| `game_core/` | Platform-independent game data and rules; host-testable `no_std` crate |
| `rust_game/src/game/` | Input, session state, rendering, and assets |
| `rust_game/src/raylib/` | Rust wrapper and raw raylib FFI |
| `rust_game/src/runtime.rs` | Allocator and fatal error handlers |
| `source/main.c` | Mount RomFS, call Rust, and unmount RomFS |
| `romfs/` | Assets packaged into the NRO |
| `Makefile`, `build.sh`, `test.sh` | Build and test entrypoints |

Each frame reads input, updates the game, begins drawing, and renders. Dropping `Frame` ends drawing. Session data and assets are separate, so resetting keeps loaded resources.

## Status and scope

Experimental; tested with Windows 11, WSL2 / Ubuntu 24.04, devkitPro / devkitA64, libnx, raylib-nx, and Rust nightly. Other host environments are untested.

The target is `aarch64-nintendo-switch-freestanding`. The template builds `core`, `alloc`, and `compiler_builtins` from source; the full Rust standard library is unavailable. `Vec`, `String`, and `Box` use an allocator backed by newlib.

The wrapper covers a subset of raylib. Use graphics resources on the context's thread and destroy textures before closing the application. The API does not enforce every resource-lifetime requirement. Fatal-error reporting is basic: the panic screen needs a usable graphics context, and allocation failure currently stops in an infinite loop.

The intended style is a straightforward update/draw loop. Grow the modules alongside your game; there is no scene framework or ECS to learn first.

## Example game

[Purridor](https://github.com/MrShurukan/purridor): a two-player Quoridor game built on this approach. Its structure differs from the current starter.

## License

MIT; see `LICENSE` in the repository root. raylib-nx and the other dependencies have their own licenses.

This is an unofficial homebrew project, unaffiliated with Nintendo, raylib, devkitPro, or switchbrew.
