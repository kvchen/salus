use graph::Graph;
use graph::Edge;
use std::collections::{HashMap};
use std::collections::hash_map::Keys;

pub struct AdjListGraph<TNode, TEdge> {
    cur_node_idx: i64,
    cur_edge_idx: i64,

    nodes: HashMap<i64, (TNode, Vec<i64>)>,
    edges: HashMap<i64, Edge<TEdge, i64>>,
}

impl<TNode, TEdge> AdjListGraph<TNode, TEdge> {
    pub fn new() -> AdjListGraph<TNode, TEdge> {
        AdjListGraph {
            cur_node_idx: 0i64,
            cur_edge_idx: 0i64,

            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }
}

impl<TNode, TEdge> Graph<TNode, TEdge> for AdjListGraph<TNode, TEdge> {
    type TIndex = i64;

    fn add_node(&mut self, data: TNode) -> Self::TIndex {
        let node_idx = self.cur_node_idx;
        self.cur_node_idx += 1;
        self.nodes.insert(node_idx, (data, Vec::new()));
        node_idx
    }

    fn remove_node(&mut self, node: i64) -> Option<TNode> {
        unimplemented!();
    }

    fn add_edge(&mut self, start_node: i64, end_node: i64, data: TEdge) -> Self::TIndex {
        // currently assumes both start_node and end_node are valid nodes
        // returned by add_node
        let edge_idx = self.cur_edge_idx;
        self.cur_edge_idx += 1;

        let new_edge = Edge {
            data: data,
            start: start_node,
            end: end_node,
        };

        self.edges.insert(edge_idx, new_edge);
        self.nodes.get_mut(&start_node).unwrap().1.push(edge_idx);
        edge_idx
    }

    fn remove_edge(&mut self, edge: i64) -> Option<TEdge> {
        unimplemented!();
    }

    fn remove_edge_between(&mut self, node_a: i64, node_b: i64) -> Option<TEdge> {
        unimplemented!();
    }

    fn order(&self) -> usize {
        self.nodes.len()
    }

    fn nodes(&self) -> Iterator<Item = &Self::TIndex> {
        self.nodes.keys()
    }

    fn edges(&self) -> Iterator<Item = &Self::TIndex> {
        self.edges.keys()
    }

    fn has_node(&self, node: i64) -> bool {
        self.nodes.contains_key(&node)
    }

    fn get_node_data(&self, node: i64) -> Option<&(TNode, Vec<i64>)> {
        self.nodes.get(&node)
    }

    fn has_edge(&self, edge: i64) -> bool {
        self.edges.contains_key(&edge)
    }

    fn has_edge_between(&self, node_a: i64, node_b: i64) -> bool {
        unimplemented!();
    }

    fn get_edge(&self, node_a: i64, node_b: i64) -> Option<&i64> {
        unimplemented!();
    }

    fn get_edge_data(&self, edge_idx: i64) -> Option<&Edge<TEdge, i64>> {
        self.edges.get(&edge_idx)
    }
}
