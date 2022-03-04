#![warn(clippy::all)]

use danube_ast::NodeId;
use danube_dependency_graph::DependencyGraph;
use std::collections::HashSet;

#[derive(Default)]
pub struct NameResolution {
    dependency_graph: DependencyGraph<NodeId>,
}

impl NameResolution {
    pub fn new() -> Self {
        NameResolution::default()
    }

    #[inline]
    pub fn add_node(&mut self, node: NodeId) {
        self.dependency_graph.add_node(node);
    }

    #[inline]
    pub fn add_edge(&mut self, from: NodeId, to: NodeId) {
        self.dependency_graph.add_edge(from, to);
    }

    #[inline]
    pub fn outgoing(&self, node: &NodeId) -> Option<&HashSet<NodeId>> {
        self.dependency_graph.outgoing(node)
    }

    #[inline]
    pub fn incoming(&self, node: &NodeId) -> Option<&HashSet<NodeId>> {
        self.dependency_graph.incoming(node)
    }

    #[inline]
    pub fn nodes(&self) -> &HashSet<NodeId> {
        self.dependency_graph.nodes()
    }
}
