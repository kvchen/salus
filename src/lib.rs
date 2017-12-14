#![feature(rustc_private)]

extern crate arena;

mod graph;
mod adjlistgraph;
mod arenagraph;
mod rcgraph;

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use graph::Graph;
use adjlistgraph::AdjListGraph;
use rcgraph::RcGraph;


// source: filepath to twitter data eg. "path/to/twitter_rv.net"
fn make_twitter_graph<G: Graph<u32, i32>>(source: &str, graph: &mut G) {
    let reader = BufReader::new(File::open(source).unwrap());
    let mut nodes: HashMap<u32, G::TIndex> = HashMap::new();

    for readline in reader.lines() {
        let line = readline.ok().expect("read error");
        let elts: Vec<&str> = line[..].split_whitespace().collect();
        let src: u32 = elts[0].parse().ok().expect("malformed src");
        let tgt: u32 = elts[1].parse().ok().expect("malformed tgt");
        if !nodes.contains_key(&src) {
            nodes.insert(src, graph.add_node(src));
        }
        if !nodes.contains_key(&tgt) {
            nodes.insert(tgt, graph.add_node(tgt));
        }
        graph.add_edge(*nodes.get(&src).unwrap(), *nodes.get(&tgt).unwrap(), 1);
    }
}

fn make_twitter_rcgraph(source: &str, graph: &mut RcGraph<u32, i32>) {
    let reader = BufReader::new(File::open(source).unwrap());
    let mut nodes: HashMap<u32, Rc<RefCell<rcgraph::Node<u32, i32>>>> = HashMap::new();

    for readline in reader.lines() {
        let line = readline.ok().expect("read error");
        let elts: Vec<&str> = line[..].split_whitespace().collect();
        let src: u32 = elts[0].parse().ok().expect("malformed src");
        let tgt: u32 = elts[1].parse().ok().expect("malformed tgt");
        if !nodes.contains_key(&src) {
            nodes.insert(src, graph.add_node(src));
        }
        if !nodes.contains_key(&tgt) {
            nodes.insert(tgt, graph.add_node(tgt));
        }
        graph.add_edge(nodes.get(&src).unwrap().clone(), nodes.get(&tgt).unwrap().clone(), 1);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rcgraph() {
        let mut g = RcGraph::new();
        make_twitter_rcgraph("twitter_500.net", &mut g);
        assert!(g.order() != 0);
        assert!(g.size() != 0);
    }

    #[test]
    fn idxgraph() {
        let mut g = AdjListGraph::new();
        make_twitter_graph("twitter_500.net", &mut g);
        assert!(g.order() != 0);
        assert!(g.size() != 0);
    }
}
