use std::cmp::{max, min};
use std::ops::Add;
use libaoc::libaoc;
use regex::Regex;
use ndarray::Array2;

struct Pipe
{
    stt: Pos,
    end: Pos
}

impl Pipe
{
    fn log(&self)
    {
        println!("{},{} -> {},{}", self.stt.x, self.stt.y, self.end.x, self.end.y);
    }
}

struct Pos
{
    x: u16,
    y: u16
}

fn parse_input(filename: &str) -> (Vec<Pipe>, Array2::<u16>)
{
    let mut pipes: Vec<Pipe> = Vec::new();
    let lines: Vec<String> = libaoc::parse_file(&filename);

    let mut max_x: u16 = 0;
    let mut max_y: u16 = 0;

    let re = Regex::new(r"[^, \->\n]+").unwrap();

    for line in lines
    {
        let mut mats = re.find_iter(&line);

        let x1 = mats.next().unwrap().as_str().parse::<u16>().unwrap();
        let y1 = mats.next().unwrap().as_str().parse::<u16>().unwrap();
        let x2 = mats.next().unwrap().as_str().parse::<u16>().unwrap();
        let y2 = mats.next().unwrap().as_str().parse::<u16>().unwrap();

        max_x = std::cmp::max(std::cmp::max(x1, x2), max_x);
        max_y = std::cmp::max(std::cmp::max(y1, y2), max_y);

        pipes.push(Pipe{
            stt: Pos {
                x: x1,
                y: y1
            },
            end: Pos{
                x: x2,
                y: y2
            }
        });
    }

    let mut diagram = ndarray::Array2::<u16>::zeros(((max_x + 1) as usize, (max_y + 1) as usize));

    (pipes, diagram)
}

fn add_horizontal_pipes(pipes: &Vec<Pipe>, diagram: &mut Array2::<u16>)
{
    for pipe in pipes
    {
        if pipe.stt.x != pipe.end.x {continue;}

        let y_strt = min(pipe.stt.y, pipe.end.y);
        let y_end = max(pipe.stt.y, pipe.end.y);

        for j in y_strt..=y_end
        {
            diagram[[pipe.stt.x as usize, j as usize]] += 1;
        }

    }
}

fn add_vertical_pipes(pipes: &Vec<Pipe>, diagram: &mut Array2::<u16>)
{
    for pipe in pipes
    {
        if pipe.stt.y != pipe.end.y {continue;}

        let x_strt = min(pipe.stt.x, pipe.end.x);
        let x_end = max(pipe.stt.x, pipe.end.x);

        for i in x_strt..=x_end
        {
            diagram[[i as usize, pipe.stt.y as usize]] += 1;
        }

    }
}

fn add_diagonal_pipes(pipes: &Vec<Pipe>, diagram: &mut Array2::<u16>)
{
    for pipe in pipes
    {
        if pipe.stt.y == pipe.end.y {continue;}
        if pipe.stt.x == pipe.end.x {continue;}

        let mut xdir = pipe.stt.x < pipe.end.x; // true if increasing
        let mut ydir = pipe.stt.y < pipe.end.y; // true if increasing

        let mut y_inc: i16;
        if (ydir){y_inc = 1;}
        else{y_inc = -1;}

        let mut x_inc: i16;
        if (xdir){x_inc = 1;}
        else{x_inc = -1;}

        for i in 0..=(pipe.stt.x as i16 - pipe.end.x as i16).abs()
        {
            let x_tmp = (pipe.stt.x as i16 + (i*x_inc)) as usize;
            let y_tmp = (pipe.stt.y as i16 + (i*y_inc)) as usize;
            diagram[[x_tmp, y_tmp]] += 1;
        }
    }
}

fn count_vals_greater_than(diagram: &Array2<u16>) -> u32
{
    let mut sum: u32 = 0;
    for val in diagram
    {
        if *val >= 2
        {
            sum += 1;
        }
    }

    sum
}

fn log_array(arr: &Array2<u16>)
{
    for row in arr.rows()
    {
        let mut msg: String = String::new();

        for val in row
        {
            msg.push_str(&val.clone().to_string());

            msg.push(',');
        }
        println!("{}", msg);
    }
}

fn main()
{

    let filename = "src/bin/day05/input.txt";
    // let filename = "src/bin/day05/example_input.txt";
    let (mut pipes, mut diagram) = parse_input(filename);

    add_horizontal_pipes(& pipes, &mut diagram);
    add_vertical_pipes(& pipes, &mut diagram);

    let val = count_vals_greater_than(&diagram);
    println!("part1: {}", val);

    add_diagonal_pipes(& pipes, &mut diagram);
    let val2 = count_vals_greater_than(&diagram);

    println!("part2: {}", val2);
}