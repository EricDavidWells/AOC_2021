use std::time::{Duration, Instant};
use std::fs;
use std::collections::{HashMap};
use std::ops::Index;

use regex::Regex;
use petgraph::{Graph, Undirected};
use petgraph::prelude::*;
use petgraph::algo::{astar, dijkstra};

fn parse_input(filename: &str) -> (GraphMap<(usize, usize), u64, Directed>, Vec<(usize, usize)>)
{
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let re = Regex::new(r"[^\n]+").unwrap();
    let mats = re.find_iter(contents.as_str());

    let len = mats.count();


    let mut graph : GraphMap<(usize, usize), u64, Directed> = GraphMap::new();

    let mut nodes: Vec<(usize, usize)> = Vec::new();

    for (i, mat) in re.find_iter(contents.as_str()).enumerate()
    {
        for (j, val) in mat.as_str().bytes().enumerate()
        {
            nodes.push(graph.add_node((i, j)));
            add_edges_to_fully_connected_graph_with_this_is_getting_to_be_a_long_name_not_sure_what_to_do_about_it_no_diagonals_though(
                &mut graph, (i, j), val as u64 - 48, len
            );
        }
    }
    (graph, nodes)
}

fn expand_graph(graph: &mut GraphMap<(usize, usize), u64, Directed>, nodes: &mut Vec<(usize, usize)>, n: usize)
{
    for i in 0..n
    {

    }
}

fn add_edges_to_fully_connected_graph_with_this_is_getting_to_be_a_long_name_not_sure_what_to_do_about_it_no_diagonals_though
(graph: &mut GraphMap<(usize, usize), u64, Directed>, pos: (usize, usize), cost: u64, graph_len: usize)
{
    if pos.0 > 0
    {
        graph.add_edge((pos.0 - 1, pos.1), (pos.0, pos.1),cost);
    }
    if pos.0 < graph_len
    {
        graph.add_edge((pos.0 + 1, pos.1), (pos.0, pos.1),cost);
    }
    if pos.1 > 0
    {
        graph.add_edge((pos.0, pos.1 - 1), (pos.0, pos.1),cost);
    }
    if pos.1 < graph_len
    {
        graph.add_edge((pos.0, pos.1 + 1), (pos.0, pos.1),cost);
    }

}

fn main()
{
    let start = Instant::now();

    let filename = "src/bin/day15/input.txt";
    // let filename = "src/bin/day15/example_input.txt";

    let (mut graph, mut nodes) = parse_input(filename);
    let path = astar(&graph, *nodes.first().unwrap(), |finish| finish == *nodes.last().unwrap(), |e| *e.weight(), |_| 0);
    println!("Cost was: {}", path.unwrap().0);

    expand_graph(&mut graph, &mut nodes, 5);
    let path = astar(&graph, *nodes.first().unwrap(), |finish| finish == *nodes.last().unwrap(), |e| *e.weight(), |_| 0);
    println!("Cost was: {}", path.unwrap().0);

    let end = Instant::now();
    println!("Took: {:?}", end.duration_since(start));
}