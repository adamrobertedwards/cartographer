use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

use super::{PriorityQueue, PriorityQueueItem, Pathing, WeightedMoves, CostMap, CostPath, Map};

/// Dijkstra
///
/// Struct implementing Dijkstra's search algorithm
pub struct Dijkstra <'a> {
    pub queue: PriorityQueue<'a>,
    pub visited: HashMap<&'a str, Option<&'a str>>,
}

impl <'a> Dijkstra <'a> {
    pub fn new() -> Self {
        Dijkstra {
            queue: BinaryHeap::new(),
            visited: HashMap::new(),
        }
    }
}

impl <'a> WeightedMoves <'a> for Dijkstra <'a> {}

impl <'a> Pathing <'a> for Dijkstra <'a> {
    fn calculate_path (&mut self, map: &'a Map, start: &'a str, end: &'a str) -> CostPath {
        let mut costs: CostMap = HashMap::new();

        self.queue.clear();
        self.visited.clear();

        self.visited.insert(start, None);
        costs.insert(start, 0);

        self.queue.push(
            PriorityQueueItem {
                id: start,
                priority: Reverse(0),
            }
        );

        while let Some(item) = self.queue.pop() {
            if item.id == end {
                break;
            }

            if let Some(current) = map.nodes.get(item.id) {
                for neighbour in &current.neighbours {
                    if let Some(neighbour_node) = map.nodes.get(neighbour.0) {
                        let new_cost = costs.get(item.id).unwrap() + neighbour.1;
                        let cost_now = costs.get(neighbour.0 as &str); 

                        if cost_now.is_none() || &new_cost < cost_now.unwrap() {
                            costs.insert(neighbour.0, new_cost);

                            self.queue.push(
                                PriorityQueueItem {
                                    id: neighbour.0,
                                    priority: Reverse(new_cost),
                                }
                            );

                            self.visited.insert(neighbour.0, Some(item.id));
                        }
                    }
                }
            }
        }

        let path = self.reconstruct_path(&self.visited, end);
        let cost = *costs.get(&end).unwrap();

        return CostPath {
            path,
            cost
        };
    }
}
