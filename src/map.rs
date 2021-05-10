use std::collections::HashMap;

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Node {
    pub position: Position,
    pub neighbours: HashMap<String, u32>,
}

impl Node {
    pub fn new(position: (f32, f32)) -> Node {
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
    pub fn add_node(&mut self, id: &str, position: (f32, f32)) {
        self.nodes
            .entry(id.to_string())
            .or_insert(Node::new(
                position
            ));
    }

    /// Remove node from the Map
    pub fn remove_node(&mut self, id: &str) -> Result<(), &str> {
        // Iterate through map nodes
        // Find where current node neighbours contain this node
        // Remove reference to this node from the current node neighbours
        for (_, node) in self.nodes.iter_mut() {
            if let Some(_) = node.neighbours.get(id) {
                node.neighbours.remove(id);
            }
        }

        self.nodes
            .remove(&id.to_string());
        
        Ok(())
    }

    pub fn connect_nodes(&mut self, from: &str, to: &str, weight: u32) -> Result<(), &str> {
        let destination_exists = self.nodes.contains_key(to);

        if let (Some(node_from), true) = (self.nodes.get_mut(from), destination_exists)  {
            node_from.neighbours
                .entry(to.to_string())
                .or_insert(weight);
        }

        Ok(())
    }
}
