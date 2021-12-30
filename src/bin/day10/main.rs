use std::time::{Duration, Instant};
use bimap::BiMap;


fn parse_input(filename: &str) -> Vec<Vec<char>>
{

    let lines = libaoc::libaoc::parse_file::<String>(filename);

    let mut ret: Vec<Vec<char>> = Vec::new();

    for (i, line) in lines.iter().enumerate()
    {
        ret.push(line.chars().collect());
    }

    ret
}


fn find_and_strip_mistakes(lines: &Vec<Vec<char>>, openers: &Vec<char>, closers: &Vec<char>) -> (Vec<Vec<char>>, Vec<char>)
{
    let mut ret_mistakes: Vec<char> = Vec::new();
    let mut ret_lines: Vec<Vec<char>> = Vec::new();

    'outer: for line in lines
    {
        let mut order: Vec<char> = Vec::new();

        for char in line
        {
            if openers.contains(char)
            {
                order.push(*char);
            }
            else
            {
                if closers.iter().position(|x| x == char).unwrap() == openers.iter().position(|x| x == order.last().unwrap()).unwrap()
                {
                    order.pop();
                }
                else
                {
                    ret_mistakes.push(*char);
                    continue 'outer;
                }
            }
        }

        let mut rev_order: Vec<char> = Vec::new();
        order.reverse();
        for val in &order
        {
            let ind = openers.iter().position(|x| x == val).unwrap();
            rev_order.push(closers[ind]);
        }

        ret_lines.push(rev_order);
    }

    (ret_lines, ret_mistakes)
}

fn count_points(mistakes: &Vec<char>, closers: &Vec<char>, points: &Vec<u64>) -> u64
{
    let mut ret: u64 = 0;

    for mistake in mistakes
    {
        let ind = closers.iter().position(|x| x == mistake).unwrap();
        ret += points[ind];
    }

    ret
}

fn count_points_2(completions: &Vec<Vec<char>>, closers: &Vec<char>, points: &Vec<u64>) -> u64
{

    let mut scores: Vec<u64> = Vec::new();

    for completion in completions
    {
        let mut sum: u64 = 0;
        for val in completion
        {
            let ind = closers.iter().position(|x| x == val).unwrap();
            sum *= 5;
            sum += points[ind];
            let tmp = 0;
        }
        scores.push(sum);
    }

    scores.sort();
    scores[scores.len()/2]

}

fn main()
{
    let start = Instant::now();

    let openers: Vec<char> = Vec::from(['(', '[', '{', '<']);
    let closers: Vec<char> = Vec::from([')', ']', '}', '>']);
    let points: Vec<u64> = Vec::from([3, 57, 1197, 25137]);
    let points_2: Vec<u64> = Vec::from([1, 2, 3, 4]);

    let filename = "src/bin/day10/input.txt";
    // let filename = "src/bin/day10/example_input.txt";

    let lines = parse_input(filename);
    let (completed_lines, mistakes) = find_and_strip_mistakes(&lines, &openers, &closers);

    println!("points: {}", count_points(&mistakes, &closers, &points));
    println!("points2: {}", count_points_2(&completed_lines, &closers, &points_2));



    let end = Instant::now();
    println!("Took: {:?}", end.duration_since(start));

}