#![feature(alloc_system)]
#![feature(rustc_private)]

extern crate alloc_system;
extern crate arena;

mod graph;
mod adjlistgraph;
mod arenagraph;
mod rcgraph;

use std::env;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use graph::Graph;


// source: filepath to twitter data eg. "path/to/twitter_rv.net"
fn create_indexgraph<G: Graph<u32, i32>>(source: &str, graph: &mut G) {
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

fn create_rcgraph(source: &str, graph: &mut rcgraph::RcGraph<u32, i32>) {
    let reader = BufReader::new(File::open(source).unwrap());
    let mut nodes: HashMap<u32, Rc<RefCell<rcgraph::Node<u32, i32>>>>
        = HashMap::new();

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

fn create_arenagraph<'a>(
    source: &str,
    graph: &'a mut arenagraph::ArenaGraph<'a, u32, u32>,
) {
	let reader = BufReader::new(File::open(source).unwrap());
    let mut nodes: HashMap<u32, &arenagraph::Node<'a, u32, u32>>
        = HashMap::new();

    for readline in reader.lines() {
        let line = readline.ok().expect("read error");
        let elts: Vec<&str> = line[..].split_whitespace().collect();
        let source_id: u32 = elts[0].parse().ok().expect("malformed src");
        let target_id: u32 = elts[1].parse().ok().expect("malformed tgt");

        if !nodes.contains_key(&source_id) {
            nodes.insert(source_id, graph.add_node(source_id));
        }
        if !nodes.contains_key(&target_id) {
            nodes.insert(target_id, graph.add_node(target_id));
        }

        let source_node = nodes.get(&source_id).unwrap();
        let end_node = nodes.get(&target_id).unwrap();

        graph.add_edge(source_node, end_node, 1);
    }
}


fn main() {
    if let Some(graph_path) = env::args().nth(1) {
        // let mut g = rcgraph::RcGraph::new();
        // make_twitter_rcgraph(&graph_path, &mut g);

        let mut g = arenagraph::ArenaGraph::new();
        create_arenagraph(&graph_path, &mut g);

        // let mut g = adjlistgraph::AdjListGraph::new();
        // make_twitter_graph(&graph_path, &mut g);
    }
}
