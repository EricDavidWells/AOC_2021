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

// fn step(polymer: &Vec<char>, insertions: &HashMap<Vec<char>, char>) -> Vec<char>
// {
//     let mut polycopy: Vec<char> = Vec::new();
//
//     polycopy.push(*polymer.first().unwrap());
//
//     for i in 1..polymer.len()
//     {
//         let mut pair: Vec<char> = vec![polymer[i-1], polymer[i]];
//         if insertions.contains_key(&pair)
//         {
//             let new_val = insertions.get(&pair).unwrap().clone();
//             pair.remove(0);
//             pair.insert(0, new_val);
//             polycopy.append(&mut pair)
//         }
//         else
//         {
//             polycopy.push(pair[1]);
//         }
//     }
//
//     polycopy
// }
//
// fn count_points(polymer: &Vec<char>) -> usize
// {
//     let mut counter: HashMap<char, usize> = HashMap::new();
//
//     for val in polymer
//     {
//         if counter.contains_key(val)
//         {
//             let count = *counter.get(val).unwrap();
//             counter.insert(*val, count + 1);
//         }
//         else{ counter.insert(*val, 1);}
//     }
//
//     let mut max_count: usize = 0;
//     let mut min_count: usize = usize::MAX;
//
//     for (val, count) in counter
//     {
//         max_count = std::cmp::max(max_count, count);
//         min_count = std::cmp::min(min_count, count);
//     }
//
//     max_count - min_count
// }

fn step_smart(polymer_map: &mut HashMap<Vec<char>, usize>, count_map: &mut HashMap<char, usize>, insertions: &HashMap<Vec<char>, char>)
{

    for (polymer, count) in polymer_map.clone().iter_mut()
    {
        // for _ in 0..*count
        // {
        let char1 = polymer[0];
        let char2 = polymer[1];
        let new_char = *insertions.get(polymer).unwrap();

        *polymer_map.get_mut(&vec![char1, char2]).unwrap() -= *count;
        *polymer_map.get_mut(&vec![char1, new_char]).unwrap() += *count;
        *polymer_map.get_mut(&vec![new_char, char2]).unwrap() += *count;

        if count_map.contains_key(&new_char)
        {
            *count_map.get_mut(&new_char).unwrap() += *count;
        }
        else
        {
            count_map.insert(new_char, *count);
        }
        // }
        // *count_map.get_mut(&new_char).unwrap() += 1;
    }

}

fn count_points_smart(count_map: &HashMap<char, usize>) -> usize
{
    let mut max_count: usize = 0;
    let mut min_count: usize = usize::MAX;

    for (_, count) in count_map
    {
        max_count = std::cmp::max(max_count, *count);
        min_count = std::cmp::min(min_count, *count);
    }

    max_count - min_count

}

fn main()
{
    let start = Instant::now();

    let filename = "src/bin/day14/input.txt";
    // let filename = "src/bin/day14/example_input.txt";

    let (template, insertions) = parse_input(filename);

    let mut polymer_map: HashMap<Vec<char>, usize> = HashMap::new();
    let mut count_map: HashMap<char, usize> = HashMap::new();

    for val in template.clone()
    {
        if count_map.contains_key(&val)
        {
            *count_map.get_mut(&val).unwrap() += 1;
        }
        else
        {
            count_map.insert(val, 1);
        }
    }

    for (polymer, char) in insertions.clone()
    {
        polymer_map.insert(polymer, 0);
    }

    for i in 1..template.len()
    {
        let val: Vec<char> = vec![template[i-1], template[i]];
        if polymer_map.contains_key(&val)
        {
            *polymer_map.get_mut(&val).unwrap() += 1;
        }
        else
        {
            polymer_map.insert(val, 1);
        }
    }

    for _ in 0..10
    {
        step_smart(&mut polymer_map, &mut count_map, &insertions);
    }
    println!("points after 10: {}", count_points_smart(&count_map));

    for _ in 0..30
    {
        step_smart(&mut polymer_map, &mut count_map, &insertions);
    }
    println!("points after 40: {}", count_points_smart(&count_map));

    let end = Instant::now();
    println!("Took: {:?}", end.duration_since(start));
}