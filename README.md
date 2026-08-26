# raylib-nx-rust-template

A minimal Rust + raylib-nx starter template for Nintendo Switch homebrew development.

The goal of this project is to provide a small, understandable foundation for writing Switch homebrew games mostly in Rust while using [raylib-nx] as the graphics/input backend.

This is **not a game engine**. The intended development style is closer to raylib, Processing, or p5.js:

```rust
while app.running() {
    let dt = app.delta_time();

    // Update game state...

    let mut frame = app.begin_frame(Color::BLACK);

    frame.rect(...);
    frame.circle(...);
    frame.text(...);
}
```

The game logic is written in Rust. A tiny C entrypoint and the devkitPro toolchain are used to produce the final `.nro`.

## Status

This project is experimental and currently tested only with:

* Windows 11
* WSL2
* Ubuntu 24.04
* devkitPro / devkitA64
* libnx
* raylib-nx
* Rust nightly
* `aarch64-nintendo-switch-freestanding`

Other host environments may work, but are currently untested.

## Why this exists

Rust has a Nintendo Switch target:

```text
aarch64-nintendo-switch-freestanding
```

but it is a freestanding `no_std` target.

This means a normal desktop Rust application cannot simply be compiled for Switch with the complete Rust standard library.

This template uses a different architecture:

```text
Rust game code
    |
    | core + alloc
    v
Rust static library
    |
    | C ABI
    v
devkitPro linker
    |
    +-- raylib-nx
    +-- Mesa / EGL / GLES2
    +-- libnx
    +-- newlib
    |
    v
Nintendo Switch .nro
```

Rust provides the game code while devkitPro remains responsible for the platform runtime and final executable.

The Rust global allocator delegates allocations to newlib, so standard `alloc` types such as these work normally:

```rust
Vec<T>
String
Box<T>
```

The current project has been tested with repeated allocations, vector growth, string formatting, deallocation, and over-aligned allocations.

## Project structure

A typical layout is:

```text
.
├── Makefile
├── build.sh
├── source/
│   └── main.c
└── rust_game/
    ├── Cargo.toml
    ├── rust-toolchain.toml
    ├── .cargo/
    │   └── config.toml
    └── src/
        ├── lib.rs
        ├── runtime.rs
        └── raylib/
            ├── mod.rs
            └── sys.rs
```

`source/main.c` is the native Switch entrypoint, should remain tiny.

`runtime.rs` contains Rust runtime glue such as the global allocator, panic handler, and allocation error handler.

`raylib/sys.rs` contains unsafe raw C ABI declarations for raylib.

`raylib/mod.rs` contains the safe ergonomic Rust API.

`lib.rs` contains the game/application entrypoint.

## Prerequisites

You need a Nintendo Switch capable of running homebrew (or an emulator, I tested this project with Eden) and a working way to copy/run `.nro` applications.

This README only covers the development toolchain.

## 1. Install WSL2

This project is currently developed and tested inside Ubuntu running under WSL2.

I used Ubuntu 24.04:

```powershell
# As admin PowerShell
wsl --install -d Ubuntu-24.04
```

All remaining commands in this README should be executed inside Ubuntu unless stated otherwise.

It is recommended to keep development files inside the Linux filesystem, for example:

```text
~/dev
```

rather than under:

```text
/mnt/c/...
```

