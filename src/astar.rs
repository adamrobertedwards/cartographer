use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;
use super::{{PriorityQueue, PriorityQueueItem, Pathing, WeightedMoves, CostMap, CostPath, Map}, map::Position, distances::{Distances, Euclidean, Manhattan, Chebyshev}};


/// A*
///
/// Implementation of A* shortest path
pub struct AStar <'a> {
    pub queue: PriorityQueue<'a>,
    pub visited: HashMap<&'a str, Option<&'a str>>,
    heuristic: Distances,
}

impl <'a> AStar <'a> {
    pub fn new() -> Self {
        AStar {
            queue: BinaryHeap::new(),
            visited: HashMap::new(),
            heuristic: Distances::Euclidean,
        }
    }
    
    pub fn set_heuristic(&mut self, heuristic: Distances) {
        self.heuristic = heuristic;
    }

    pub fn heuristic_cost(&self, start: &Position, goal: &Position) -> u32 {
        match self.heuristic {
            Distances::Manhattan => Manhattan::calculate(start, goal),
            Distances::Chebyshev => Chebyshev::calculate(start, goal),
            Distances::Euclidean => Euclidean::calculate(start, goal),
            
        }
    }
}

impl <'a> WeightedMoves <'a> for AStar <'a> {}

impl <'a> Pathing <'a> for AStar <'a> {
    fn calculate_path (&mut self, map: &'a Map, start: &'a str, end: &'a str) -> CostPath {
        let mut costs: CostMap = HashMap::new();
        let destination_node = map.nodes.get(end).unwrap();

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
                for next in &current.neighbours {
                    if let Some(next_node) = map.nodes.get(next.0) {
                        let new_cost = costs.get(item.id).unwrap() + next.1;
                        let cost_now = costs.get(next.0 as &str); 

                        if cost_now.is_none() || &new_cost < cost_now.unwrap() {
                            costs.insert(next.0, new_cost);

                            let priority = new_cost + AStar::heuristic_cost(&self, &next_node.position, &destination_node.position);

                            self.queue.push(
                            PriorityQueueItem {
                                    id: next.0,
                                    priority: Reverse(priority),
                                }
                            );

                            self.visited.insert(next.0, Some(item.id));
                        }
                    }
                }
            }
        }

        let path = self.reconstruct_path(&self.visited, end);
        let cost = *costs.get(&end).unwrap_or(&0);

        return CostPath {
            path,
            cost
        };
    }
}
