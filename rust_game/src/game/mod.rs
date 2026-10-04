mod assets;
mod controller;
pub mod input;
mod render;
mod util;

use crate::raylib::{Frame, TextureError};
use assets::Assets;
use controller::Session;
use input::GameInput;

pub const SCREEN_WIDTH: usize = 1280;
pub const SCREEN_HEIGHT: usize = 720;

pub struct Game {
    /// It is encouraged to separate your world info from assets
    /// as it makes it easier to avoid multiple borrows of the entire game state.
    session: Session,
    assets: Assets,
}

impl Game {
    pub fn new() -> Result<Self, TextureError> {
        Ok(Self {
            session: Session::new(),
            assets: Assets::load()?,
        })
    }

    pub fn reset(&mut self) {
        // Keep loaded assets. Only info belonging to the current session is reset.
        self.session = Session::new();
    }

    pub fn update(&mut self, input: &GameInput, dt: f32) {
        if input.reset {
            self.reset();
            return;
        }

        controller::update(&mut self.session, input, dt);
    }

    pub fn draw(&self, frame: &mut Frame) {
        render::draw(&self.session, &self.assets, frame);
    }
}
