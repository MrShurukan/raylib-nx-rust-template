use crate::game::input::GameInput;
use crate::raylib::{Color, Frame, TextureError};
use alloc::string::String;

type GS = GameState;

/// Major divisions of the game state. Fitting additions would be something like MainMenu,
/// Settings, GameOver, etc.
#[derive(Debug)]
pub enum GameState {
    Running(GameSubState),
    /// Recoverable error. Similar to panic, but you can still process inputs so you can try to
    /// recover (for example restart the game)
    Error(ErrorInfo),
}

#[derive(Debug)]
pub struct ErrorInfo {
    pub message: String,
    pub origination: String
}

/// Used to conveniently form an error state with file and line of origination
macro_rules! error_state {
    ($message:expr) => {
        GS::Error(
            ErrorInfo { message: $message, origination: format!("{}; line: {}", module_path!(), line!()) }
        )
    };
}

type GSS = GameSubState;

/// Captures current running state of the game, can be used to store which turn it is or what the
/// current player is doing
#[derive(Debug)]
pub enum GameSubState {
    // TODO: Fill your actual game logic states here
    BlankScreen
}

pub struct Assets {
    // TODO: Fill your romfs assets
    // for example, Textures
}

impl Assets {
    pub fn load() -> Result<Self, TextureError> {
        Ok(Self {
            // TODO: Load yours assets from romfs
            // Texture::load(c"romfs:/...")?
        })
    }
}

pub struct Game {
    state: GameState,
    /// It is encouraged to separate your world info from assets and states
    /// as it makes it easier to avoid multiple borrows of the entire game state
    world: World,

    assets: Assets,
    elapsed_time: f32
}

pub struct World {
    /// Allows for the world logic to influence next frame's state
    /// (for instance to trigger an error)
    next_state: Option<GameState>,
}
impl World {

}

impl Game {
    pub fn new() -> Self {
        Game {
            state: GS::Running(GSS::BlankScreen),
            world: World {
                next_state: None,
            },

            assets: Assets::load().unwrap(),
            elapsed_time: 0.0
        }
    }

    pub fn reset(&mut self) {
        *self = Game::new();
    }
}

// ====== Update (runs every frame) ======
impl Game {
    pub fn update(&mut self, input: &GameInput, dt: f32) {
        // Global state transition check
        if let Some(state) = self.world.next_state.take() {
            self.state = state;
        }

        self.elapsed_time += dt;

        // Reset Logic
        if input.reset {
            self.reset();
        }

        let state = &mut self.state;
        let world = &mut self.world;

        match state {
            GS::Running(GSS::BlankScreen) => {},
            GS::Error(_) => {}
        }
    }
}

// ====== Draw ======
impl Game {
    pub fn draw(&self, frame: &mut Frame) {
        match &self.state {
            GS::Running(sub_state) => self.draw_running(frame),
            GS::Error(info) => self.draw_error(info, frame),
        }
    }

    fn draw_running(&self, frame: &mut Frame) {
        frame.const_text(c"Hello World!", (100, 100).into(), 24, Color::RAY_WHITE);
    }

    fn draw_error(&self, error_info: &ErrorInfo, frame: &mut Frame) {
        frame.const_text(c"Error occurred :(", (100, 20).into(), 64, Color::RED);
        frame.text(&error_info.message, (100, 100).into(), 32, Color::WHITE);
        frame.text(&error_info.origination, (100, 150).into(), 24, Color::WHITE);

        frame.const_text(c"Press ZL + Minus to reset.", (100, 400).into(), 24, Color::BLUE)
    }
}