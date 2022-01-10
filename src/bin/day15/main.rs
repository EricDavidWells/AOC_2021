use std::time::{Duration, Instant};
use std::fs;
use std::collections::{HashMap};
use std::ops::Index;

use regex::Regex;
use petgraph::{Graph, Undirected};
use petgraph::prelude::*;
use petgraph::algo::{astar, dijkstra};
use ndarray::{Array2, Zip, Axis};

fn parse_input(filename: &str) -> Array2<usize>
{
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let re = Regex::new(r"[^\n]+").unwrap();
    let mats = re.find_iter(contents.as_str());

    let len = mats.count();


    // let mut graph : GraphMap<(usize, usize), u64, Directed> = GraphMap::new();
    // let mut nodes: Vec<(usize, usize)> = Vec::new();

    let mut weights: Array2<usize> = Array2::from_elem((len, len), 0);

    for (i, mat) in re.find_iter(contents.as_str()).enumerate()
    {
        for (j, val) in mat.as_str().bytes().enumerate()
        {
            weights[[i, j]] = val as usize - 48;
        }
    }

    weights
}

fn make_graph_from_weights(weights: &Array2<usize>) -> (GraphMap<(usize, usize), u64, Directed>, Vec<(usize, usize)>)
{
    let mut graph : GraphMap<(usize, usize), u64, Directed> = GraphMap::new();
    let mut nodes: Vec<(usize, usize)> = Vec::new();

    for i in 0..weights.nrows()
    {
        for j in 0..weights.ncols()
        {
            nodes.push(graph.add_node((i, j)));
            add_edges_to_fully_connected_graph_with_this_is_getting_to_be_a_long_name_not_sure_what_to_do_about_it_no_diagonals_though(
                &mut graph, (i, j), weights[[i, j]] as u64, weights.nrows()
            );
        }
    }

    (graph, nodes)
}

// this is pretty trash and confusing but it works
fn expand_weights(weights: &mut Array2<usize>, n: usize) -> Array2<usize>
{

    let mut ret: Array2<usize> = weights.clone();

    let mut row_start_vals: Array2<usize> = weights.clone();

    for i in 0..n
    {
        let mut next_block_vals = row_start_vals.clone();
        let mut row_agregator = next_block_vals.clone();

        for j in 1..n
        {
            Zip::from(&mut next_block_vals)
                .for_each(|x| {
                    *x = ((*x + 1) - 1)%9 + 1;
                    if *x > 9
                    {
                        *x = 1;
                    }
                });

            row_agregator.append(Axis(1), next_block_vals.view()).unwrap();

        }

        // increment new weights
        Zip::from(&mut row_start_vals)
            .for_each(|x| {
                *x = ((*x + 1) - 1)%9 + 1;
                if *x > 9
                {
                    *x = 1;
                }
            });

        if i == 0
        {
            ret = row_agregator.clone();
        }
        else
        {
            ret.append(Axis(0), row_agregator.view()).unwrap();
        }
    }

    ret
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

    let mut weights = parse_input(filename);
    let (mut graph, mut nodes) = make_graph_from_weights(&weights);

    let path = astar(&graph, *nodes.first().unwrap(), |finish| finish == *nodes.last().unwrap(), |e| *e.weight(), |_| 0);
    println!("Cost was: {}", path.unwrap().0);

    let new_weights = expand_weights(&mut weights, 5);
    let (mut graph2, mut nodes2) = make_graph_from_weights(&new_weights);

    let path2 = astar(&graph2, *nodes2.first().unwrap(), |finish| finish == *nodes2.last().unwrap(), |e| *e.weight(), |_| 0);
    println!("Cost was: {}", path2.unwrap().0);

    let end = Instant::now();
    println!("Took: {:?}", end.duration_since(start));
}