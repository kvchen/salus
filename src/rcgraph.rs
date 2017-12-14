use std::rc::Rc;
use std::cell::RefCell;

/*******************************************************************/
/*******************************************************************/

#[derive(Debug)]
struct Node<TNode, TEdge> {
    // Associated data for this node
    data: TNode,

    // Edges leading to predecessors and successors of this node
    pred: Vec<Rc<RefCell<Edge<TEdge, TNode>>>>,
    succ: Vec<Rc<RefCell<Edge<TEdge, TNode>>>>,
}

// impl<TNode, TEdge> Drop for Node<TNode, TEdge> {
//     fn drop(&mut self) {
//         for pred in self.pred.drain(..) {
//             drop(pred);
//         }
//         for succ in self.succ.drain(..) {
//             drop(succ);
//         }
//     }
// }

impl<TNode, TEdge> Node<TNode, TEdge> {
    fn new(data: TNode) -> Node<TNode, TEdge> {
        Node {
            data: data,
            pred: Vec::new(),
            succ: Vec::new(),
        }
    }
}

/*******************************************************************/
/*******************************************************************/

#[derive(Debug)]
struct Edge<TEdge, TNode> {
    // Associated data for this edge
    data: TEdge,

    // Start and end nodes for this edge
    start: Rc<RefCell<Node<TNode, TEdge>>>,
    end: Rc<RefCell<Node<TNode, TEdge>>>,
}

// impl<TEdge, TNode> Drop for Edge<TEdge, TNode> {
//     fn drop(&mut self) {
//         drop(self.start);
//         drop(self.end);
//     }
// }

impl<TEdge, TNode> Edge<TEdge, TNode> {
    fn new(data: TEdge, start: Rc<RefCell<Node<TNode, TEdge>>>, end: Rc<RefCell<Node<TNode, TEdge>>>)
        -> Edge<TEdge, TNode>
    {
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
pub struct RcGraph<TNode, TEdge> {
    nodes: Vec<Rc<RefCell<Node<TNode, TEdge>>>>,
    edges: Vec<Rc<RefCell<Edge<TEdge, TNode>>>>,
}

// impl<TNode, TEdge> Drop for RcGraph<TNode, TEdge> {
//     fn drop(&mut self) {
//         for edge in self.edges.drain(..) {
//             drop(edge);
//         }
//         for node in self.nodes.drain(..) {
//             drop(node);
//         }
//     }
// }

impl<TNode, TEdge> RcGraph<TNode, TEdge> {
    pub fn new() -> RcGraph<TNode, TEdge> {
        RcGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn add_node(&mut self, data: TNode) -> Rc<RefCell<Node<TNode, TEdge>>> {
        let node_idx = Rc::new(RefCell::new(Node::new(data)));
        self.nodes.push(node_idx.clone());
        node_idx
    }

    // Removes node and all incident edges
    fn remove_node(&mut self, node: Rc<RefCell<Node<TNode, TEdge>>>) -> Option<TNode> {
        unimplemented!();
    }

    // Assumes given nodes exist...which is basically guaranteed
    fn add_edge(&mut self, start_node: Rc<RefCell<Node<TNode, TEdge>>>, end_node: Rc<RefCell<Node<TNode, TEdge>>>, data: TEdge)
        -> Rc<RefCell<Edge<TEdge, TNode>>>
    {
        let edge_idx = Rc::new(RefCell::new(Edge::new(data, start_node.clone(), end_node.clone())));
        self.edges.push(edge_idx.clone());
        start_node.borrow_mut().succ.push(edge_idx.clone());
        end_node.borrow_mut().pred.push(edge_idx.clone());

        edge_idx
    }

    fn remove_edge(&mut self, edge: Rc<RefCell<Edge<TEdge, TNode>>>) -> Option<TEdge> {
        unimplemented!();
    }

    fn remove_edge_between(&mut self, node_a: Rc<RefCell<Node<TNode, TEdge>>>, node_b: Rc<RefCell<Node<TNode, TEdge>>>) -> Option<TEdge> {
        unimplemented!();
    }

    fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }

    fn order(&self) -> usize {
        self.nodes.len()
    }

    fn size(&self) -> usize {
        self.edges.len()
    }

    fn get_node_data(&self, node: Rc<RefCell<Node<TNode, TEdge>>>) -> Option<&TNode> {
        unimplemented!();
    }

    fn get_edge_data(&self, edge: Rc<RefCell<Edge<TEdge, TNode>>>) -> Option<&TEdge> {
        unimplemented!();
    }

    fn nodes<'a>(&'a self) -> Box<Iterator<Item = &'a Rc<RefCell<Node<TNode, TEdge>>>> + 'a> {
        Box::new(self.nodes.iter())
    }

    fn edges<'a>(&'a self) -> Box<Iterator<Item = &'a Rc<RefCell<Edge<TEdge, TNode>>>> + 'a> {
        Box::new(self.edges.iter())
    }
}
