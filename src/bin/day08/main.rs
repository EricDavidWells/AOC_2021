use regex::Regex;
use std::fs;
use std::time::{Duration, Instant};

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
                patterns.push(String::from(mat.as_str()).into_bytes());
            }
            else{
                outputs.push(String::from(mat.as_str()).into_bytes());
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


/// holds all elements
#[derive(Clone)]
struct record
{
    symbols: Vec<symbol>,
    solutions: Vec<Vec<u8>>,
    solved: Vec<u8>
}

impl Default for record
{
    fn default() -> record
    {
        let symbols: Vec<u8> = vec![97, 98, 99, 100, 101, 102, 103];
        let solutions: Vec<String> = vec![
            "cf".to_string(),
            "acf".to_string(),
            "bcdf".to_string(),
            "acdeg".to_string(),
            "acdfg".to_string(),
            "abdfg".to_string(),
            "abcefg".to_string(),
            "abdefg".to_string(),
            "abcdfg".to_string(),
            "abcdefg".to_string()
        ];

        let mut ret: record = record{
            symbols: Vec::new(),
            solutions: Vec::new(),
            solved: Vec::new()
        };

        for symbol in &symbols
        {
            ret.symbols.push(symbol {value: *symbol, candidates: symbols.clone(), solved: false});
        }

        for solution in solutions
        {
            ret.solutions.push(solution.into_bytes());
        }

        ret
    }
}

/// holds an encrypted value, and the solution candidates
#[derive(Clone)]
struct symbol
{
    candidates: Vec<u8>,
    value: u8,
    solved: bool
}



fn solve_problem(problem: &mut Problem)
{
    let mut rec: record = record::default();

    // while not solved
    while !check_solved(&mut rec)
    {
        // iterate over each pattern
        for pattern in &problem.0
        {
            let num_unique = rec.solutions.iter().fold(0, |acc, x| {if x.len() == pattern.len() {return acc + 1} acc});

            if num_unique == 1
            {
                let sol = rec.solutions.iter().find(|x| x.len() == pattern.len()).unwrap().clone();
                update_from_unique(&mut rec, pattern, &sol);
            }
        }

        update_rec(&mut rec);
        update_problem(&mut rec, problem);
    }
}


/// take matching symbol chars from unique length puzzle and answer and update record
fn update_from_unique(rec: &mut record, pattern: &Vec<u8>, symbol_ans: &Vec<u8>)
{

    if pattern.len() != symbol_ans.len()
    {
        panic!("bad call to update_from_unique");
    }

    for i in 0..rec.symbols.len()
    {
        if pattern.contains(&rec.symbols[i].value)
        {
            rec.symbols[i].candidates.retain(|x| symbol_ans.contains(&x));
        }
        else
        {
            rec.symbols[i].candidates.retain(|x| !symbol_ans.contains(&x));
        }
    }

    rec.solutions.retain(|x| x != symbol_ans);
}

fn update_rec(rec: &mut record)
{
    for i in 0..rec.symbols.len()
    {
        if rec.symbols[i].candidates.len() == 1 && !rec.symbols[i].solved
        {
            rec.symbols[i].solved = true;
            rec.solved.push(rec.symbols[i].candidates[0]);

            let tmp = rec.symbols[i].candidates[0];

            for j in 0..rec.symbols.len()
            {
                if j == i {continue;}

                rec.symbols[j].candidates.retain(|y| *y != tmp);
            }

            for sol in &mut rec.solutions
            {
                sol.retain(|x| *x != rec.symbols[i].candidates[0]);
            }
        }
    }
}

fn update_problem(rec: &mut record, problem: &mut Problem)
{
    for i in 0..rec.symbols.len()
    {
        if rec.symbols[i].solved
        {
            for pattern in &mut problem.0
            {
                pattern.retain(|x| *x != rec.symbols[i].value);
            }
        }
    }

}

fn check_solved(rec: &mut record) -> bool
{
    let mut ret = true;

    for element in &rec.symbols
    {
        if element.candidates.len() != 1
        {
            return false
        }
    }

    for i in 0..rec.solutions.len()
    {
        let mut tmp = true;
        for val in &rec.solutions[i]
        {
            if !rec.solved.contains(val)
            {
                tmp = false;
            }
        }

        if tmp
        {
            rec.solutions.remove(i);
            break;
        }
    }

    ret
}

fn main()
{
    // let filename = "src/bin/day08/input.txt";
    let filename = "src/bin/day08/example_input.txt";

    let start = Instant::now();

    let mut problems = parse_input(filename);
    println!("part 1 {}", count_unique(&problems));

    for problem in &mut problems
    {
        solve_problem(problem);
    }

    let end = Instant::now();
    println!("Took: {:?}", end.duration_since(start));
}