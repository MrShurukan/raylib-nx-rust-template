# Starting your own game

The starter shows a `Hello World` screen and supports `ZL + Minus` to reset the current session.
The structures are intentionally small. Add the fields and states needed by your game and you are good to go!

## What lives where

Paths below are relative to the repository root.

| File                                          | Main structures                                               | Responsibility                                                                                           |
|-----------------------------------------------|---------------------------------------------------------------|----------------------------------------------------------------------------------------------------------|
| `rust_game/src/lib.rs`                        | `App`, `Game`, `Gamepad`, `Frame`                             | Create the application and run input, update, and draw each frame.                                       |
| `rust_game/src/game/mod.rs`                   | `Game`                                                        | Own a `Session` and `Assets`, connect the components, and handle reset. Screen dimensions live here too. |
| `rust_game/src/game/world.rs`                 | `World`                                                       | Store game data and implement rules: movement, collisions, turns, victory conditions.                    |
| `rust_game/src/game/controller.rs`            | `Session`, `GameState`, `ErrorInfo`                           | Translate input into world actions and decide when screen states change.                                 |
| `rust_game/src/game/input.rs`                 | `GameInput`                                                   | Read gamepad buttons and sticks once per frame.                                                          |
| `rust_game/src/game/render.rs`                | —                                                             | Draw the world, animations, and UI using `Frame`.                                                        |
| `rust_game/src/game/assets.rs`                | `Assets`                                                      | Load and own textures and other resources.                                                               |
| `rust_game/src/game/util.rs`                  | `SinePulser`                                                  | Optional animation helper for pulsing values.                                                            |
| `rust_game/src/raylib/mod.rs`                 | `App`, `Frame`, `Texture`, `Vec2`, `Rect`, `Color`, `Gamepad` | Ergonomic wrapper around the available raylib functions.                                                 |
| `rust_game/src/raylib/sys.rs`                 | C declarations                                                | Raw FFI for the wrapper.                                                                                 |
| `rust_game/src/runtime.rs`, `panic_buffer.rs` | allocator, panic handler, `PanicBuffer`                       | Platform runtime and fatal error reporting. Usually left alone.                                          |
| `source/main.c`                               | —                                                             | Mount RomFS, call `rust_main`, and unmount RomFS.                                                        |
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

Add session data to `world.rs`, initialize it in `World::new`, and implement actions as methods.
For example, a player position and movement:

```rust
use crate::raylib::Vec2;

pub struct World {
    player_pos: Vec2,
}

impl World {
    pub fn new() -> Self {
        Self { player_pos: Vec2::new(320.0, 240.0) }
    }

    pub fn player_pos(&self) -> Vec2 {
        self.player_pos
    }

    pub fn move_player(&mut self, delta: Vec2) {
        self.player_pos += delta;
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
session.world.move_player(input.movement * (speed * dt));
```

`dt` is in seconds. Use `pressed` for one-time actions and `down` for held buttons.

## 4. Draw the world

Replace Hello World in `render.rs` with:

```rust
frame.circle(session.world.player_pos(), 20.0, Color::WHITE);
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
use super::world::TurnOutcome;

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
inside `game/` and declare them in `game/mod.rs`. Let `World` own these smaller structures.

If target selection becomes complicated, store the selected enemy and attack in `Session`.
The renderer can highlight the target and show expected damage. On confirmation, the controller calls `World::attack`
to validate and apply the action. Previewing an attack shouldn't reduce health or spend action points.

Keep files focused on responsibilities rather than splitting one large implementation into arbitrary chunks.
There is no separate logic crate or test harness in this minimal starter. Those can be introduced later if the game's
rules become complex enough to benefit from host-side tests.

For more raylib functionality, add the matching C declaration in `raylib/sys.rs` and the ergonomic method in
`raylib/mod.rs`. Check signatures against the actual `raylib-nx` headers used for your build.
