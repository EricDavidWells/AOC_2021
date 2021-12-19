use regex::Regex;
use std::fs;
use std::time::{Duration, Instant};
// use chrono;

fn parse_input(filename: &str) -> Vec<u16>
{
    let mut ret: Vec<u16> = Vec::new();

    let re = Regex::new(r"[0-9]+").unwrap();
    let contents = fs::read_to_string(filename).unwrap();
    let mats = re.find_iter(contents.as_str());

    for mat in mats
    {
        ret.push(mat.as_str().parse::<u16>().unwrap());
    }

    ret
}

fn organize_crabs(crabs: &Vec<u16>) -> u64
{
    let mut min_fuel = u64::MAX;
    let max_pos = *crabs.iter().max().unwrap();

    for i in 0..max_pos
    {
        let fuel = crabs.iter().fold(0, |acc: u64, x: &u16| acc + ((*x as i16 - i as i16).abs() as u64));
        min_fuel = min_fuel.min(fuel);
    }

    min_fuel
}

fn organize_crabs_2(crabs: &Vec<u16>) -> u64
{
    let mut min_fuel = u64::MAX;

    let max_pos = *crabs.iter().max().unwrap();
    let mut possible_positions = Vec::from_iter(0..=max_pos);
    let mean_pos = crabs.iter().fold(0, |acc, x| acc + *x as u64) / (crabs.len() as u64);

    let comparator = |a: &u16, b: &u16| -> std::cmp::Ordering {

        let d1 = (*a as i64 - mean_pos as i64).abs();
        let d2 = (*b as i64 - mean_pos as i64).abs();
        d1.cmp(&d2)
    };

    possible_positions.sort_by(comparator);


    'outer: for i in possible_positions
    {
        let mut fuel = 0;

        for crab in crabs
        {
            let n = (*crab as i16 - i as i16).abs() as u64;
            fuel += (n + 1) * n/2;

            if fuel > min_fuel
            {
                continue 'outer;
            }
        }


        if fuel < min_fuel {min_fuel = fuel}
    }

    min_fuel
}

// fn organize_crabs_w_binary_search(crabs: &Vec<u16>) -> u64
// {
//     let mut min_fuel = u64::MAX;
//     let max_pos = *crabs.iter().max().unwrap();
//     let min_pos = *crabs.iter().min().unwrap();
//     let mean_pos = crabs.iter().fold(0, |acc, x| acc + *x as u64) / (crabs.len() as u64);
//
//     let possible_positions = vec!(0, max_pos);
//
//     // let btree = std::collections::BTreeSet::from_iter(possible_positions);
//
//     let comparator = |pos: &u16| -> std::cmp::Ordering {
//
//         let fuel = crabs.iter().fold(0, |acc: u64, x: &u16| acc + ((*x as i16 - *pos as i16).abs() as u64));
//         // let prev_min_fuel = min_fuel;
//         min_fuel = min_fuel.min(fuel);
//         fuel.cmp(&(0 as u64))
//     };
//
//     let val = possible_positions.binary_search_by(comparator);
//
//     println!("asdf");
//     min_fuel
// }

fn main()
{
    let filename = "src/bin/day07/input.txt";
    // let filename = "src/bin/day07/example_input.txt";

    let crabs = parse_input(filename);
    println!("min_fuel: {}", organize_crabs(&crabs));

    let start = Instant::now();
    println!("min_fuel2: {}", organize_crabs_2(&crabs));
    let end = Instant::now();

    println!("Took: {:?}", end.duration_since(start));
}