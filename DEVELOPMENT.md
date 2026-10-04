# Starting your own game

The starter shows a `Hello World` screen and supports `ZL + Minus` to reset the current session.
The structures are intentionally small. Add the fields and states needed by your game and you are good to go!

## What lives where

Paths below are relative to the repository root.

| File                                          | Main structures                                               | Responsibility                                                                                           |
|-----------------------------------------------|---------------------------------------------------------------|----------------------------------------------------------------------------------------------------------|
| `rust_game/src/lib.rs`                        | `App`, `Game`, `Gamepad`, `Frame`                             | Create the application and run input, update, and draw each frame.                                       |
| `rust_game/src/game/mod.rs`                   | `Game`                                                        | Own a `Session` and `Assets`, connect the components, and handle reset. Screen dimensions live here too. |
| `game_core/src/world.rs`                      | `World`                                                       | Store game data and implement rules: movement, collisions, turns, victory conditions.                    |
| `rust_game/src/game/controller.rs`            | `Session`, `GameState`, `ErrorInfo`                           | Translate input into world actions and decide when screen states change.                                 |
| `rust_game/src/game/input.rs`                 | `GameInput`                                                   | Read gamepad buttons and sticks once per frame.                                                          |
| `rust_game/src/game/render.rs`                | —                                                             | Draw the world, animations, and UI using `Frame`.                                                        |
| `rust_game/src/game/assets.rs`                | `Assets`                                                      | Load and own textures and other resources.                                                               |
| `rust_game/src/game/util.rs`                  | `SinePulser`                                                  | Optional animation helper for pulsing values.                                                            |
| `rust_game/src/raylib/mod.rs`                 | `App`, `Frame`, `Texture`, `Vec2`, `Rect`, `Color`, `Gamepad` | Ergonomic wrapper around the available raylib functions.                                                 |
| `rust_game/src/raylib/sys.rs`                 | C declarations                                                | Raw FFI for the wrapper.                                                                                 |
| `rust_game/src/runtime.rs`, `panic_buffer.rs` | allocator, panic handler, `PanicBuffer`                       | Platform runtime and fatal error reporting. Usually left alone.                                          |
| `source/main.c`                               | —                                                             | Mount RomFS, call `rust_main`, and unmount RomFS.                                                        |
| `test.sh`                                     | —                                                             | Run game rules tests on the host, independently of the Switch toolchain.                                 |
| `Makefile`, `build.sh`                        | —                                                             | Compile Rust, link the Switch executable, and package the NRO with RomFS.                                |

The normal flow is:

1. `GameInput::read`
2. `Game::update` → `controller::update` → `World` methods
3. `App::begin_frame`
4. `Game::draw` → `render::draw`
5. Frame is dropped (`EndDrawing`)

`World` doesn't have to know about gamepad buttons, textures, or the current screen.
`render.rs` reads the session and assets; it doesn't change the rules while drawing.
`controller.rs` connects these parts. Keep the familiar update/draw style as your game grows.

## 1. Set up the project identity

- Project folder name → output NRO name.
- `Makefile`: set `APP_TITLE`, `APP_AUTHOR`, and `APP_VERSION`.
- `lib.rs`: change the window title in `App::new`.
- If you rename the Cargo package, update `RUST_LIB` in the Makefile too, or set `[lib] name = "rust_game"` to keep the archive name.

Build in WSL with devkitPro and Cargo on PATH: `./build.sh`. Changes are tracked automatically; use `./build.sh clean` only when needed.

## 2. Add world data and rules

Add game data to `game_core/src/world.rs`, initialize it in `World::new`, and implement actions as methods.
Keep this crate independent of raylib and platform code; `core` and `alloc` are available.
For example, a player position and movement:

```rust
pub struct World {
    player_pos: (f32, f32),
}

impl World {
    pub fn new() -> Self {
        Self { player_pos: (320.0, 240.0) }
    }

    pub fn player_pos(&self) -> (f32, f32) {
        self.player_pos
    }

    pub fn move_player(&mut self, dx: f32, dy: f32) {
        self.player_pos.0 += dx;
        self.player_pos.1 += dy;
    }
}
```

Add collision checks here when needed. Use a dedicated `Position` type for coordinates with their own rules, such as board cells.

## 3. Connect input to actions

In `input.rs`, import `Vec2`, add `pub movement: Vec2` to `GameInput`, and read it in `GameInput::read`:

```rust
movement: gamepad.left_stick().deadzone(0.2),
```