**Please note:** Makefile will rely on `~/dev/` folder in this example, so if you want to install your packages somewhere
you will have to edit Makefile, more on that in [section 7](#7-configure-the-raylib-nx-path)

## 2. Install basic Linux development tools

```bash
sudo apt update

sudo apt install -y \
    build-essential \
    git \
    curl \
    wget \
    cmake \
    ninja-build \
    pkg-config \
    zip \
    unzip
```

## 3. Install devkitPro

## 3. Install devkitPro

devkitPro uses its own `pacman` package manager for toolchains and platform libraries.

On Debian/Ubuntu systems, install the devkitPro pacman bootstrap first:

```bash
cd /tmp

wget https://apt.devkitpro.org/install-devkitpro-pacman
chmod +x install-devkitpro-pacman
sudo ./install-devkitpro-pacman
```

**Please note:** wget may fail with 403. In that case, refer to the instructions below (but I did something easier, I just opened the website and copied the script manually)

The official devkitPro pacman installation instructions are available here:

[https://devkitpro.org/wiki/devkitPro_pacman](https://devkitpro.org/wiki/devkitPro_pacman)

After `dkp-pacman` is available, install the Switch development toolchain:

```bash
sudo dkp-pacman -Syu

sudo dkp-pacman -S --needed \
    switch-dev \
    dkp-toolchain-vars \
    switch-mesa
```

`switch-dev` installs the basic Switch development stack, including devkitA64 and libnx.

`dkp-toolchain-vars` provides helper scripts that configure the devkitPro environment (without it you won't get proper tools on path).

`switch-mesa` provides the Switch EGL/OpenGL ES implementation required by raylib-nx. Otherwise won't compile.

### Important: configure the devkitPro environment

On some installations, devkitA64 may be physically installed but not available in your interactive shell.


Load the Switch environment explicitly:

```bash
source /opt/devkitpro/switchvars.sh
```

To make this permanent for Bash:

```bash
echo 'source /opt/devkitpro/switchvars.sh' >> ~/.bashrc
source ~/.bashrc
```

Verify the toolchain:

```bash
which aarch64-none-elf-gcc
which aarch64-none-elf-g++
which elf2nro
which nacptool
```

You should see paths under `/opt/devkitpro`.

You can also verify:

```bash
echo "$DEVKITPRO"
```

Expected:

```text
/opt/devkitpro
```

A `DEVKITA64` environment variable is not required by this template. The current devkitPro environment scripts use `$DEVKITPRO/devkitA64` and configure `PATH` directly.

## 4. Verify Mesa / OpenGL ES

raylib-nx uses EGL and OpenGL ES on Switch.

Verify that the Switch GLES2 headers exist:

```bash
test -f /opt/devkitpro/portlibs/switch/include/GLES2/gl2.h \
    && echo "GLES2 OK"
```

You should see:

```text
GLES2 OK
```

If compilation fails with:

```text
fatal error: GLES2/gl2.h: No such file or directory
```

install:

```bash
sudo dkp-pacman -S --needed switch-mesa
```

Notice **dkp-pacman** here.

Installing Ubuntu's desktop Mesa development package does **not** solve this problem.

## 5. Build raylib-nx

Clone the `luizpestana/raylib-nx` repository from GitHub.

A convenient layout is:

```text
~/dev/
├── raylib-nx/
└── raylib-nx-rust-template/
```

Build raylib-nx for Switch:

```bash
cd ~/dev/raylib-nx/src

make PLATFORM=PLATFORM_NX -j"$(nproc)"
```

After a successful build, verify:

```bash
ls -lh libraylib.a
```

Optionally, raylib-nx examples can also be built as a toolchain sanity check:

```bash
cd ~/dev/raylib-nx/examples

make PLATFORM=PLATFORM_NX -j"$(nproc)"
```

This should produce Switch `.nro` example applications.

## 6. Install Rust

Install Rust using `rustup`.

This template requires a nightly compiler because the Switch target does not ship with a normal precompiled Rust standard library and the project uses `build-std`.

The repository contains a `rust-toolchain.toml` that selects the appropriate toolchain.

Make sure `rust-src` is installed:

```bash
rustup toolchain install nightly --component rust-src
```

Verify:

```bash
rustc --version
cargo --version
rustup show active-toolchain
```

The project should select nightly automatically when commands are executed inside `rust_game`.

## Rust target configuration

The project uses:

```text
aarch64-nintendo-switch-freestanding
```

and builds:

```text
core
alloc
compiler_builtins
```

from source.

The relevant `.cargo/config.toml` configuration is similar to:

```toml
[build]
target = "aarch64-nintendo-switch-freestanding"

[unstable]
build-std = ["core", "alloc", "compiler_builtins"]

[target.aarch64-nintendo-switch-freestanding]
rustflags = [
    "-C", "relocation-model=pic",
]
```

The Rust application is built as a static library and linked into the final Switch executable by devkitPro.

## 7. Configure the raylib-nx path

The Makefile needs to know where `raylib-nx` was built.

The default template assumes:

```text
~/dev/raylib-nx
```

and should contain something similar to:

```make
RAYLIB_DIR ?= $(HOME)/dev/raylib-nx
RAYLIB_LIB := $(RAYLIB_DIR)/src/libraylib.a
```

If your copy of raylib-nx is somewhere else, either edit `RAYLIB_DIR` in the Makefile or override it when invoking make:

```bash
make RAYLIB_DIR=/your/path/to/raylib-nx
```

## 8. Build the template

The easiest way is:

```bash
./build.sh
```

or manually:

```bash
make clean
make -j"$(nproc)"
```

The Makefile first invokes Cargo to build the Rust static library and then uses devkitPro to produce the final Switch executable.

A successful build should produce:

```text
*.elf
*.nacp
*.nro
```

The `.nro` is the file to copy to your Switch.

## IDE support

RustRover and VS Code with rust-analyzer can both be used with WSL (using a remote connection, i.e. SSH, not just opening the files natively in Windows)

The library target should also disable normal Rust test targets:

```toml
[lib]
crate-type = ["staticlib"]
test = false
bench = false
doctest = false
```

The Switch target does not provide the normal Rust test harness.

### `alloc_error_handler` false positive

rust-analyzer may underline:

```rust
#[alloc_error_handler]
```

and report something similar to:

```text
this built-in macro is not implemented
```

If `cargo check` and `cargo build` succeed, this is currently only an IDE/rust-analyzer limitation and can be ignored.

The handler is intentionally isolated in `runtime.rs` so this diagnostic does not affect normal game code.

## Troubleshooting

### `aarch64-none-elf-gcc: command not found`

First verify that the compiler exists:

```bash
ls /opt/devkitpro/devkitA64/bin/aarch64-none-elf-gcc
```

Then load the environment:

```bash
source /opt/devkitpro/switchvars.sh
```

If `switchvars.sh` does not exist, install:

```bash
sudo dkp-pacman -S dkp-toolchain-vars
```

### `GLES2/gl2.h: No such file or directory`

Install the Switch Mesa port:

```bash
sudo dkp-pacman -S switch-mesa
```

Note the **dkp-pacman** usage.

### Hundreds of C++ linker errors from `libEGL.a`

Make sure the final linker is C++:

```make
export LD := $(CXX)
```

### `can't find crate for core`

Make sure:

* the project is using Rust nightly;
* `rust-src` is installed;
* `.cargo/config.toml` enables `build-std`;
* the IDE has reloaded the Cargo project.

Try from a terminal:

```bash
cargo check
cargo build --release
```

If both succeed but the IDE still reports an error, the problem is likely the IDE code model rather than the actual build.

## Design philosophy

This repository intentionally keeps the abstraction small.

The intended layers are:

```text
game code
    |
safe Rust API
    |
raylib/sys.rs
    |
raylib-nx
    |
libnx / devkitPro
```

Unsafe C interoperability should stay mostly inside `raylib/sys.rs`.

The ergonomic layer currently focuses on simple APIs such as:

```rust
App
Frame
Gamepad
Vec2
Rect
Color
Text
```

Additional raylib functionality will be added gradually as real games require it.

Potential future additions include:

```text
Texture
Font
Sound
Music
Camera2D
RenderTexture
Shader
Touch
```

The intention is to evolve this repository alongside actual homebrew games rather than attempting to bind the complete raylib API upfront.

## License

The code in this repository is licensed under the MIT License.

raylib-nx is a separate external dependency and uses its own license. This repository does not claim ownership of raylib or raylib-nx.

See `LICENSE` for the license covering this repository.

## Disclaimer

This project is an unofficial homebrew development project and is not affiliated with or endorsed by Nintendo, raylib, devkitPro, or switchbrew.
