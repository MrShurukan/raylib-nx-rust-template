use crate::raylib::{Button, Gamepad};

#[derive(Debug, Clone, Copy, Default)]
pub struct GameInput {
    pub reset: bool,
    // TODO: Add input needed by your game
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down
}

impl Direction {
    fn from_gamepad(gamepad: &Gamepad) -> Option<Direction> {
        if gamepad.pressed(Button::Up) { Some(Direction::Up) } else if gamepad.pressed(Button::Right) { Some(Direction::Right) } else if gamepad.pressed(Button::Left) { Some(Direction::Left) } else if gamepad.pressed(Button::Down) { Some(Direction::Down) } else { None }
    }
}

impl GameInput {
    pub fn read(gamepad: &Gamepad) -> Self {
        Self {
            reset:
                (gamepad.pressed(Button::Minus) && gamepad.down(Button::ZL)) ||
                (gamepad.pressed(Button::ZL)    && gamepad.down(Button::Minus)),

            // TODO: More input can be added in a similar way, for instance:
            // direction: Direction::from_gamepad(gamepad),
        }
    }
}
