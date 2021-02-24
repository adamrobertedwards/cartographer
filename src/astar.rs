use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;
use super::{{PriorityQueueItem, Pathing, CostPath, Map}, map::Position};

/// A*
///
/// Implementation of A* shortest path
pub struct AStar <'a> {
    pub queue: BinaryHeap<PriorityQueueItem<'a>>,
    pub visited: HashMap<&'a str, Option<&'a str>>,
}

impl <'a> AStar <'a> {
    pub fn new() -> Self {
        AStar {
            queue: BinaryHeap::new(),
            visited: HashMap::new(),
        }
    }

    pub fn heuristic_cost_euclidean(start: &Position, goal: &Position) -> u32 {
        ((start.x - goal.x).abs() + (start.y - goal.y).abs()) as u32
    }

    pub fn heuristic_cost_manhattan(start: &Position, goal: &Position) -> u32 {
        ((start.x - goal.x).abs() + (start.y - goal.y).abs()) as u32
    }
}

impl <'a> Pathing <'a> for AStar <'a> {
    fn calculate_path (&mut self, map: &'a Map, start: &'a str, end: &'a str) -> CostPath {
        let mut costs: HashMap<&str, u32> = HashMap::new();
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
                    let next_node = map.nodes.get(next.0).unwrap();
                    let new_cost = costs.get(item.id).unwrap() + next.1;
                    let cost_now = costs.get(next.0 as &str); 

                    if cost_now.is_none() || &new_cost < cost_now.unwrap() {
                        costs.insert(next.0, new_cost);

                        let priority = new_cost + AStar::heuristic_cost_euclidean(&next_node.position, &destination_node.position);
    
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

        let path = self.reconstruct_path(&self.visited, end);
        let cost = *costs.get(&end).unwrap();

        return CostPath {
            path,
            cost, 
        };
    }

    fn calculate_moves(&mut self, map: &'a Map, start: &'a str, moves: u32) -> Vec<String> {
        let mut costs: HashMap<&str, u32> = HashMap::new();

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
            if let Some(current) = map.nodes.get(item.id) {
                for next in &current.neighbours {
                    let new_cost = costs.get(item.id).unwrap() + next.1;
                    let cost_now = costs.get(next.0 as &str); 

                    if (cost_now.is_none() || &new_cost < cost_now.unwrap()) && &new_cost <= &moves {
                        costs.insert(next.0, new_cost);

                        self.queue.push(
                        PriorityQueueItem {
                                id: next.0,
                                priority: Reverse(new_cost),
                            }
                        );

                        self.visited.insert(next.0, Some(item.id));
                    }
                }
            }
        }

        let available: Vec<String> = self.visited
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
