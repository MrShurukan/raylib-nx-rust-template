use crate::raylib::{Color, Frame};

use super::assets::Assets;
use super::controller::{ErrorInfo, GameState, Session};

// ====== Draw ======
pub fn draw(session: &Session, assets: &Assets, frame: &mut Frame) {
    match &session.state {
        GameState::Running => draw_running(session, assets, frame),
        GameState::Error(info) => draw_error(info, frame),
    }
}

fn draw_running(session: &Session, assets: &Assets, frame: &mut Frame) {
    // TODO: Draw your world and UI here
    frame.const_text(c"Hello World!", (100, 100).into(), 24, Color::RAY_WHITE);
}

fn draw_error(error_info: &ErrorInfo, frame: &mut Frame) {
    frame.const_text(c"Error occurred :(", (100, 20).into(), 64, Color::RED);
    frame.text(&error_info.message, (100, 100).into(), 32, Color::WHITE);
    frame.text(&error_info.origination, (100, 150).into(), 24, Color::WHITE);

    frame.const_text(c"Press ZL + Minus to reset.", (100, 400).into(), 24, Color::BLUE)
}
