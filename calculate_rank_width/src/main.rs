extern crate calculate_rank_width;
extern crate time;

use std::env;
use calculate_rank_width::graph::*;
use time::precise_time_ns;

fn main() {
    let args: Vec<String> = env::args().collect();

    for n in 1..args.len() {
        let filename : &str = &args[n];
        let (info, data) = read_graph(filename);
        let graph = Graph{v: info.1, e: info.0, edge: data};
        println!("checking {}...", filename);
        let start = precise_time_ns();
        for i in 1..graph.v {
            if graph.rank_width_bigger_k(i) {
                println!("rank-width: {}", i);
                let end = precise_time_ns();
                println!("exec time : {} ns", end - start);
                break;
            }
        }
    }

}
