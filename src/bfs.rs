use std::collections::{BinaryHeap, HashMap};
use super::{QueueItem, Pathing, CostPath, Map};
/// BreadthFirstSearch
///
/// Struct implementing a simple BFS algorithm
pub struct BreadthFirstSearch <'a> {
    pub queue: BinaryHeap<QueueItem<'a>>,
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
                    if !self.visited.contains_key(&neighbour.0[..]) {
                        self.visited.insert(neighbour.0, Some(item.0));
                        self.queue.push(QueueItem (neighbour.0));
                    }
                }
            }
        }

        let path = self.reconstruct_path(&self.visited, end);
        let cost = path.len() as u32;

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

        self.queue.push(QueueItem(start));

        while let Some(item) = self.queue.pop() {
            if let Some(current) = map.nodes.get(item.0) {
                for neighbour in &current.neighbours {
                    let new_cost = costs.get(item.0).unwrap() + 1;
                    let cost_now = costs.get(neighbour.0 as &str); 

                    if cost_now.is_none() && &new_cost <= &moves {
                        costs.insert(neighbour.0, new_cost);
                        self.visited.insert(neighbour.0, Some(item.0));

                        self.queue.push(QueueItem(&*neighbour.0));
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
