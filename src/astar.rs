use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;
use super::{{PriorityQueue, PriorityQueueItem, Pathing, WeightedMoves, CostMap, CostPath, Map}, map::Position};

/// A*
///
/// Implementation of A* shortest path
pub struct AStar <'a> {
    pub queue: PriorityQueue<'a>,
    pub visited: HashMap<&'a str, Option<&'a str>>,
}

impl <'a> AStar <'a> {
    pub fn new() -> Self {
        AStar {
            queue: BinaryHeap::new(),
            visited: HashMap::new(),
        }
    }

//     if node[1] == end[1]:
//     return end[0] - node[0]
//   else:
//     a = end[0] - node[0]
//     b = abs(node[1] - end[1])
//     return math.sqrt((a**2) + (b**2))

    /// Euclidean distance
    ///
    /// Calculate the Euclidean distance between two points
    pub fn heuristic_cost_euclidean(start: &Position, goal: &Position) -> u32 {
        ((start.x - goal.x).pow(2) as f32 + (start.y - goal.y).pow(2) as f32).sqrt() as u32
    }

    /// Manhattan distance
    ///
    /// Calculate the manhattan distance between two points (4 way movement)
    pub fn heuristic_cost_manhattan(start: &Position, goal: &Position, minimum_cost: Option<u32>) -> u32 {
        minimum_cost.unwrap_or(1) * ((start.x - goal.x).abs() + (start.y - goal.y)) as u32
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
                    let next_node = map.nodes.get(next.0).unwrap();
                    let new_cost = costs.get(item.id).unwrap() + next.1;
                    let cost_now = costs.get(next.0 as &str); 

                    if cost_now.is_none() || &new_cost < cost_now.unwrap() {
                        costs.insert(next.0, new_cost);

                        let priority = new_cost + AStar::heuristic_cost_manhattan(&next_node.position, &destination_node.position, None);

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
            cost
        };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
