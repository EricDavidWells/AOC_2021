use std::fs;
use std::time::{Duration, Instant};
use ndarray::{Array2, ArrayView, Axis};

fn parse_input(filename: &str) -> Array2<u8>
{


    let lines = libaoc::libaoc::parse_file::<String>(filename);

    let mut ret: Array2<u8> = Array2::zeros((lines.len(), lines[0].len()));

    for (i, line) in lines.iter().enumerate()
    {
        let chars: Vec<u8> = line.chars()
            .map(|x| (x as u8 - 48 as u8))
            .collect();
        for j in 0..chars.len()
        {
            ret[[i, j]] = chars[j];
        }
    }

    ret
}

fn is_valley_adjacent(hmap: &Array2<u8>, i: usize, j: usize) -> bool
{
    let mut ret = true;
    let val = hmap[[i, j]];

    if i > 0    // check above
    {
        ret = ret && (hmap[[i-1, j]] > val);
    }

    if i < hmap.shape()[0] - 1  // check below
    {
        ret = ret && (hmap[[i+1, j]] > val);
    }

    if j > 0    // check left
    {
        ret = ret && (hmap[[i, j-1]] > val);
    }

    if j < hmap.shape()[1] - 1  // check right
    {
        ret = ret && (hmap[[i, j+1]] > val);
    }

    ret
}

fn find_valleys(hmap: &Array2<u8>) -> (Vec<(usize, usize)>, u64)
{
    let mut ret: u64 = 0;
    let mut inds: Vec<(usize, usize)> = Vec::new();

    for i in 0..hmap.shape()[0]
    {
        for j in 0..hmap.shape()[1]
        {
            if is_valley_adjacent(hmap, i, j)
            {
                ret += (hmap[[i, j]] + 1) as u64;
                inds.push((i, j));
            }
        }
    }

    (inds, ret)
}

fn count_basins(hmap: &Array2<u8>, inds: Vec<(usize, usize)>) -> Vec<usize>
{
    let mut ret: Vec<usize> = Vec::new();

    for (i, j) in inds
    {
        let mut val: usize = 0;
        recurse_count(hmap, i, j, &mut val);
        ret.push(val);
    }

    ret
}

fn recurse_count(hmap: &Array2<u8>, i: usize, j: usize, cum: &mut usize)
{

    // base case, out of index
    if i < 0 || i >= hmap.shape()[0] || j < 0 || j >= hmap.shape()[1]
    {
        return
    }

    *cum += 1;

    let curval = hmap[[i, j]];


    if hmap[[i + 1, j]] > curval
    {
        recurse_count(hmap, i+1, j, cum);
    }

    if hmap[[i - 1, j]] > curval
    {
        recurse_count(hmap, i-1, j, cum);
    }

    if hmap[[i, j+1]] > curval
    {
        recurse_count(hmap, i, j+1, cum);
    }

    if hmap[[i, j-1]] > curval
    {
        recurse_count(hmap, i, j-1, cum);
    }

}

fn main()
{
    let start = Instant::now();

    let filename = "src/bin/day09/input.txt";
    // let filename = "src/bin/day09/example_input.txt";

    let mut hmap = parse_input(filename);

    // libaoc::libaoc::log_array(&problems);

    let (inds, sum) = find_valleys(&hmap);

    println!("risk factor: {}", sum);

    let vals = count_basins(&hmap, inds);
    // println("basin")

    libaoc::libaoc::log_vec(&vals);

    let end = Instant::now();
    println!("Took: {:?}", end.duration_since(start));
}