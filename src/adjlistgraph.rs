use graph::Graph;
use std::collections::HashMap;

/*******************************************************************/
/*******************************************************************/

#[derive(Debug)]
struct Node<TNode, TIndex> {
    // Associated data for this node
    data: TNode,

    // Edges leading to predecessors and successors of this node
    pred: Vec<TIndex>,
    succ: Vec<TIndex>,
}

impl<TNode, TIndex: PartialEq> Node<TNode, TIndex> {
    fn new(data: TNode) -> Node<TNode, TIndex> {
        Node {
            data: data,
            pred: Vec::new(),
            succ: Vec::new(),
        }
    }

    fn remove_succ(&mut self, edge: TIndex) {
        for i in 0..self.succ.len() {
            if self.succ[i] == edge {
                self.succ.swap_remove(i);
                break;
            }
        }
    }

    fn remove_pred(&mut self, edge: TIndex) {
        for i in 0..self.pred.len() {
            if self.pred[i] == edge {
                self.pred.swap_remove(i);
                break;
            }
        }
    }
}

/*******************************************************************/
/*******************************************************************/

#[derive(Debug)]
struct Edge<TEdge, TIndex> {
    // Associated data for this edge
    data: TEdge,

    // Start and end nodes for this edge
    start: TIndex,
    end: TIndex,
}

impl<TEdge, TIndex> Edge<TEdge, TIndex> {
    fn new(data: TEdge, start: TIndex, end: TIndex) -> Edge<TEdge, TIndex> {
        Edge {
            data: data,
            start: start,
            end: end,
        }
    }
}

/*******************************************************************/
/*******************************************************************/

#[derive(Debug)]
pub struct AdjListGraph<TNode, TEdge> {
    cur_node_idx: i64,
    cur_edge_idx: i64,

    nodes: HashMap<i64, Node<TNode, i64>>,
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
        self.nodes.insert(node_idx, Node::new(data));
        node_idx
    }

    // Removes node and all incident edges
    fn remove_node(&mut self, node: Self::TIndex) -> Option<TNode> {
        if let Some(n) = self.nodes.remove(&node) {
            for edge_idx in n.succ {
                if let Some(e) = self.edges.remove(&edge_idx) {
                    self.nodes.get_mut(&e.end).unwrap().remove_pred(edge_idx);
                }
            }
            for edge_idx in n.pred {
                if let Some(e) = self.edges.remove(&edge_idx) {
                    self.nodes.get_mut(&e.start).unwrap().remove_succ(edge_idx);
                }
            }
            Some(n.data)
        } else {
            None
        }
    }

    fn add_edge(&mut self, start_node: Self::TIndex, end_node: Self::TIndex, data: TEdge) -> Option<Self::TIndex> {
        if !self.nodes.contains_key(&start_node) || !self.nodes.contains_key(&end_node) {
            return None;
        }
        let edge_idx = self.cur_edge_idx;
        self.cur_edge_idx += 1;

        self.edges.insert(edge_idx, Edge::new(data, start_node, end_node));
        self.nodes.get_mut(&start_node).unwrap().succ.push(edge_idx);
        self.nodes.get_mut(&end_node).unwrap().pred.push(edge_idx);

        Some(edge_idx)
    }

    fn remove_edge(&mut self, edge: Self::TIndex) -> Option<TEdge> {
        if let Some(e) = self.edges.remove(&edge) {
            self.nodes.get_mut(&e.start).unwrap().remove_succ(edge);
            self.nodes.get_mut(&e.end).unwrap().remove_pred(edge);
            Some(e.data)
        } else {
            None
        }
    }

    fn remove_edge_between(&mut self, node_a: Self::TIndex, node_b: Self::TIndex) -> Option<TEdge> {
        let mut edge_opt: Option<Self::TIndex> = None;
        if let Some(start) = self.nodes.get(&node_a) {
            for edge_idx in start.succ.iter() {
                let edge = self.edges.get(edge_idx).unwrap();
                if edge.end == node_b {
                    edge_opt = Some(*edge_idx);
                }
            }
        }

        if let Some(e) = edge_opt {
            self.nodes.get_mut(&node_a).unwrap().remove_succ(e);
            self.nodes.get_mut(&node_b).unwrap().remove_pred(e);
            Some(self.edges.remove(&e).unwrap().data)
        } else {
            None
        }
    }

    fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }

    fn order(&self) -> usize {
        self.nodes.len()
    }

    fn has_node(&self, node: Self::TIndex) -> bool {
        self.nodes.contains_key(&node)
    }

    fn get_node_data(&self, node: Self::TIndex) -> Option<&TNode> {
        if let Some(n) = self.nodes.get(&node) {
            Some(&n.data)
        } else {
            None
        }
    }

    fn has_edge(&self, edge: Self::TIndex) -> bool {
        self.edges.contains_key(&edge)
    }

    fn has_edge_between(&self, node_a: Self::TIndex, node_b: Self::TIndex) -> bool {
        if let Some(start) = self.nodes.get(&node_a) {
            for edge_idx in start.succ.iter() {
                let edge = self.edges.get(&edge_idx).unwrap();
                if edge.end == node_b {
                    return true;
                }
            }
        }
        false
    }

    fn get_edge(&self, node_a: Self::TIndex, node_b: Self::TIndex) -> Option<&Self::TIndex> {
        if let Some(start) = self.nodes.get(&node_a) {
            for edge_idx in start.succ.iter() {
                let edge = self.edges.get(&edge_idx).unwrap();
                if edge.end == node_b {
                    return Some(edge_idx);
                }
            }
        }
        None
    }

    fn get_edge_start(&self, edge: Self::TIndex) -> Option<&Self::TIndex> {
        if let Some(e) = self.edges.get(&edge) {
            Some(&e.start)
        } else {
            None
        }
    }

    fn get_edge_end(&self, edge: Self::TIndex) -> Option<&Self::TIndex> {
        if let Some(e) = self.edges.get(&edge) {
            Some(&e.end)
        } else {
            None
        }
    }

    fn get_edge_data(&self, edge: Self::TIndex) -> Option<&TEdge> {
        if let Some(e) = self.edges.get(&edge) {
            Some(&e.data)
        } else {
            None
        }
    }

    fn nodes<'a>(&'a self) -> Box<Iterator<Item = &'a Self::TIndex> + 'a> {
        Box::new(self.nodes.keys())
    }

    fn edges<'a>(&'a self) -> Box<Iterator<Item = &'a Self::TIndex> + 'a> {
        Box::new(self.edges.keys())
    }

    fn get_edges_of<'a>(&'a self, node: Self::TIndex) ->
                            Box<Iterator<Item = &'a Self::TIndex> + 'a> {
        if let Some(n) = self.nodes.get(&node) {
            Box::new(n.pred.iter().chain(n.succ.iter()))
        } else {
            Box::new(::std::iter::empty())
        }
    }

    fn get_pred_edges_of<'a>(&'a self, node: Self::TIndex) ->
                            Box<Iterator<Item = &'a Self::TIndex> + 'a> {
        if let Some(n) = self.nodes.get(&node) {
            Box::new(n.pred.iter())
        } else {
            Box::new(::std::iter::empty())
        }
    }

    fn get_succ_edges_of<'a>(&'a self, node: Self::TIndex) ->
                            Box<Iterator<Item = &'a Self::TIndex> + 'a> {
        if let Some(n) = self.nodes.get(&node) {
            Box::new(n.succ.iter())
        } else {
            Box::new(::std::iter::empty())
        }
    }
}
