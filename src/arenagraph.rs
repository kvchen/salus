use std::cell::UnsafeCell;
use arena::TypedArena;

/**
 * This graph implementation utilizes a memory arena (only in unstable Rust),
 * which allocates nodes and edges in what is essentially a large vector.
 * Everything is deallocated all at once when the arena is dropped.
 */

// ============================================================================
// EDGE IMPLEMENTATION
// ============================================================================

// 'a here indicates the lifetime of our Edge and its associated data.
#[derive(Debug)]
pub struct Edge<'a, TEdge: 'a, TNode: 'a> {
    // Associated data for this edge
    data: TEdge,

    // Start and end nodes for this edge
    start: &'a Node<'a, TNode, TEdge>,
    end: &'a Node<'a, TNode, TEdge>,
}

impl<'a, TEdge, TNode> Edge<'a, TEdge, TNode> {
    fn new<'b>(
        arena: &'b TypedArena<Edge<'b, TEdge, TNode>>,
        data: TEdge,
        start: &'b Node<'b, TNode, TEdge>,
        end: &'b Node<'b, TNode, TEdge>,
    ) -> &'b Edge<'b, TEdge, TNode> {
        arena.alloc(Edge {
            data: data,
            start: start,
            end: end,
        })
    }
}

// ============================================================================
// NODE IMPLEMENTATION
// ============================================================================

#[derive(Debug)]
pub struct Node<'a, TNode: 'a, TEdge: 'a> {
    data: TNode,

    // This is a directed graph, so each node has predecessors and successors
    predecessors: UnsafeCell<Vec<&'a Edge<'a, TEdge, TNode>>>,
    successors: UnsafeCell<Vec<&'a Edge<'a, TEdge, TNode>>>,
}


impl<'a, TNode: 'a, TEdge: 'a> Node<'a, TNode, TEdge> {
    fn new<'b>(
        arena: &'b TypedArena<Node<'b, TNode, TEdge>>,
        data: TNode,
    ) -> &'b Node<'b, TNode, TEdge> {
        arena.alloc(Node {
            data: data,
            predecessors: UnsafeCell::new(Vec::new()),
            successors: UnsafeCell::new(Vec::new()),
        })
    }
}

// ============================================================================
// GRAPH IMPLEMENTATION
// ============================================================================

pub struct ArenaGraph<'a, TNode: 'a, TEdge: 'a> {
    nodes: TypedArena<Node<'a, TNode, TEdge>>,
    edges: TypedArena<Edge<'a, TEdge, TNode>>,
}

impl<'a, TNode, TEdge> ArenaGraph<'a, TNode, TEdge> {
    pub fn new<'b>() -> ArenaGraph<'b, TNode, TEdge> {
        ArenaGraph {
            nodes: TypedArena::new(),
            edges: TypedArena::new(),
        }
    }

    pub fn add_node(&'a self, data: TNode) -> &'a Node<'a, TNode, TEdge> {
        Node::new(&self.nodes, data)
    }

    // Removes node and all incident edges
    fn remove_node(
        &mut self,
        node: &'a Node<'a, TNode, TEdge>,
    ) -> Option<TNode> {
        unimplemented!();
    }

    pub fn add_edge(
        &'a self,
        start_node: &'a Node<'a, TNode, TEdge>,
        end_node: &'a Node<'a, TNode, TEdge>,
        data: TEdge
    ) -> &'a Edge<'a, TEdge, TNode> {
        let edge = Edge::new(
            &self.edges,
            data,
            start_node,
            end_node,
        );

        unsafe {
            (*start_node.successors.get()).push(edge);
            (*end_node.predecessors.get()).push(edge);
        }

        edge
    }

    // use drop() instead
    fn clear(&mut self) {
        unimplemented!()
    }
}
