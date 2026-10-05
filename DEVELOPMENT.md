# Starting a game

[Overview](README.md) · [Setup](SETUP.md)

The starter displays **Hello World** and supports `ZL + Minus` to reset. This guide builds one small example: move a player with the left stick, reach a finish line, and reset. The snippets describe edits to make yourself; the template does not include this example yet.

All source paths below are relative to the repository root.

## Where things belong

| File | Responsibility |
| --- | --- |
| `game_core/src/world.rs` | Game data, rules, and outcomes |
| `rust_game/src/game/input.rs` | Read buttons and sticks into `GameInput` |
| `rust_game/src/game/controller.rs` | Own `Session`, translate input into actions, and change screen states |
| `rust_game/src/game/render.rs` | Draw the world and UI |
| `rust_game/src/game/assets.rs` | Load and own resources |
| `rust_game/src/game/mod.rs` | Own session and assets; connect update, draw, and reset |

`rust_game/src/lib.rs` runs input → update → begin frame → draw. Dropping `Frame` calls `EndDrawing`.

Keep raylib, textures, and gamepad polling outside `game_core`. Rendering reads state without applying game actions. `Session` stores selections and screen state; `World` validates and applies rules.

## Name the project

- The project folder name determines the output NRO filename.
- Set `APP_TITLE`, `APP_AUTHOR`, and `APP_VERSION` in the Makefile.
- Change the window title in `rust_game/src/lib.rs`.
- If you rename the `rust_game` Cargo package, update `RUST_LIB` in the Makefile, or set `[lib] name = "rust_game"` to preserve the archive name.

## Add a world

Replace the empty `World` in `game_core/src/world.rs` with:

```rust
pub struct World {
    player_pos: (f32, f32),
}

impl World {
    pub fn new() -> Self {
        Self { player_pos: (320.0, 360.0) }
    }

    pub fn player_pos(&self) -> (f32, f32) {
        self.player_pos
    }

    pub fn move_player(&mut self, dx: f32, dy: f32) {
        self.player_pos.0 += dx;
        self.player_pos.1 += dy;
    }

    pub fn reached_finish(&self) -> bool {
        self.player_pos.0 >= 1000.0
    }
}
```

This example has free movement and a finish line at world x = 1000. Add boundaries and collisions when needed. For board games, use checked position types instead of unrestricted coordinates.

For actions that can fail, validate before changing data and return an error without partially applying the action. Return gameplay outcomes from the world; the controller chooses the next screen or animation.

## Connect input

In `input.rs`, add `Vec2` to the raylib imports and add this field to `GameInput`:

```rust
pub movement: Vec2,
```

In the `Self` initializer in `GameInput::read`, keep the reset input and add:

```rust
movement: gamepad.left_stick().deadzone(0.2),
```

In the `GameState::Running` branch of `controller::update`, add:

```rust
let speed = 200.0;
let delta = input.movement * (speed * dt);
session.world.move_player(delta.x, delta.y);
```

`dt` is in seconds. Use `pressed` for one-time actions and `down` for held buttons. Simulation-heavy games may need a fixed update step later.

## Draw the player

In `render::draw_running`, replace Hello World with:

```rust
frame.line((1000, 0).into(), (1000, 720).into(), 2.0, Color::GREEN);
frame.circle(session.world.player_pos().into(), 20.0, Color::WHITE);
```

The example draws world coordinates directly as screen coordinates. Keep camera transforms, layout, colors, and UI in the renderer. `session.elapsed_time` is available for visual effects.

Use `frame.const_text(c"...", ...)` for fixed text. `frame.text(&str, ...)` allocates a temporary C string; its input must not contain embedded NUL bytes.

## Add a finish screen

Add `GameOver` to `GameState` in `controller.rs`. After movement in the `Running` branch, add:

```rust
if session.world.reached_finish() {
    session.state = GameState::GameOver;
}
```

Add `GameState::GameOver => {}` to the controller's match so movement stops after finishing.

In `render::draw`, add this match arm:

```rust
GameState::GameOver => {
    draw_running(session, assets, frame);
    frame.const_text(
        c"Finished! Press ZL + Minus to reset.",
        (100, 100).into(),
        24,
        Color::WHITE,
    );
},
```

The existing `Game::reset` recreates `Session` and keeps `Assets`. No separate reset handler is needed for `GameOver`.

State changes affect drawing in the same frame. If a later game needs an animation before changing screens, keep transition timing in the controller. Ordinary invalid selections should show feedback; use `Error` when the session cannot continue normally.

## Add a texture when needed

To replace the circle, put your own PNG at `romfs/textures/player.png`. Replace the empty asset definition in `assets.rs` with:

```rust
use crate::raylib::{Texture, TextureError};

pub struct Assets {
    pub player: Texture,
}

impl Assets {
    pub fn load() -> Result<Self, TextureError> {
        Ok(Self {
            player: Texture::load(c"romfs:/textures/player.png")?,
        })
    }
}
```

Replace the circle draw call with:

```rust
frame.texture(&assets.player, session.world.player_pos().into());
```

Texture positions mark the top-left corner; the earlier circle position marks its center. Adjust the rendering offset to preserve the same visual center.

Assets load once and survive reset. Keep the owning `Game` within the lifetime of `App` and use graphics resources on the context's thread. Partial load failures clean up previously loaded textures through `Drop`; the current entrypoint returns an error code if `Game::new` fails.

## Testing rules

Run from the project root in WSL:

```bash
./test.sh
./test.sh finish                 # Filter test names
./test.sh -- --nocapture         # Show test output
```

The script runs `cargo test` inside `game_core`, keeping the sibling Switch configuration out of host tests. You can also run `cargo test` directly from `game_core`. The starter has no tests until you add them.

Put unit tests in a `#[cfg(test)] mod tests` beside the rule, or scenario tests in `game_core/tests/`. Integration tests import public types through `game_core`, for example `use game_core::world::World`.

For the example, useful cases are the finish boundary (before, exactly at, and beyond x = 1000) and movement across it. For a larger game, prioritize rule interactions, invalid actions leaving state unchanged, and sequences leading to victory.

The host test harness supplies the runtime while `game_core` stays `no_std`. Keep the allocator and panic handler in `rust_game`. Run `./build.sh` and check on Switch separately for graphics, input, and platform integration.

## Grow the modules gradually

Split rules into focused `game_core` modules when distinct concepts emerge, such as players, enemies, or combat. Keep selections and animation state in `Session`; keep textures in `Assets`.

To expose another raylib function, add its declaration to `raylib/sys.rs` and a wrapper to `raylib/mod.rs`. Check the signature against your actual raylib-nx headers, and consider context, ownership, and thread requirements before exposing a safe method.

The platform entrypoint (`source/main.c`), allocator (`runtime.rs`), and panic formatter (`panic_buffer.rs`) usually need no changes during ordinary game development.
