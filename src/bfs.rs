use std::collections::{BinaryHeap, HashMap};
use super::{Queue, QueueItem, Pathing, UniformMoves, CostPath, Map};

/// BreadthFirstSearch
///
/// Struct implementing a simple BFS algorithm
pub struct BreadthFirstSearch <'a> {
    pub queue: Queue<'a>,
    pub visited: HashMap<&'a str, Option<&'a str>>,
}

impl <'a> BreadthFirstSearch <'a> {
    pub fn new() -> Self {
        BreadthFirstSearch {
            queue: BinaryHeap::new(),
            visited: HashMap::new(),
        }
    }
}

impl <'a> UniformMoves <'a> for BreadthFirstSearch <'a> {}

impl <'a> Pathing <'a> for BreadthFirstSearch <'a> {
    fn calculate_path(&mut self, map: &'a Map, start: &'a str, end: &'a str) -> CostPath {
        self.queue.clear();
        self.visited.clear();

        self.visited.insert(start, None);
        self.queue.push(QueueItem (start));

        while let Some(item) = self.queue.pop() {
            if item.0 == end {
                break;
            }

            if let Some(current) = map.nodes.get(item.0) {
                for neighbour in &current.neighbours {
                    if let Some(neighbour_node) = map.nodes.get(neighbour.0) {
                        if !self.visited.contains_key(&neighbour.0[..]) {
                            self.visited.insert(neighbour.0, Some(item.0));
                            self.queue.push(QueueItem (neighbour.0));
                        }
                    }
                }
            }
        }

        let path = self.reconstruct_path(&self.visited, end);
        let cost = path.len() as u32;

        return CostPath {
            path,
            cost
        };
    }
}

