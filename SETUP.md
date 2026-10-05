# Setup

[Overview](README.md) · [Development](DEVELOPMENT.md)

This guide covers setup on Windows 11 with WSL2 / Ubuntu 24.04. You need a Switch capable of running homebrew, or a suitable emulator, and a way to run `.nro` files. This setup has also been tested with Eden. Device setup and file transfer are outside this guide.

If the toolchain is installed, go to [Build the template](#6-build-the-template). For failures, see [Troubleshooting](#troubleshooting).

## 1. Install WSL and basic tools

If Ubuntu is not installed, run in an administrator PowerShell:

```powershell
wsl --install -d Ubuntu-24.04
```

Run all remaining commands inside Ubuntu. Keep development files in its Linux filesystem, for example `~/dev`, rather than under `/mnt/c`.

```bash
sudo apt update
sudo apt install -y \
    build-essential \
    git curl wget \
    cmake ninja-build pkg-config \
    zip unzip
```

## 2. Install devkitPro and Switch packages

devkitPro supplies its own `pacman` package manager. Install its Debian/Ubuntu bootstrap:

```bash
cd /tmp
wget https://apt.devkitpro.org/install-devkitpro-pacman
chmod +x install-devkitpro-pacman
sudo ./install-devkitpro-pacman
```

The download may fail with HTTP 403, so be prepared to use an alternative installation method. 
If this happens, just download the file manually or consult the 
[official devkitPro pacman instructions](https://devkitpro.org/wiki/devkitPro_pacman).

Install the Switch packages:

```bash
sudo dkp-pacman -Syu
sudo dkp-pacman -S --needed \
    switch-dev \
    dkp-toolchain-vars \
    switch-mesa
```

| Package | Purpose |
| --- | --- |
| `switch-dev` | Switch tools, including devkitA64 and libnx |
| `dkp-toolchain-vars` | Toolchain environment scripts |
| `switch-mesa` | Switch EGL / OpenGL ES implementation used by raylib-nx |

Load the environment in the current shell:

```bash
source /opt/devkitpro/switchvars.sh
```

For future Bash sessions, add that line once to `~/.bashrc`. If it is not already present:

```bash
echo 'source /opt/devkitpro/switchvars.sh' >> ~/.bashrc
```

Verify the environment:

```bash
echo "$DEVKITPRO"
which aarch64-none-elf-gcc aarch64-none-elf-g++ elf2nro nacptool
test -f /opt/devkitpro/portlibs/switch/include/GLES2/gl2.h && echo "GLES2 OK"
```

With this setup, `DEVKITPRO` is `/opt/devkitpro` and tool paths are beneath it. The template does not require a separate `DEVKITA64` variable.

## 3. Build raylib-nx

The template links an externally built raylib-nx archive:

```bash
mkdir -p ~/dev
cd ~/dev
git clone https://github.com/luizpestana/raylib-nx.git
cd raylib-nx/src
make PLATFORM=PLATFORM_NX -j"$(nproc)"
ls -lh libraylib.a
```

Use an existing clone if you have one. Record its commit when you establish a working setup; the Rust FFI declarations must match the raylib-nx version you build.

Optionally build the native examples to check the platform toolchain independently of Rust:

```bash
cd ~/dev/raylib-nx/examples
make PLATFORM=PLATFORM_NX -j"$(nproc)"
```

## 4. Install Rust

Install Cargo and Rust through [rustup](https://rustup.rs/) if needed. Then install nightly and the standard-library sources:

```bash
rustup toolchain install nightly --component rust-src
```

The template uses nightly for `build-std` and `alloc_error_handler`. `rust_game/rust-toolchain.toml` selects nightly when Cargo runs inside that directory.

The checked-in `rust_game/.cargo/config.toml` selects `aarch64-nintendo-switch-freestanding`, builds `core`, `alloc`, and `compiler_builtins` from source, and requests position-independent code. You do not need to recreate this configuration manually.

For reproducibility, you can later replace the floating `nightly` channel with a dated nightly you have tested.

## 5. Get the template and configure paths

To start your own game, open the [template repository on GitHub](https://github.com/MrShurukan/raylib-nx-rust-template), select **Use this template → Create a new repository**, and choose a name for your project. Then clone the new repository beside raylib-nx:

```bash
cd ~/dev
git clone https://github.com/YOUR_USERNAME/YOUR_GAME.git
cd YOUR_GAME
```

Replace `YOUR_USERNAME` and `YOUR_GAME` with your GitHub username and repository name. See [GitHub's template guide](https://docs.github.com/en/repositories/creating-and-managing-repositories/creating-a-repository-from-a-template) for details.

To try the starter without creating a repository, clone it directly:

```bash
cd ~/dev
git clone https://github.com/MrShurukan/raylib-nx-rust-template.git
cd raylib-nx-rust-template
```

The Makefile defaults to `~/dev/raylib-nx`. If raylib-nx is elsewhere, pass `RAYLIB_DIR` when building. Moving the template itself does not require changing that variable.

Check the Rust toolchain from inside `rust_game`:

```bash
cd rust_game
rustc --version
cargo --version
rustup show active-toolchain
cd ..
```

## 6. Build the template

From the project root, with the devkitPro environment loaded:

```bash
./build.sh
```

For a different raylib-nx location:

```bash
./build.sh RAYLIB_DIR=/your/path/to/raylib-nx
```

The script runs from its own directory, stops on failure, and forwards arguments to Make. Cargo builds the Rust static library; devkitPro links and packages the executable. The final link uses the C++ driver because Mesa / EGL dependencies include C++ code.

A successful build leaves `.elf`, `.nacp`, and `.nro` files in the project root. The NRO includes `romfs/`, even when it contains only the starter placeholder. Copy the `.nro` to your device.

Regular builds are incremental: archive changes trigger relinking, and RomFS file or directory changes trigger repackaging. To remove Switch build outputs:

```bash
./build.sh clean
```

This does not remove Cargo's target directory or rebuild the external raylib-nx library. You can also invoke Make directly with `make -j"$(nproc)"`.

## Host tests and IDE

Run `./test.sh` from the project root to test `game_core` on the host. Only a working host Rust toolchain is needed. See [Testing rules](DEVELOPMENT.md#testing-rules) for details.

Open the Linux project through your editor's WSL or remote-development integration so Cargo runs inside Ubuntu. RustRover and VS Code with rust-analyzer can be used for this workflow.

`rust_game` disables normal Rust test targets because the Switch target has no standard test harness. `game_core` remains a separate Cargo package for host tests.

## Troubleshooting

### `aarch64-none-elf-gcc: command not found`

Check whether the compiler is installed:

```bash
ls /opt/devkitpro/devkitA64/bin/aarch64-none-elf-gcc
```

If present, load `source /opt/devkitpro/switchvars.sh` in your current shell. If that script is missing, install `dkp-toolchain-vars` through `dkp-pacman`. If the compiler is missing, check the `switch-dev` installation.

### `GLES2/gl2.h: No such file or directory`

```bash
sudo dkp-pacman -S --needed switch-mesa
```

Ubuntu's desktop Mesa development package does not supply the Switch headers needed by this build.

### C++ linker errors from `libEGL.a`

Check that the Makefile uses the C++ driver:

```make
export LD := $(CXX)
```

The template already includes this setting.

### `can't find crate for core`

From `rust_game`, check that nightly is selected, `rust-src` is installed, and `.cargo/config.toml` contains `build-std`. Then run:

```bash
cargo check
cargo build --release --lib
```

If command-line builds pass but the IDE reports an error, reload the Cargo project and check its WSL toolchain configuration.

### IDE warning on `alloc_error_handler`

rust-analyzer may report `this built-in macro is not implemented` for `#[alloc_error_handler]`. If command-line `cargo check` and `cargo build` pass, treat the diagnostic as an IDE limitation. If they fail too, investigate the compiler error.
