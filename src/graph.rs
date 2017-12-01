pub trait Graph<TNode, TEdge> {
    type TIndex;
    // MUTATORS

    // Adds a single node to the graph and returns its corresponding index.
    fn add_node(&mut self, data: TNode) -> Self::TIndex;

    // Remove node n from the graph and returns its associated value.
    fn remove_node(&mut self, node: Self::TIndex) -> Option<TNode>;

    // Adds a single edge to the graph and returns its corresponding index.
    fn add_edge(&mut self, node_a: Self::TIndex, node_b: Self::TIndex, data: TEdge) -> Self::TIndex;

    // Removes an edge between nodes a and b and returns its associated value.
    fn remove_edge(&mut self, edge: Self::TIndex) -> Option<TEdge>;

    // Removes an edge between nodes a and b and returns its associated value.
    fn remove_edge_between(&mut self, node_a: Self::TIndex, node_b: Self::TIndex) -> Option<TEdge>;

    // Removes all nodes and edges from the graph.
    // fn clear(&mut self);

    // ACCESSORS

    // Returns the total number of nodes in the graph.
    fn order(&self) -> usize;

    // Returns an iterator over all the graph nodes.
    fn nodes(&self) -> Iterator<Item = &Self::TIndex>;

    // Returns an iterator over all the graph edges.
    fn edges(&self) -> Iterator<Item = &Self::TIndex>;

    // Returns true if graph contains the node, else false
    fn has_node(&self, node: Self::TIndex) -> bool;

    // Returns the data contained in a node.
    fn get_node_data(&self, node: Self::TIndex) -> Option<&TNode>;

    // Returns true if graph contains the edge, else false
    fn has_edge(&self, edge: Self::TIndex) -> bool;

    // Returns true if graph contains an edge between two nodes, else false
    fn has_edge_between(&self, node_a: Self::TIndex, node_b: Self::TIndex) -> bool;

    // Returns the edge between two nodes
    fn get_edge(&self, node_a: Self::TIndex, node_b: Self::TIndex) -> Option<&Self::TIndex>;

    // Returns an iterator over all edges incident on a node.
    // fn get_edges_of(&self, node: Self::TIndex) -> Iterator<Item = &Self::TIndex>;

    // Returns the source node of an edge.
    // fn get_edge_source(&self, edge: Self::TIndex) ->

    // Returns the associated metadata with an edge between two nodes a and b.
    fn get_edge_data(&self, edge: Self::TIndex) -> Option<&TEdge>;
}
