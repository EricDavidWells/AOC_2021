use std::time::{Duration, Instant};
use regex::Regex;
use petgraph::{Graph, Undirected};
use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[derive(Clone, Copy, Ord, Eq, PartialOrd, Hash)]
struct Cave
{
    val: u64,
    visits: u64,
}

impl PartialEq for Cave
{
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

fn count_paths(graph: &mut GraphMap<Cave, u8, Undirected>, dehasher: &HashMap<u64, String>) -> Vec<Vec<String>>
{
    let mut ret: Vec<Vec<String>> = Vec::new();
    let mut curpath: Vec<String> = Vec::new();

    for n in graph.nodes()
    {
        if dehasher.get(&n.val).unwrap() == "start"
        {
            recurse_count(&n, graph, dehasher, &mut ret, &mut curpath);
            break;
        }
    }

    ret
}

fn count_paths_2(graph: &mut GraphMap<Cave, u8, Undirected>, dehasher: &HashMap<u64, String>) -> Vec<Vec<String>>
{
    let mut ret: Vec<Vec<String>> = Vec::new();
    let mut curpath: Vec<String> = Vec::new();
    let mut doublepath: bool = false;

    for n in graph.nodes()
    {
        if dehasher.get(&n.val).unwrap() == "start"
        {
            recurse_count_2(&n, graph, dehasher, &mut ret, &mut curpath, &mut doublepath);
            break;
        }
    }

    ret
}

fn recurse_count(cave: &Cave, graph: &GraphMap<Cave, u8, Undirected>, dehasher: &HashMap<u64, String>, paths: &mut Vec<Vec<String>>, curpath: &mut Vec<String>)
{
    let node_name = dehasher.get(&cave.val).unwrap().to_string();
    curpath.push(node_name.clone());

    // base condition success
    if node_name == "end".to_string()
    {
        paths.push(curpath.clone());
        curpath.pop();
        return
    }

    // base condition failure, not capital and visited once already
    if !is_capital(&node_name) && curpath.iter().filter(|&x| *x == node_name).count() >= 2
    {
        curpath.pop();
        return;
    }

    // successful path, continue iteration
    for neighbour in graph.neighbors(*cave)
    {
        recurse_count(&neighbour, graph, dehasher, paths, curpath);
    }

    // finished checking
    curpath.pop();
    return;
}

fn recurse_count_2(cave: &Cave, graph: &GraphMap<Cave, u8, Undirected>, dehasher: &HashMap<u64, String>, paths: &mut Vec<Vec<String>>, curpath: &mut Vec<String>, doublecave: &mut bool)
{
    let node_name = dehasher.get(&cave.val).unwrap().to_string();
    curpath.push(node_name.clone());

    let mut doublecaveswap = false;

    // base condition success
    if node_name == "end".to_string()
    {
        paths.push(curpath.clone());
        curpath.pop();
        return
    }

    // base condition failure, not capital and visited once already
    if !is_capital(&node_name)
    {

        let count = curpath.iter().filter(|&x| *x == node_name).count();

        if count <= 1
        {
            // do nothing all is fine
        }
        else if count == 2 && !*doublecave && node_name != "start"
        {
            *doublecave = true;
            doublecaveswap = true;
        }
        else
        {
            curpath.pop();
            return;
        }
    }

    // successful path, continue iteration
    for neighbour in graph.neighbors(*cave)
    {
        recurse_count_2(&neighbour, graph, dehasher, paths, curpath, doublecave);
    }

    if doublecaveswap
    {
        *doublecave = false;
    }

    // finished checking
    curpath.pop();
    return;
}

fn is_capital(input: &String) -> bool
{
    let ret = if input.as_bytes()[0] >= 65 && input.as_bytes()[0] <= 90 {true} else {false};
    ret
}


fn parse_input(filename: &str) ->  (GraphMap<Cave, u8, Undirected>, HashMap<u64, String>)
{

    let re = Regex::new(r"\w+").unwrap();

    let mut graph_dehasher: HashMap<u64, String> = HashMap::new();
    let mut graph : GraphMap<Cave, u8, Undirected> = GraphMap::new();
    let lines = libaoc::libaoc::parse_file::<String>(filename);

    for line in lines
    {
        let mut mats = re.find_iter(line.as_str()).into_iter();

        let n1 = mats.next().unwrap().as_str();
        let n1_2 = calculate_hash(&n1);

        let c1 = Cave{val: n1_2, visits: 0};

        let n2 = mats.next().unwrap().as_str();
        let n2_2 = calculate_hash(&n2);

        let c2 = Cave{val: n2_2, visits: 0};


        if !graph.contains_node(c1)
        {
            graph.add_node(c1);
            graph_dehasher.insert(n1_2, n1.to_owned());
        }

        if !graph.contains_node(c2)
        {
            graph.add_node(c2);
            graph_dehasher.insert(n2_2, n2.to_owned());
        }

        graph.add_edge(c1, c2,  1);
    }

    (graph, graph_dehasher)
}


fn main()
{
    let start = Instant::now();

    let filename = "src/bin/day12/input.txt";
    // let filename = "src/bin/day12/example_input.txt";

    let (mut graph, graph_dehasher) = parse_input(filename);

    for n in graph.nodes()
    {
        println!("{}", graph_dehasher.get(&n.val).unwrap());
    }

    let paths = count_paths(&mut graph, &graph_dehasher);
    println!("Path count: {}", paths.len());

    let paths2 = count_paths_2(&mut graph, &graph_dehasher);
    println!("Path count: {}", paths2.len());

    let end = Instant::now();
    println!("Took: {:?}", end.duration_since(start));

}