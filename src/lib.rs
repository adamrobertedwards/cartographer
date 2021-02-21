use std::{collections::{BinaryHeap, HashMap}, vec};
use std::cmp::{Ordering, Reverse};

#[derive(Debug)]
pub struct Node {
    neighbours: HashMap<String, u32>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            neighbours: HashMap::new(),
        }
    }
}
#[derive(Debug)]
pub struct CostPath {
    path: Vec<String>,
    cost: u32,
}

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


fn reconstruct_path(visited: &HashMap<&str, Option<&str>>, end: &str) -> Vec<String> {
    let mut path = vec![end.to_string()];
    let mut current = visited.get(end);

    while let Some(node) = *current.unwrap() {
        path.push(node.to_string());
        current = visited.get(node);
    }

    path.reverse();

    return path;
}

#[derive(Debug)]
pub struct Map {
    // HashMap of NodeIDs and Nodes
    pub nodes: HashMap<String, Node>,
}

impl Map {
    /// Create new instance of Map
    pub fn new() -> Self {
        Map {
            nodes: HashMap::new(),
        }
    }

    /// Add new node to the Map
    pub fn add_node(&mut self, id: &str) {
        self.nodes
            .entry(id.to_string())
            .or_insert(Node::new());
    }

    pub fn remove_node(&mut self, id: &str) -> Option<Node> {
        self.nodes
            .remove(&id.to_string())
    }

    pub fn connect_nodes(&mut self, from: &str, to: &str, weight: u32) -> Result<(), &str> {
        if let Some(node_from) = self.nodes.get_mut(from) {
            node_from.neighbours
                .entry(to.to_string())
                .or_insert(weight);
        }

        Ok(())
    }
}

pub struct BreadthFirstSearch <'a> {
    pub queue: BinaryHeap<QueueItem<'a>>
}

impl <'a> BreadthFirstSearch <'a> {
    pub fn new() -> Self {
        BreadthFirstSearch {
            queue: BinaryHeap::new(),
        }
    }

    pub fn calculate_path(&mut self, map: &'a Map, start: &'a str, end: &'a str) -> CostPath {
        let mut visited: HashMap<&str, Option<&str>> = HashMap::new(); 

        self.queue.clear();

        visited.insert(start, None);
        self.queue.push(QueueItem (start));

        while let Some(item) = self.queue.pop() {
            if item.0 == end {
                break;
            }

            if let Some(current) = map.nodes.get(item.0) {
                for neighbour in &current.neighbours {
                    if !visited.contains_key(&neighbour.0[..]) {
                        visited.insert(neighbour.0, Some(item.0));
                        self.queue.push(QueueItem (neighbour.0));
                    }
                }
            }
        }

        let path = reconstruct_path(&visited, end);
        let cost = path.len() as u32;

        return CostPath {
            path,
            cost,
        };
    }

    pub fn calculate_moves(&mut self, map: &'a Map, start: &'a str, moves: u32) -> Vec<String> {
        let mut visited: HashMap<&str, Option<&str>> = HashMap::new();
        let mut costs: HashMap<&str, u32> = HashMap::new();

        self.queue.clear();

        visited.insert(start, None);
        costs.insert(start, 0);

        self.queue.push(QueueItem(start));

        while let Some(item) = self.queue.pop() {
            if let Some(current) = map.nodes.get(item.0) {
                for neighbour in &current.neighbours {
                    let new_cost = costs.get(item.0).unwrap() + 1;
                    let cost_now = costs.get(neighbour.0 as &str); 

                    if cost_now.is_none() && &new_cost <= &moves {
                        costs.insert(neighbour.0, new_cost);
                        visited.insert(neighbour.0, Some(item.0));

                        self.queue.push(QueueItem(&*neighbour.0));
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

pub struct Dijkstra <'a> {
    pub queue: BinaryHeap<PriorityQueueItem<'a>>
}

impl <'a> Dijkstra <'a> {
    pub fn new() -> Self {
        Dijkstra {
            queue: BinaryHeap::new(),
        }
    }

    pub fn calculate_moves(&mut self, map: &'a Map, start: &'a str, moves: u32) -> Vec<String> {
        let mut visited: HashMap<&str, Option<&str>> = HashMap::new();
        let mut costs: HashMap<&str, u32> = HashMap::new();

        self.queue.clear();

        visited.insert(start, None);
        costs.insert(start, 0);

        self.queue.push(
            PriorityQueueItem {
                id: start,
                priority: Reverse(0),
            }
        );

        while let Some(item) = self.queue.pop() {
            if let Some(current) = map.nodes.get(item.id) {
                for neighbour in &current.neighbours {
                    let new_cost = costs.get(item.id).unwrap() + neighbour.1;
                    let cost_now = costs.get(neighbour.0 as &str); 

                    if (cost_now.is_none() || &new_cost < cost_now.unwrap()) && &new_cost <= &moves {
                        costs.insert(neighbour.0, new_cost);

                        self.queue.push(
                        PriorityQueueItem {
                                id: &*neighbour.0,
                                priority: Reverse(new_cost),
                            }
                        );

                        visited.insert(neighbour.0, Some(item.id));
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

    pub fn calculate_path (&mut self, map: &'a Map, start: &'a str, end: &'a str) -> CostPath {
        let mut visited: HashMap<&str, Option<&str>> = HashMap::new();
        let mut costs: HashMap<&str, u32> = HashMap::new();

        self.queue.clear();

        visited.insert(start, None);
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
                    let new_cost = costs.get(item.id).unwrap() + neighbour.1;
                    let cost_now = costs.get(neighbour.0 as &str); 

                    if cost_now.is_none() || &new_cost < cost_now.unwrap() {
                        costs.insert(neighbour.0, new_cost);

                        self.queue.push(
                        PriorityQueueItem {
                                id: &*neighbour.0,
                                priority: Reverse(new_cost),
                            }
                        );

                        visited.insert(neighbour.0, Some(item.id));
                    }
                }
            }
        }

        let path = reconstruct_path(&visited, end);
        let cost = *costs.get(&end).unwrap();

        return CostPath {
            path,
            cost, 
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
