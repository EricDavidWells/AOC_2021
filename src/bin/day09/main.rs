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

fn count_basins(hmap: &Array2<u8>, bmap: &mut Array2<bool>, inds: &Vec<(usize, usize)>) -> Vec<usize>
{
    let mut ret: Vec<usize> = Vec::new();

    for ind in inds
    {
        let mut cum: usize = 0;
        let val: i8 = -1;
        recurse_count(hmap, &mut bmap.clone(), ind.0 as i64, ind.1 as i64, val, &mut cum);
        ret.push(cum);
    }

    ret
}

fn recurse_count(hmap: &Array2<u8>, bmap: &mut Array2<bool>, i: i64, j: i64, val: i8, cum: &mut usize)
{

    // base case, out of index
    if i < 0 || i >= hmap.shape()[0] as i64 || j < 0 || j >= hmap.shape()[1] as i64
    {
        let nothing = 0;
        return
    }

    let i_u = i as usize;
    let j_u = j as usize;

    // base case, visited before
    if bmap[[i_u, j_u]]
    {
        return
    }

    let new_val = hmap[[i_u, j_u]];

    // base case, value of 9
    if new_val == 9
    {
        return
    }

    // base case, val not higher
    if  new_val as i8 <= val
    {
        return
    }

    bmap[[i_u, j_u]] = true;

    *cum += 1;

    recurse_count(hmap, bmap, i+1, j, hmap[[i_u, j_u]] as i8, cum);
    recurse_count(hmap, bmap, i-1, j, hmap[[i_u, j_u]] as i8, cum);
    recurse_count(hmap, bmap, i, j+1, hmap[[i_u, j_u]] as i8, cum);
    recurse_count(hmap, bmap, i, j-1, hmap[[i_u, j_u]] as i8, cum);

}



fn main()
{
    let start = Instant::now();

    let filename = "src/bin/day09/input.txt";
    // let filename = "src/bin/day09/example_input.txt";

    let mut hmap = parse_input(filename);
    let (inds, sum) = find_valleys(&hmap);

    println!("risk factor: {}", sum);

    let mut bmap: Array2<bool> = Array2::from_elem((hmap.shape()[0], hmap.shape()[1]), false);

    let vals = count_basins(&hmap, &mut bmap, &inds);
    let top3 = libaoc::libaoc::find_n_max(vals, 3);
    let sum: usize = top3.iter().product();
    println!("product {}", sum);


    let end = Instant::now();
    println!("Took: {:?}", end.duration_since(start));
}