#![feature(rustc_private)]

extern crate arena;

mod graph;
mod adjlistgraph;
mod arenagraph;
mod rcgraph;

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

use graph::Graph;


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


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
