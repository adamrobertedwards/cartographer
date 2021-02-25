pub mod map;
pub mod bfs;
pub mod dijkstra;
pub mod astar;

use std::{collections::{HashMap, BinaryHeap}, vec};
use std::cmp::{Ordering, Reverse};
use map::Map;

#[derive(Debug)]
pub struct CostPath {
    path: Vec<String>,
    cost: u32
}

type CostMap <'a> = HashMap<&'a str, u32>;
type Queue <'a> = BinaryHeap<QueueItem<'a>>;
type PriorityQueue <'a> = BinaryHeap<PriorityQueueItem<'a>>;

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
}

pub trait UniformMoves <'a> {
    fn calculate_moves(&mut self, map: &'a Map, start: &'a str, moves: u32) -> Vec<String> {
        let mut queue: Queue = BinaryHeap::new();
        let mut visited = HashMap::new();
        let mut costs: CostMap = HashMap::new();

        visited.insert(start, None);
        costs.insert(start, 0);
        queue.push(QueueItem(start));

        while let Some(item) = queue.pop() {
            if let Some(current) = map.nodes.get(item.0) {
                for neighbour in &current.neighbours {
                    let new_cost = costs.get(item.0).unwrap() + 1;
                    let cost_now = costs.get(neighbour.0 as &str); 

                    if cost_now.is_none() && &new_cost <= &moves {
                        costs.insert(neighbour.0, new_cost);
                        visited.insert(neighbour.0, Some(item.0));
                        queue.push(QueueItem(&*neighbour.0));
                    }
                }
            }
        }

        let available: Vec<String> = visited
            .keys()   
            .map(|k| k.to_string())
            .collect();

        return available;
    }
}

pub trait WeightedMoves <'a> {
    fn calculate_moves(&mut self, map: &'a Map, start: &'a str, moves: u32) -> Vec<String> {
        let mut queue: PriorityQueue = BinaryHeap::new();
        let mut visited = HashMap::new();
        let mut costs: CostMap = HashMap::new();

        visited.insert(start, None);
        costs.insert(start, 0);
        queue.push(
            PriorityQueueItem {
                id: start,
                priority: Reverse(0),
            }
        );

        while let Some(item) = queue.pop() {
            if let Some(current) = map.nodes.get(item.id) {
                for next in &current.neighbours {
                    let new_cost = costs.get(item.id).unwrap() + next.1;
                    let cost_now = costs.get(next.0 as &str); 

                    if (cost_now.is_none() || &new_cost < cost_now.unwrap()) && &new_cost <= &moves {
                        costs.insert(next.0, new_cost);

                        queue.push(
                        PriorityQueueItem {
                                id: next.0,
                                priority: Reverse(new_cost),
                            }
                        );

                        visited.insert(next.0, Some(item.id));
                    }
                }
            }
        }

        let available: Vec<String> = visited
            .keys()   
            .map(|k| k.to_string())
            .collect();

        return available;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
