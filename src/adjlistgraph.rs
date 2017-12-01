use graph::Graph;
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Keys;

pub struct AdjListGraph<TNode, TEdge> {
    nodes: HashMap<i64, TNode>,
    cur_node_idx: i64,
    edges_list: HashMap<i64, Vec<i64>>, 
    // above, key is for a node
    // below, key is for an edge
    edges_data: HashMap<i64, TEdge>,
    cur_edge_idx: i64,
}

impl<TNode, TEdge> AdjListGraph<TNode, TEdge> {
    pub fn new() -> AdjListGraph<TNode, TEdge> {
        AdjListGraph {nodes: HashMap::new(),
                      cur_node_idx: 0i64,
                      edges_list: HashMap::new(),
                      edges_data: HashMap::new(),
                      cur_edge_idx: 0i64}
    }
}

impl<TNode, TEdge> Graph<TNode, TEdge> for AdjListGraph<TNode, TEdge> {
    type TIndex = i64;

    fn add_node(&mut self, data: TNode) -> i64 {
        let n = self.cur_node_idx;
        self.cur_node_idx += 1;
        self.nodes.insert(n, data);
        self.edges_list.insert(n, Vec::new());
        n
    }

    fn remove_node(&mut self, data: TNode) -> Option<TNode> {
        unimplemented!();
    }

    fn add_edge(&mut self, node_a: i64, node_b: i64, data: TEdge) -> i64 {
        // currently assumes both node_a and node_b are valid nodes
        // returned by add_node
        let n = self.cur_edge_idx;
        self.cur_edge_idx += 1;
        self.edges_list.get_mut(&node_a).unwrap().push(node_b);
        self.edges_data.insert(n, data);
        n
    }

    fn remove_edge(&mut self, node_a: i64, node_b: i64) -> Option<TEdge> {
        unimplemented!();
    }

    fn order(&self) -> i64 {
        unimplemented!();
    }

    fn nodes(&self) -> Keys<i64, TNode> {
        self.nodes.keys()
    }

    fn get_node_data(&self, node: i64) -> Option<&TNode> {
        self.nodes.get(&node)
    }

    fn get_edge_data(&self, edge: i64) -> Option<&TEdge> {
        self.edges_data.get(&edge)
    }
}
