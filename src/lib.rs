mod graph;
mod adjlistgraph;

use std::io::BufReader;
use std::fs::File;

use graph::Graph;


// source: filepath to twitter data eg. "path/to/twitter_rv.net"
fn make_twitter_graph<G: Graph<u32, i32>>(source: &str, &mut graph: G) {
    let reader = BufReader::new(File::open(source).unwrap());

    for readline in reader.lines() {
        let line = readline.ok().expect("read error");
        let elts: Vec<&str> = line[..].split_whitespace().collect();
        let src: u32 = elts[0].parse().ok().expect("malformed src");
        let tgt: u32 = elts[1].parse().ok().expect("malformed tgt");
        let a = graph.add_node(src);
        let b = graph.add_node(tgt);
        graph.add_edge(a, b, 1);
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
