use std::fs;
use petgraph::{graphmap::UnGraphMap};

const START: &str = "start";
const END: &str = "end";

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut graph = UnGraphMap::new();

    // populate graph
    for line in input.lines() {
        let nodes: Vec<&str> = line.split("-").collect();
        if nodes.len() == 2 {
            let n1 = graph.add_node(nodes[0]);
            let n2 = graph.add_node(nodes[1]);
            graph.add_edge(n1, n2, 1);
        }
    }
    let paths = explore(&graph, START, END);
    println!("{:?}", paths.len());
}

fn explore<'a>(graph: &UnGraphMap<&'a str, i32>, start: &'a str, end: &'a str) -> Vec<Vec<&'a str>> {
    let mut paths: Vec<Vec<&str>>= vec![];
    let mut path: Vec<&str> = vec![start];
    _explore(graph, &mut paths, &mut path, start, end);
    paths
}

fn _explore<'a>(graph: &UnGraphMap<&'a str, i32>, paths: &mut Vec<Vec<&'a str>>, path: &mut Vec<&'a str>, cave: &'a str, end: &'a str){
    for next_cave in graph.neighbors(cave){
        if next_cave == end {
            let mut temp = path.clone();
            temp.reverse();
            paths.push(temp);
        }
        else if !(is_small_cave(next_cave) && path.contains(&next_cave)) {
            path.push(next_cave);
            _explore(graph, paths, path, next_cave, end);
            path.pop();
        }
    }
}

fn is_small_cave(cave: &str) -> bool {
    return cave == cave.to_ascii_lowercase();
}
