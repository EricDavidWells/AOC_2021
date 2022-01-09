use std::time::{Duration, Instant};
use std::fs;
use std::collections::{HashMap, LinkedList};
use std::collections::linked_list;
use std::hash::Hash;

use regex::Regex;


fn parse_input(filename: &str) -> (Vec<char>, HashMap<Vec<char>, char>)
{
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let re_1 = Regex::new(r"(^\w+)").unwrap();
    let template: Vec<char> = re_1.find(contents.as_str()).unwrap().as_str().chars().collect();

    let mut insertions: HashMap<Vec<char>, char> = HashMap::new();
    let re = Regex::new(r"(\w{2}) -> (\w)").unwrap();
    for cap in re.captures_iter(contents.as_str())
    {
        if cap.iter().count() != 3 {panic!("wtf even is regex really tho");}

        let mut it = cap.iter();
        it.next();

        let val1: Vec<char> = it.next().unwrap().unwrap().as_str().chars().collect();
        let val2: char = it.next().unwrap().unwrap().as_str().chars().next().unwrap();
        insertions.insert(val1, val2);
    }

    (template, insertions)
}

fn step(polymer: &Vec<char>, insertions: &HashMap<Vec<char>, char>) -> Vec<char>
{
    let mut polycopy: Vec<char> = Vec::new();

    polycopy.push(*polymer.first().unwrap());

    for i in 1..polymer.len()
    {
        let mut pair: Vec<char> = vec![polymer[i-1], polymer[i]];
        if insertions.contains_key(&pair)
        {
            let new_val = insertions.get(&pair).unwrap().clone();
            pair.remove(0);
            pair.insert(0, new_val);
            polycopy.append(&mut pair)
        }
        else
        {
            polycopy.push(pair[1]);
        }
    }

    polycopy
}


fn step_smart(polymer_map: HashMap<Vec<char>, usize>, insertions: &HashMap<Vec<char>, char>) -> Vec<char>
{

}

fn count_points(polymer: &Vec<char>) -> usize
{
    let mut counter: HashMap<char, usize> = HashMap::new();

    for val in polymer
    {
        if counter.contains_key(val)
        {
            let count = *counter.get(val).unwrap();
            counter.insert(*val, count + 1);
        }
        else{ counter.insert(*val, 1);}
    }

    let mut max_count: usize = 0;
    let mut min_count: usize = usize::MAX;

    for (val, count) in counter
    {
        max_count = std::cmp::max(max_count, count);
        min_count = std::cmp::min(min_count, count);
    }

    max_count - min_count
}

fn main()
{
    let start = Instant::now();

    let filename = "src/bin/day14/input.txt";
    // let filename = "src/bin/day14/example_input.txt";

    let (template, insertions) = parse_input(filename);
    // let mut polymer = LinkedList::from_iter(template.iter());

    let mut polymer: Vec<char> = template.clone();
    for _ in 0..10
    {
        polymer = step(&polymer, &insertions);
        println!("polymer length: {}", polymer.len());
    }

    println!("points: {}", count_points(&polymer));

    for _ in 0..30
    {
        polymer = step(&polymer, &insertions);
        println!("polymer length: {}", polymer.len());
    }

    println!("points: {}", count_points(&polymer));

    let end = Instant::now();
    println!("Took: {:?}", end.duration_since(start));
}