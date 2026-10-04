use alloc::string::String;

use super::input::GameInput;
use super::world::World;

/// Major divisions of the game state. Fitting additions would be something like MainMenu,
/// Settings, GameOver, etc.
#[derive(Debug)]
pub enum GameState {
    Running,
    /// Recoverable error. Similar to panic, but you can still process inputs so you can try to
    /// recover (for example restart the game)
    Error(ErrorInfo),
}

#[derive(Debug)]
pub struct ErrorInfo {
    pub message: String,
    pub origination: String,
}

/// Info belonging to the current game session. Assets live separately and survive a reset.
pub struct Session {
    pub state: GameState,
    pub world: World,
    pub elapsed_time: f32,
}

impl Session {
    pub fn new() -> Self {
        Self {
            state: GameState::Running,
            world: World::new(),
            elapsed_time: 0.0,
        }
    }
}

/// Update (runs every frame)
pub fn update(session: &mut Session, input: &GameInput, dt: f32) {
    session.elapsed_time += dt;

    match &mut session.state {
        GameState::Running => {
            // TODO: Process input and update the world here
        },
        GameState::Error(_) => {},
    }
}
