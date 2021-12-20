use regex::Regex;
use std::fs;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use bimap::BiMap;

type Problem = (Vec<Vec<u8>>, Vec<Vec<u8>>);
type ProblemSet = Vec<Problem>;

fn parse_input(filename: &str) -> ProblemSet
{
    // let mut patterns: Vec<Vec<Vec<String>>> = Vec::new();
    // let mut outputs: Vec<Vec<Vec<String>>> = Vec::new();

    let mut problems: ProblemSet = Vec::new();

    let re = Regex::new(r"([A-z])+").unwrap();

    let lines = libaoc::libaoc::parse_file::<String>(filename);

    for line in lines
    {

        let mut patterns: Vec<Vec<u8>> = Vec::new();
        let mut outputs: Vec<Vec<u8>> = Vec::new();

        let mats = re.find_iter(line.as_str());

        let mut n = 0;
        for mat in mats
        {
            if n < 10 {
                let mut tmp = String::from(mat.as_str()).into_bytes();
                tmp.sort();
                patterns.push(tmp);
            }
            else{
                let mut tmp = String::from(mat.as_str()).into_bytes();
                tmp.sort();
                outputs.push(tmp);
            }
            n += 1;
        }

        problems.push((patterns, outputs));
    }

    problems
}

fn count_unique(problems: &ProblemSet) -> u64
{
    let mut count: u64 = 0;

    for (_, output) in problems
    {
        for val in output
        {
            match val.len()
            {
                2 => count += 1,
                3 => count += 1,
                4 => count += 1,
                7 => count += 1,
                _ => continue
            }
        }
    }

    count
}

fn solve_problems_2(problems: &ProblemSet) -> u64
{
    let mut ret: u64 = 0;

    for problem in problems
    {
        let mut pattern_2_num_m: BiMap<Vec<u8>, u8> = BiMap::new();

        for pattern in &problem.0
        {
            match pattern.len()
            {
                2 => {
                    pattern_2_num_m.insert(pattern.clone(), 1);
                },
                3 => {
                    pattern_2_num_m.insert(pattern.clone(), 7);
                },
                4 => {
                    pattern_2_num_m.insert(pattern.clone(), 4);
                }
                7 => {
                    pattern_2_num_m.insert(pattern.clone(), 8);
                }
                _ => {}
            }
        }

        for pattern in &problem.0
        {
            match pattern.len()
            {
                5=> {   // 2, 3, 5
                    let four = pattern_2_num_m.get_by_right(&4).unwrap();
                    let seven = pattern_2_num_m.get_by_right(&7).unwrap();

                    let four_sum = four.iter().fold(0, |acc, x| if pattern.contains(x){return acc +1} else{return acc});
                    let seven_sum = seven.iter().fold(0, |acc, x| if pattern.contains(x){return acc +1} else{return acc});

                    if four_sum == 2
                    {
                        pattern_2_num_m.insert(pattern.clone(), 2);
                    }
                    else
                    {
                        if seven_sum == 3
                        {
                            pattern_2_num_m.insert(pattern.clone(), 3);
                        }
                        else
                        {
                            pattern_2_num_m.insert(pattern.clone(), 5);
                        }
                    }

                },
                6 => {  // 0, 6, 9
                    let four = pattern_2_num_m.get_by_right(&4).unwrap();
                    let seven = pattern_2_num_m.get_by_right(&7).unwrap();

                    let four_sum = four.iter().fold(0, |acc, x| if pattern.contains(x){return acc +1} else{return acc});
                    let seven_sum = seven.iter().fold(0, |acc, x| if pattern.contains(x){return acc +1} else{return acc});

                    if seven_sum == 2
                    {
                        pattern_2_num_m.insert(pattern.clone(), 6);
                    }
                    else
                    {
                        if four_sum == 4
                        {
                            pattern_2_num_m.insert(pattern.clone(), 9);
                        }
                        else
                        {
                            pattern_2_num_m.insert(pattern.clone(), 0);
                        }
                    }
                },
                _ => {}
            }
        }

        for i in 0..problem.1.len()
        {
            ret += (*pattern_2_num_m.get_by_left(&problem.1[i]).unwrap() as u64) * 10_u64.pow(3 - i as u32);
        }
    }

    ret

}

fn main()
{
    let start = Instant::now();

    let filename = "src/bin/day08/input.txt";
    // let filename = "src/bin/day08/example_input.txt";

    let mut problems = parse_input(filename);
    println!("part 1 {}", count_unique(&problems));


    let sum = solve_problems_2(&problems);
    println!("part 2 {}", sum);


    let end = Instant::now();
    println!("Took: {:?}", end.duration_since(start));
}