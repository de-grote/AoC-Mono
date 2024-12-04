use glam::IVec2;
use itertools::Itertools;
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

/// Get the 4 points diagnonal to the given point
pub fn diagonal_points(point: IVec2) -> [IVec2; 4] {
    [
        point + IVec2::new(-1, -1),
        point + IVec2::new(-1, 1),
        point + IVec2::new(1, -1),
        point + IVec2::new(1, 1),
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

pub trait Grid<T> {
    /// Return the item at a specified 2d location
    fn at<Q: num::ToPrimitive>(&self, pos: impl Into<(Q, Q)>) -> Option<&T>;
    /// Returns width and heigth (in that order)
    fn size(&self) -> (usize, usize);
    /// An iterator over all the rows as iterator
    fn rows<'a>(&'a self) -> impl Iterator<Item = impl Iterator<Item = &T>>
    where
        T: 'a,
    {
        let (_, y) = self.size();
        (0..y).map(move |y2| (0..).map_while(move |x2| self.at((x2, y2))))
    }
    /// An iterator over all the columns as iterator
    fn columns<'a>(&'a self) -> impl Iterator<Item = impl Iterator<Item = &T>>
    where
        T: 'a,
    {
        let (x, _) = self.size();
        (0..x).map(move |x2| (0..).map_while(move |y2| self.at((x2, y2))))
    }
    /// Transposes the array, clones every value
    fn transpose(&self) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        self.columns()
            .map(|x| x.cloned().collect_vec())
            .collect_vec()
    }
}

impl<T, A: AsRef<[T]>> Grid<T> for [A] {
    fn at<Q: num::ToPrimitive>(&self, pos: impl Into<(Q, Q)>) -> Option<&T> {
        let (x, y) = pos.into();
        self.get(y.to_usize()?)?.as_ref().get(x.to_usize()?)
    }

    fn size(&self) -> (usize, usize) {
        (
            self.first().map_or(0, |inner| inner.as_ref().len()),
            self.len(),
        )
    }
}

pub trait MutableGrid<T>
where
    Self: Grid<T>,
{
    /// Return a mutable reference to the item at a specified 2d location
    fn at_mut<Q: num::ToPrimitive>(&mut self, pos: impl Into<(Q, Q)>) -> Option<&mut T>;
}

impl<T, A: AsMut<[T]> + AsRef<[T]>> MutableGrid<T> for [A] {
    fn at_mut<Q: num::ToPrimitive>(&mut self, pos: impl Into<(Q, Q)>) -> Option<&mut T> {
        let (x, y) = pos.into();
        self.get_mut(y.to_usize()?)?.as_mut().get_mut(x.to_usize()?)
    }
}
