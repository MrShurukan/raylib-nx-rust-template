use crate::raylib::{Button, Gamepad};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down
}

impl Direction {
    fn from_gamepad(gamepad: &Gamepad) -> Option<Direction> {
        if gamepad.pressed(Button::Up)          { Some(Direction::Up) }
        else if gamepad.pressed(Button::Right)  { Some(Direction::Right) }
        else if gamepad.pressed(Button::Left)   { Some(Direction::Left) }
        else if gamepad.pressed(Button::Down)   { Some(Direction::Down) }
        else                                    { None }
    }

    pub const fn delta(self) -> (i32, i32) {
        match self {
            Self::Up    => (0, -1),
            Self::Right => (1, 0),
            Self::Down  => (0, 1),
            Self::Left  => (-1, 0),
        }
    }

    pub const fn others(self) -> [Direction; 3] {
        match self {
            Direction::Up => [Direction::Left, Direction::Down, Direction::Right],
            Direction::Right => [Direction::Left, Direction::Up, Direction::Down],
            Direction::Left => [Direction::Right, Direction::Up, Direction::Down],
            Direction::Down => [Direction::Left, Direction::Up, Direction::Right],
        }
    }

    pub const fn opposite(self) -> Direction {
        match self {
            Self::Up    => Direction::Down,
            Self::Right => Direction::Left,
            Self::Down  => Direction::Up,
            Self::Left  => Direction::Right,
        }
    }

    pub const fn exclude_opposite(self) -> [Direction; 3] {
        self.opposite().others()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GameInput {
    pub reset: bool,
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