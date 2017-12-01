pub trait Graph<TNode, TEdge> {
    type TIndex;
    // MUTATORS

    // Adds a single node to the graph and returns its corresponding index.
    fn add_node(&mut self, data: TNode) -> Self::TIndex;

    // Remove node n from the graph and returns its associated value.
    fn remove_node(&mut self, data: TNode) -> Option<TNode>;

    // Adds a single edge to the graph and returns its corresponding index.
    fn add_edge(&mut self, node_a: Self::TIndex, node_b: Self::TIndex, data: TEdge) -> Self::TIndex;

    // Removes an edge between nodes a and b and returns its associated value.
    fn remove_edge(&mut self, node_a: Self::TIndex, node_b: Self::TIndex) -> Option<TEdge>;

    // ACCESSORS

    // Returns the total number of nodes in the graph.
    fn order(&self) -> i64;

    // Returns an iterator over all the graph nodes.
    fn nodes(&self) -> impl Iterator<Item = &Self::TIndex>;

    // Returns the data contained in a node.
    fn get_node_data(&self, node: Self::TIndex) -> Option<&TNode>;

    // Returns the associated metadata with an edge between two nodes a and b.
    fn get_edge_data(&self, edge: Self::TIndex) -> Option<&TEdge>;
}
