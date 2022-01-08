use std::time::{Duration, Instant};
use std::fs;

use regex::Regex;
use ndarray::{Array2, ArrayView, Axis, Zip, s, Slice};
use libaoc::libaoc::log_array;

fn parse_input(filename: &str) ->  (Array2<bool>, Vec<(usize, usize)>)
{
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    // parse points

    let mut points: Vec<(usize, usize)> = Vec::new();
    let re = Regex::new(r"(\d+),(\d+)").unwrap();

    for cap in re.captures_iter(contents.as_str())
    {
        if cap.iter().count() != 3 {panic!("wtf even is regex really tho");}

        let mut it = cap.iter();
        it.next();

        let val1: usize = it.next().unwrap().unwrap().as_str().parse().unwrap();
        let val2: usize = it.next().unwrap().unwrap().as_str().parse().unwrap();
        points.push((val1, val2));
    }

    let max_x = points.iter().fold(0, |acc, x| std::cmp::max(acc, x.1));
    let max_y = points.iter().fold(0, |acc, x| std::cmp::max(acc, x.0));

    let mut ret: Array2<bool> = Array2::from_elem((max_x + 1, max_y + 1), false);

    for point in points
    {
        ret[[point.1, point.0]] = true;
    }

    // parse folds

    let mut ret2: Vec<(usize, usize)> = Vec::new();

    let re2 = Regex::new(r"along (x|y)=(\d+)").unwrap();
    for cap in re2.captures_iter(contents.as_str())
    {
        if cap.iter().count() != 3 {panic!("wtf even is regex really tho");}

        let mut it = cap.iter();
        it.next();

        let val1 = it.next().unwrap().unwrap().as_str();
        let val2: usize = it.next().unwrap().unwrap().as_str().parse().unwrap();

        ret2.push(((val1 == "x") as usize, val2));
    }

    (ret, ret2)
}

fn fold_paper_n(paper: &mut Array2<bool>, folds: &Vec<(usize, usize)>, n: usize)
{
    for i in 0..n
    {
        fold_paper(paper, folds[i].0, folds[i].1);
    }
}

fn fold_paper(paper: &mut Array2<bool>, axis: usize, ind: usize)
{
    let mut paper2 = paper.clone();
    paper2.invert_axis(Axis(axis));

    Zip::from(&mut *paper)
        .and(& paper2)
        .for_each(|x, b| {
            *x = *x | *b;
        });


    paper.slice_axis_inplace(Axis(axis), Slice::from(0..ind));

    // // Alternative slower method
    // while paper.shape()[axis] > ind
    // {
    //     paper.remove_index(Axis(axis), ind);
    //     // paper.slice_axis_in
    // }
}

fn count_dots(paper: &Array2<bool>) -> usize
{
    let mut ret: usize = 0;

    Zip::from(paper)
        .for_each(|x| {
            if *x
            {
                ret += 1;
            }
        });

    ret
}

fn pretty_log_array(paper: &Array2<bool>)
{
    for row in paper.rows()
    {
        let mut msg: String = String::new();

        for val in row
        {
            if *val
            {
                msg.push_str("#");
            }
            else
            {
                msg.push_str(" ");
            }
        }
        println!("{}", msg);
    }
}

fn main()
{
    let start = Instant::now();

    let filename = "src/bin/day13/input.txt";
    // let filename = "src/bin/day13/example_input.txt";

    let (mut paper, folds) = parse_input(filename);
    let mut paper2 = paper.clone();

    fold_paper_n(&mut paper, &folds, 1);
    println!("Dot count after 1 fold: {}", count_dots(&paper));

    fold_paper_n(&mut paper2, &folds, folds.len());
    pretty_log_array(&paper2);

    let end = Instant::now();
    println!("Took: {:?}", end.duration_since(start));
}