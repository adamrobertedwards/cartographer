use super::map::Position;
use std::cmp::max;

pub enum Distances {
    Euclidean,
    Manhattan,
    Chebyshev
}

/// Euclidean distance
///
/// Calculate the Euclidean distance between two points
pub struct Euclidean {}
impl Euclidean {
    pub fn calculate(start: &Position, goal: &Position) -> u32 {
        let dx = (start.x - goal.x).abs();
        let dy = (start.y - goal.y).abs();
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt() as u32
    }
}

/// Manhattan distance
///
/// Calculate the manhattan distance between two points on a grid (4 way movement)
pub struct Manhattan {}
impl Manhattan {
    pub fn calculate(start: &Position, goal: &Position) -> u32 {
        ((start.x - goal.x).abs() + (start.y - goal.y).abs()) as u32
    }
}
/// Chebyshev distance
///
/// Calculate the chebyshev distance between two points on a grid (8 way movement)
pub struct Chebyshev {}
impl Chebyshev {
    pub fn calculate(start: &Position, goal: &Position) -> u32 {
        max((start.x - goal.x).abs(), (start.y - goal.y).abs()) as u32
    }
}