use std::time::Instant;
use std::fs;
use std::ops::Deref;
use ndarray::{Array2, ArrayView2, Axis, s};

fn parse_input(filename: &str) -> (Vec<bool>, Array2<bool>)
{

    let contents: Vec<String> = libaoc::libaoc::parse_file(filename);

    let mut contents_iter = contents.iter().peekable();

    let mut ret_enhancement: Vec<bool> = Vec::new();
    for char in contents_iter.next().unwrap().chars()
    {
        ret_enhancement.push(char == '#');
    }

    contents_iter.next();

    let mut ret_input: Array2<bool> = Array2::from_elem((contents_iter.clone().count(), contents_iter.peek().unwrap().len()), false);

    for (i, line) in contents_iter.enumerate()
    {
        let line_chars = line.chars();
        for (j, char) in line.chars().enumerate()
        {
            ret_input[[i, j]] = char == '#';
        }
    }

    (ret_enhancement, ret_input)

}


pub fn log_bool_array(arr: &Array2<bool>)
{
    for row in arr.rows()
    {
        let mut msg: String = String::new();

        for val in row
        {
            msg.push(if *val {'#'} else {'.'});
        }
        println!("{}", msg);
    }
}

/// can't find a good way to not allocate a whole new array
fn expand(input: &Array2<bool>, n: usize, val: bool) -> Array2<bool>
{

    let shape = input.shape();
    let mut ret: Array2<bool> = Array2::from_elem((shape[0] + 2*n, shape[1] + 2*n), val);

    let start_ind: usize = n;
    let end_ind: usize = n + shape[0];

    for i in start_ind..end_ind
    {
        for j in start_ind..end_ind
        {
            ret[[i, j]] = input[[i - n, j - n]]   ;
        }
    }

    ret
}

fn enhance(input: &mut Array2<bool>, algo: &Vec<bool>, n: usize) -> Array2<bool>
{

    let mut input_mut: Array2<bool> = input.clone();

    for _ in 0..n
    {
        for i in 1..(input.shape()[0] - 1)
        {
            for j in 1..(input.shape()[0] - 1)
            {
                let mut window = input.slice(s![(i-1)..(i+2), (j-1)..(j+2)]);
                let mut bool_vec: Vec<bool> = Vec::new();
                for row in window.rows()
                {
                    for val in row
                    {
                        bool_vec.push(*val);
                    }
                }

                let val: usize = libaoc::libaoc::boolvec_to_int(&bool_vec) as usize;

                input_mut[[i, j]] = algo[val];
            }
        }

        for i in 0..input.shape()[0]
        {
            for j in 0..input.shape()[1]
            {
                if i == 0 || i == input.shape()[0]-1 || j == 0 || j ==  input.shape()[0]-1
                {
                    input_mut[[i, j]] = !input_mut[[i, j]];
                }
            }
        }
    }




    input_mut
}

fn main()
{
    let start = Instant::now();

    let filename = "src/bin/day20/input.txt";
    // let filename = "src/bin/day20/example_input.txt";

    let (enhancement, input) = parse_input(filename);

    let mut input_2 = input.clone();


    for i in 0..2
    {
        let tmp = i%2 == 1 ;
        input_2 = expand(&mut input_2, 2, i%2 == 1);
        input_2 = enhance(&mut input_2, &enhancement, 1);
    }
    println!("Lit Pixels: {}", input_2.iter().fold(0, |acc, x| acc + if *x {1} else {0}));

    for i in 0..48
    {
        let tmp = i%2 == 1 ;
        input_2 = expand(&mut input_2, 2, i%2 == 1);
        input_2 = enhance(&mut input_2, &enhancement, 1);
    }
    println!("Lit Pixels: {}", input_2.iter().fold(0, |acc, x| acc + if *x {1} else {0}));

    println!("Took: {:?}", Instant::now().duration_since(start));
}