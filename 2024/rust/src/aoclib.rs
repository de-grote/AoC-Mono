use glam::IVec2;
use std::error::Error;

/// Answer type for solutions
pub type Answer<'a> = Result<String, Box<dyn Error + 'a>>;

/// Get the 8 points orthogonal and diagonal to the given point
pub fn adjacent_points(point: IVec2) -> [IVec2; 8] {
    [
        point + IVec2::new(-1, -1),
        point + IVec2::new(0, -1),
        point + IVec2::new(1, -1),
        point + IVec2::new(-1, 0),
        point + IVec2::new(1, 0),
        point + IVec2::new(-1, 1),
        point + IVec2::new(0, 1),
        point + IVec2::new(1, 1),
    ]
}

/// Get the 4 points orthogonal to the given point
pub fn orthogonal_points(point: IVec2) -> [IVec2; 4] {
    [
        point + IVec2::new(0, -1),
        point + IVec2::new(-1, 0),
        point + IVec2::new(1, 0),
        point + IVec2::new(0, 1),
    ]
}

/// Represents a orthogonal direction
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub const fn rotate_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub const fn rotate_left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    pub const fn reverse(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }

    /// Right = 1x, Down = 1y, Left = -1x, Up = -1y
    pub const fn unit_vector(self) -> IVec2 {
        match self {
            Direction::Up => IVec2::NEG_Y,
            Direction::Right => IVec2::X,
            Direction::Down => IVec2::Y,
            Direction::Left => IVec2::NEG_X,
        }
    }
}
