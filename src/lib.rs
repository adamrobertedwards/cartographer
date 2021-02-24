pub mod map;
pub mod bfs;
pub mod dijkstra;
pub mod astar;

use std::{collections::{HashMap}, vec};
use std::cmp::{Ordering, Reverse};
use map::Map;

#[derive(Debug)]
pub struct CostPath {
    path: Vec<String>,
    cost: u32,
}

/// QueueItem
/// 
/// Struct for storing a uniform cost node in a BinaryHeap
#[derive(Ord, Eq, PartialEq, PartialOrd)]
pub struct QueueItem<'a> (&'a str);

/// PriorityQueueItem
/// 
/// Struct for storing a node with a priority in BinaryHeap
#[derive(Eq, Debug)]
pub struct PriorityQueueItem<'a> {
    priority: Reverse<u32>,
    id: &'a str,
}

impl <'a> Ord for PriorityQueueItem<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl <'a> PartialOrd for PriorityQueueItem<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl <'a> PartialEq for PriorityQueueItem<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

pub trait Pathing <'a> {
    fn reconstruct_path(&self, visited: &HashMap<&str, Option<&str>>, end: &str) -> Vec<String> {
        let mut path = vec![end.to_string()];
        let mut current = visited.get(end);

        while let Some(node) = *current.unwrap() {
            path.push(node.to_string());
            current = visited.get(node);
        }

        path.reverse();
        return path;
    }

    fn calculate_path (&mut self, map: &'a Map, start: &'a str, end: &'a str) -> CostPath;
    fn calculate_moves(&mut self, map: &'a Map, start: &'a str, moves: u32) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
