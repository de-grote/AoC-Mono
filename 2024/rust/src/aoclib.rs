use glam::IVec2;
use std::error::Error;

pub type Answer<'a> = Result<String, Box<dyn Error + 'a>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn rotate_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn rotate_left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    pub fn reverse(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }

    pub fn unit_vector(self) -> IVec2 {
        match self {
            Direction::Up => IVec2::NEG_Y,
            Direction::Right => IVec2::X,
            Direction::Down => IVec2::Y,
            Direction::Left => IVec2::NEG_X,
        }
    }
}
