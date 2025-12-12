use ahash::HashMap;
use itertools::Itertools;
use std::{fmt::Display, hash::Hash};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct NodeId(usize);

#[derive(Default)]
pub struct OrientedGraph<Node> {
    nodes: Vec<Node>,
    edges: Vec<Vec<NodeId>>,
    lookup: HashMap<Node, NodeId>,
}

impl<Node> OrientedGraph<Node> {
    pub fn add_node(&mut self, node: Node) -> NodeId
    where
        Node: Clone + Eq + Hash,
    {
        self.nodes.push(node.clone());
        let id = NodeId(self.nodes.len() - 1);
        self.edges.push(Vec::default());
        self.lookup.insert(node, id);
        id
    }

    // NodeId must be valid
    pub fn add_edge(&mut self, from: &NodeId, to: NodeId) {
        self.edges[from.0].push(to);
    }

    pub fn get_node_id(&self, node: &Node) -> Option<&NodeId>
    where
        Node: Eq + Hash,
    {
        self.lookup.get(node)
    }

    pub fn get_or_insert_node(&mut self, node: Node) -> NodeId
    where
        Node: Clone + Eq + Hash,
    {
        match self.get_node_id(&node) {
            Some(id) => *id,
            None => self.add_node(node),
        }
    }

    pub fn get_edges(&self, from: &NodeId) -> &Vec<NodeId> {
        &self.edges[from.0]
    }

    pub fn get_node(&self, id: &NodeId) -> &Node {
        &self.nodes[id.0]
    }
}

impl<Node> Display for OrientedGraph<Node>
where
    Node: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "OrientedGraph {{")?;
        for (i, node) in self.nodes.iter().enumerate() {
            let edges = &self.edges[i];
            write!(f, "  {} ->", node)?;
            if edges.is_empty() {
                writeln!(f, " empty")?;
            } else {
                writeln!(
                    f,
                    " [{}]",
                    edges.iter().map(|id| self.get_node(id)).join(", ")
                )?;
            }
        }
        write!(f, "}}")
    }
}
