// use std::collections::{HashMap, HashSet};

pub trait Graph<TNode, TEdge, TIndex> {
    // MUTATORS

    // Adds a single node to the graph and returns its corresponding index.
    fn add_node(&mut self, data: TNode) -> TIndex;

    // Remove node n from the graph and returns its associated value.
    fn remove_node(&mut self, data: TNode) -> Option<TNode>;

    // Adds a single edge to the graph and returns its corresponding index.
    fn add_edge(&mut self, node_a: TIndex, node_b: TIndex) -> TIndex;

    // Removes an edge between nodes a and b and returns its associated value.
    fn remove_edge(&mut self, node_a: TIndex, node_b: TIndex) -> Option<TEdge>;

    // ACCESSORS

    // Returns the total number of nodes in the graph.
    fn order(&self) -> i64;

    // Returns an iterator over all the graph nodes.
    fn nodes(&self) -> Iterator<Item = &TIndex>;

    // Returns
    fn get_node_data(&self, node: TIndex) -> Option<TNode>;

    // Returns the associated metadata with an edge between two nodes a and b.
    fn get_edge_data(&self, edge: TIndex) -> Option<TEdge>;
}