In the controller's `Running` branch:

```rust
let speed = 200.0;
let delta = input.movement * (speed * dt);
session.world.move_player(delta.x, delta.y);
```

`dt` is in seconds. Use `pressed` for one-time actions and `down` for held buttons.

## 4. Draw the world

Replace Hello World in `render.rs` with:

```rust
frame.circle(session.world.player_pos().into(), 20.0, Color::WHITE);
```

Keep screen layout, colors, UI, and board-to-pixel conversions here. `session.elapsed_time` is available for animation.
Use `const_text` with `c"..."` for fixed text and `text` with `&str` for dynamic text.

## 5. Add assets

Place the file at `romfs/textures/player.png`. In `assets.rs`:

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

Draw it in `render.rs`:

```rust
frame.texture(&assets.player, session.world.player_pos());
```

Texture positions mark the top-left corner. Assets load once and survive reset; keep `Game` alive inside the lifetime of `App`.

## 6. Handle results and change states

Add screen states to `GameState` and handle them in both controller and renderer.
World methods return errors or outcomes; the controller chooses the screen. Propagate errors from nested world methods with `?`.
Validate an action before changing world data.

For example, in a turn-based battle the player selects an enemy and confirms an attack.
`World::attack(target)` checks the action, applies damage, and returns `Result<TurnOutcome, BattleError>`:

```rust
// world.rs
pub enum TurnOutcome {
    TurnFinished,
    Victory,
}
```

Add `TurnTransition` and `GameOver` to `GameState`. In the controller, handle the attack result
(`target` is the selected enemy):

```rust
use alloc::format;
use game_core::world::TurnOutcome;

match session.world.attack(target) {
    Ok(TurnOutcome::TurnFinished) => session.state = GameState::TurnTransition,
    Ok(TurnOutcome::Victory) => session.state = GameState::GameOver,
    Err(error) => {
        session.state = GameState::Error(ErrorInfo {
            message: format!("Couldn't perform attack: {:?}", error),
            origination: format!("{}; line: {}", module_path!(), line!()),
        });
    },
}
```

The world reports whether the turn ended or the battle was won. The controller decides what screen or animation follows.
Use `Error` when the session cannot continue. For an ordinary invalid selection, keep the current state and show feedback instead.

Assigning `session.state` changes what the renderer draws this frame. To defer a transition, add `next_state: Option<GameState>`
to `Session` and take it at the start of update or when an animation finishes. The controller owns this timing.

`Game::reset` recreates `Session` (world, state, time) and keeps `Assets`. Store settings or other values that should survive reset separately in `Game`.

## 7. Split files as the game grows

Start with the existing modules. When `World` grows around distinct concepts, add `player.rs`, `enemy.rs`, or `combat.rs`
inside `game_core/src/` and declare them in `game_core/src/lib.rs`. Let `World` own these smaller structures.
Keep input, rendering, assets, and screen transitions in `rust_game/src/game/`.

If target selection becomes complicated, store the selected enemy and attack in `Session`.
The renderer can highlight the target and show expected damage. On confirmation, the controller calls `World::attack`
to validate and apply the action. Previewing an attack shouldn't reduce health or spend action points.

Keep files focused on responsibilities rather than splitting one large implementation into arbitrary chunks.

For more raylib functionality, add the matching C declaration in `raylib/sys.rs` and the ergonomic method in
`raylib/mod.rs`. Check signatures against the actual `raylib-nx` headers used for your build.

## 8. Test game rules

Run `./test.sh` in WSL from the project root, or `cargo test` from `game_core`.
These are host tests; the two crates intentionally remain separate Cargo packages.
Do not run the test command from `rust_game`, which selects the Switch target.

Put unit tests in a `#[cfg(test)] mod tests` beside the rule they exercise. For example, after adding
movement from section 2, append this to `game_core/src/world.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::World;

    #[test]
    fn movement_applies_both_axes() {
        let mut world = World::new();
        world.move_player(10.0, -5.0);
        assert_eq!(world.player_pos(), (330.0, 235.0));
    }
}
```

For sequences of actions, add integration tests under `game_core/tests/` and import the public API
with `use game_core::world::World;`. Prefer tests for boundaries, invalid actions leaving state unchanged,
and interactions between rules. The empty starter needs no placeholder tests.

Host tests use the standard test harness while game code remains `no_std`. The allocator and panic handler
stay in `rust_game`; do not add them to `game_core`. Run `./build.sh` separately to check Switch integration.
