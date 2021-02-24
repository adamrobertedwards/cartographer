use std::collections::HashMap;

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Node {
    pub position: Position,
    pub neighbours: HashMap<String, u32>,
}

impl Node {
    pub fn new(position: (i32, i32)) -> Node {
        Node {
            neighbours: HashMap::new(),
            position: Position {
                x: position.0,
                y: position.1
            }
        }
    }
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
    pub fn add_node(&mut self, id: &str, position: (i32, i32)) {
        self.nodes
            .entry(id.to_string())
            .or_insert(Node::new(
                position
            ));
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
