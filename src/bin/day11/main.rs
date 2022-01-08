use std::time::{Duration, Instant};
use ndarray::{Array2, ArrayView, Axis, Zip};


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

fn flash(octogrid: &mut Array2<u8>, i: usize, j: usize)
{
    let min_i: usize = std::cmp::max((i as i8 - 1), 0) as usize;
    let max_i: usize = std::cmp::min(octogrid.shape()[0]-1, i + 1);

    let min_j: usize = std::cmp::max((j as i8 - 1), 0) as usize;
    let max_j: usize = std::cmp::min(octogrid.shape()[1]-1, j + 1);

    for i_ in min_i..=max_i
    {
        for j_ in min_j..=max_j
        {
            if i_ == i && j_ == j {continue;};
            octogrid[[i_, j_]] += 1;
        }
    }
}

fn step(octogrid: &mut Array2<u8>, flashes: &mut u64)
{
    let mut bgrid: Array2<bool> = Array2::from_elem(octogrid.dim(), false);

    Zip::from(&mut *octogrid)
        .for_each(|x| {
            *x += 1;
        });

    'outer: loop {
        for i in 0..octogrid.shape()[0]
        {
            for j in 0..octogrid.shape()[1]
            {
                if octogrid[[i, j]] > 9 && bgrid[[i, j]] == false
                {
                    flash(octogrid, i, j);
                    *flashes += 1;
                    bgrid[[i, j]] = true;
                    continue 'outer;
                }
            }
        }
        break 'outer;
    }

    Zip::from(&mut *octogrid)
        .and(&mut bgrid)
        .for_each(|x, b| {
            if *b
            {
                *x = 0;
            }
        });
}


fn main()
{
    let start = Instant::now();

    let filename = "src/bin/day11/input.txt";
    // let filename = "src/bin/day11/example_input.txt";

    let mut octogrid = parse_input(filename);
    let mut octogrid_2 = octogrid.clone();
    let mut flashes: u64 = 0;

    for _i in 0..100
    {
        step(&mut octogrid, &mut flashes);
    }
    println!("flashes after 100: {}", flashes);

    flashes = 0;
    let mut step_c: u64 = 0;
    let mut prev_flashes: u64 = 0;
    loop
    {
        step(&mut octogrid_2, &mut flashes);
        step_c += 1;

        if flashes - prev_flashes == octogrid.len() as u64
        {
            break;
        }
        prev_flashes = flashes;
    }

    println!("steps till sync: {}", step_c);

    let end = Instant::now();
    println!("Took: {:?}", end.duration_since(start));

}