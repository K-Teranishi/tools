
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
pub struct Graph {
    pub v: usize,
    pub e: usize,
    pub edge: Vec<(usize, usize)>
}

//read graph data
pub fn read_graph(path: &str) -> ((usize, usize), Vec<(usize, usize)>) {
    let lines = BufReader::new(File::open(path).unwrap()).lines();
    let mut edges : Vec<(usize, usize)> = Vec::new();
    let mut info : (usize, usize) = (0, 0);

    let mut sw = true;
    for line in lines {
        let line_content = line.unwrap();
        let mut v : Vec<&str> = line_content.split_whitespace().collect();
        let i : usize = v.pop().unwrap().parse().unwrap();
        let j : usize = v.pop().unwrap().parse().unwrap();

        if sw {
            info = (i, j);
            sw = false;
        }

        else {
            edges.push((i,j));
        }
    }

    return (info, edges);
}