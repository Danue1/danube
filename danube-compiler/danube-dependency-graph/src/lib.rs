#![warn(clippy::all)]

use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct DependencyGraph<Node> {
    nodes: HashSet<Node>,
    outgoing: HashMap<Node, HashSet<Node>>,
    incoming: HashMap<Node, HashSet<Node>>,
}

impl<Node> DependencyGraph<Node>
where
    Node: Default,
{
    pub fn new() -> Self {
        DependencyGraph::default()
    }
}

impl<Node> DependencyGraph<Node>
where
    Node: Eq + std::hash::Hash,
{
    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node);
    }

    pub fn add_edge(&mut self, from: Node, to: Node)
    where
        Node: Clone,
    {
        self.outgoing
            .entry(from.clone())
            .or_default()
            .insert(to.clone());
        self.incoming.entry(to).or_default().insert(from);
    }

    pub fn outgoing(&self, node: &Node) -> Option<&HashSet<Node>> {
        self.outgoing.get(node)
    }

    pub fn incoming(&self, node: &Node) -> Option<&HashSet<Node>> {
        self.incoming.get(node)
    }
}

impl<Node> DependencyGraph<Node> {
    pub fn nodes(&self) -> &HashSet<Node> {
        &self.nodes
    }
}
