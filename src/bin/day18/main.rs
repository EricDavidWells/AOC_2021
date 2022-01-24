use std::time::{Duration, Instant};
use std::fs;
use num::Integer;
use regex::Regex;

fn parse_input(filename: &str) -> Vec<String>
{
    let ret: Vec<String> = libaoc::libaoc::parse_file(filename);
    ret
}

fn reduce_pair(pair: &mut String) -> u64
{
    let mut ret: u64 = 0;
    let re = Regex::new(r"(\d+)").unwrap();

    'outer: loop
    {

        // println!("pair: {}", pair);

        let mut caps = re.captures_iter(pair.as_str()).peekable();
        let mut vals: Vec<u64> = Vec::new();
        let mut inds: Vec<usize> = Vec::new();
        let mut depths: Vec<u64> = Vec::new();

        for cap in caps
        {
            // println!("{}", cap.len());
            if cap.len() != 2 {panic!("regex confused");}
            vals.push(cap.get(1).unwrap().as_str().parse().unwrap());
            inds.push(cap.get(1).unwrap().start());
        }

        for ind in inds.iter_mut()
        {
            let dep = count_depth(pair, *ind);
            depths.push(dep);
        }

        for i in 0..inds.len()
        {
            if i > 0
            {
                if depths[i] == depths[i-1] && depths[i] > 4
                {
                    // println!("preexplode: {}", pair);
                    explode_pair(pair, &mut inds, &mut vals, i-1);
                    // println!("posexplode: {}", pair);
                    continue 'outer;
                }
            }

            if i < inds.len() - 1
            {
                if depths[i] == depths[i+1] && depths[i] > 4
                {
                    // println!("preexplode: {}", pair);
                    explode_pair(pair, &mut inds, &mut vals, i);
                    // println!("posexplode: {}", pair);
                    continue 'outer;
                }
            }
        }

        for i in 0..inds.len()
        {
            if vals[i] > 9
            {
                // println!("presplit: {}", pair);
                split_pair(pair, inds[i], vals[i]);
                // println!("postsplit: {}", pair);

                continue 'outer;
            }
        }

        break;
    }

    // println!("{}", pair);

    ret
}

fn explode_pair(pair: &mut String, inds: &mut Vec<usize>, vals: &mut Vec<u64>, i: usize)
{
    let mut left_bracket_ind = inds[i] - 1;
    let mut ind_adjust: i32 = 0;

    let tmp = vals[i].to_string().len() + vals[i+1].to_string().len() + 3;
    // println!("tmp: {}", tmp);
    for _ in 0..tmp
    {
        pair.remove(left_bracket_ind);
        ind_adjust -= 1;
    }


    pair.insert(left_bracket_ind, '0');
    ind_adjust += 1;


    if i > 0
    {

        for _ in vals[i-1].to_string().chars()
        {
            pair.remove(inds[i-1]);
            ind_adjust -= 1;
        }

        vals[i-1] += vals[i];

        for char in vals[i-1].to_string().chars().rev()
        {
            pair.insert(inds[i-1], char);
            ind_adjust += 1;
        }
    }

    if i < vals.len()-2
    {
        for _ in vals[i+2].to_string().chars()
        {
            pair.remove((inds[i+2] as i32 + ind_adjust) as usize);
        }

        vals[i+2] += vals[i+1];

        for char in vals[i+2].to_string().chars().rev()
        {
            pair.insert((inds[i+2] as i32 + ind_adjust) as usize, char);
        }
    }
}

fn split_pair(pair: &mut String, ind: usize, val: u64)
{

    let left = val.div_floor(&2);
    let right = val.div_ceil(&2);

    let val = format!("[{},{}]", left, right);

    pair.remove(ind);
    pair.remove(ind);
    for char in val.chars().rev()
    {
        pair.insert(ind, char);
    }
}

fn add_pair(pair1: &String, pair2: &String) -> String
{
    format!("[{},{}]", pair1, pair2)
}

fn count_depth(pair: &mut String, ind: usize) -> u64
{
    let mut depth: u64 = 0;

    for char in pair.chars().take(ind)
    {
        if char == '[' {depth += 1;}
        if char == ']' {depth -= 1;}
    }

    depth
}

fn add_all_pairs(pairs: &Vec<String>) -> String
{
    let mut ret: String = String::new();

    let mut pairs_it = pairs.iter();
    ret = pairs_it.next().unwrap().clone();

    // println!("ret: {}", ret);
    // reduce_pair(&mut ret);
    // println!("ret: {}", ret);


    for pair in pairs_it
    {
        // println!("preadd : {}", ret);
        ret = add_pair(&ret, pair);
        println!("postadd: {}", ret);
        reduce_pair(&mut ret);
        println!("ret: {}", ret);
    }

    ret
}

fn get_magnitude(pair: &String) -> u64
{
    let mut pair_cp = pair.clone();

    let re = Regex::new(r"(\d+),(\d+)").unwrap();

    'outer: loop {
        let mut caps = re.captures_iter(&pair_cp);

        for cap in caps
        {
            let val1: u64 = cap.get(1).unwrap().as_str().parse().unwrap();
            let val2: u64 = cap.get(2).unwrap().as_str().parse().unwrap();
            let ind = cap.get(1).unwrap().start() - 1;
            for _ in 0..(cap.get(2).unwrap().end() - cap.get(1).unwrap().start() +2)
            {
                pair_cp.remove(ind);
            }

            let val_str = format!("{}", val1*3 + val2*2);

            for char in val_str.chars().rev()
            {
                pair_cp.insert(ind, char);
            }
            continue 'outer;
        }
        break;
    }

    pair_cp.parse().unwrap()
}

fn find_max_magnitude_twosum(pairs: &Vec<String>) -> u64
{
    let mut ret: u64 = 0;

    for i in 0..pairs.len()
    {
       for j in 1..pairs.len()
       {
           let p1 = pairs.get(i).unwrap().clone();
           let p2 = pairs.get(j).unwrap().clone();

           let mut pcomb1 = add_pair(&p1, &p2);
           let mut pcomb2 = add_pair(&p2, &p1);

           reduce_pair(&mut pcomb1);
           reduce_pair(&mut pcomb2);

           ret = std::cmp::max(ret, get_magnitude(&pcomb1));
           ret = std::cmp::max(ret, get_magnitude(&pcomb2));
       }
    }

    ret

}

fn main()
{
    let start = Instant::now();

    let filename = "src/bin/day18/input.txt";
    // let filename = "src/bin/day18/example_input.txt";

    let mut pairs = parse_input(filename);

    let result = add_all_pairs(&pairs);
    println!("result: {}", result);

    let magnitude = get_magnitude(&result);
    println!("magnitude: {}", magnitude);

    let magnitude_2 = find_max_magnitude_twosum(&pairs);
    println!("magnitude: {}", magnitude_2);

    println!("Took: {:?}", Instant::now().duration_since(start));
}
