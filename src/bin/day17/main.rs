use std::fmt::Display;
use std::time::{Duration, Instant};
use std::fs;
use std::fmt;
use regex::Regex;

use std::cmp::max;

#[derive(Default)]
struct TargetArea
{
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64
}

impl TargetArea
{
    fn contains(&self, pos: &Pos) -> bool
    {
        let mut ret: bool = pos.x >= self.xmin && pos.x <= self.xmax && pos.y >= self.ymin && pos.y <= self.ymax;
        ret
    }
}

impl Display for TargetArea
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "xmin:{}, xmax:{}, ymin:{}, ymax:{}", self.xmin, self.xmax, self.ymin, self.ymax)
    }
}

#[derive(Default, Clone, Copy)]
struct Pos
{
    x: i64,
    y: i64
}

impl Display for Pos
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "x:{}, y:{}", self.x, self.y)
    }
}

#[derive(Default, Clone, Copy)]
struct Vel
{
    x: i64,
    y: i64
}

impl Display for Vel
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "x:{}, y:{}", self.x, self.y)
    }
}

fn parse_input(filename: &str) -> TargetArea
{
    let mut ret: TargetArea = TargetArea::default();

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let re = Regex::new(r"=(.*?)\.{2}(.*?)(?:$|,)").unwrap();

    let mut caps = re.captures_iter(&contents);

    let cap = caps.next().unwrap();
    ret.xmin = cap.get(1).unwrap().as_str().parse().unwrap();
    ret.xmax = cap.get(2).unwrap().as_str().parse().unwrap();

    let cap = caps.next().unwrap();
    ret.ymin = cap.get(1).unwrap().as_str().parse().unwrap();
    ret.ymax = cap.get(2).unwrap().as_str().parse().unwrap();

    ret
}

fn step(vel: &mut Vel, target: &TargetArea) -> (bool, i64)
{
    let mut ret_hit: bool = false;
    let mut ret_height: i64 = 0;

    let mut pos: Pos = Pos{x: 0, y: 0};
    let mut count = 0;

    loop {

        pos.x += vel.x;
        if vel.x > 0 {vel.x -= 1};
        if vel.x < 0 {vel.x += 1};

        pos.y += vel.y;
        vel.y -= 1;

        ret_height = max(ret_height, pos.y);

        // if hit target
        if target.contains(&pos)
        {
            ret_hit = true;
            break;
        }

        // if passed target
        if pos.x > target.xmax || pos.y < target.ymin
        {
            ret_hit = false;
            break;
        }

        count += 1;
        println!("step {}: {}", count, pos);
    }

    count += 1;
    println!("step {}: {}", count, pos);

    (ret_hit, ret_height)
}

fn find_initial_vel(target: &TargetArea) -> (Vel, i64, i64)
{
    let max_xvel = target.xmax + 1;
    let max_yvel = num::abs(target.ymin) + 1;
    let mut success_count = 0;
    let mut max_h = 0;
    let mut max_vel = Vel{x: 0, y: 0};

    for i in 0..max_xvel
    {
        for j in -max_yvel..max_yvel
        {
            let vel = Vel{x: i, y: j};
            let (success, height) = step(&mut vel.clone(), target);

            if success
            {
                success_count += 1;

                if height > max_h
                {
                    max_h = height;
                    max_vel = vel;
                }
            }

        }
    }

   (max_vel, max_h, success_count)
}

fn main()
{
    let start = Instant::now();

    let filename = "src/bin/day17/input.txt";
    // let filename = "src/bin/day17/example_input.txt";

    let mut target = parse_input(filename);

    let (vel, max_h, suc) = find_initial_vel(&target);

    println!("initial velocity: {}, height: {}, successes: {}", vel, max_h, suc);

    println!("Took: {:?}", Instant::now().duration_since(start));
}