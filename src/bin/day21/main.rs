use std::time::Instant;
use std::fs;
use regex::Regex;
use std::cmp::min;
use std::collections::HashMap;

struct DeterministicDie
{
    val: u64,
    count: u64
}

impl DeterministicDie
{
    fn roll(&mut self) -> u64
    {
        self.count += 1;

        self.val += 1;
        self.val %= 100;

        self.val + 1
    }
}

impl Default for DeterministicDie
{
    fn default() -> DeterministicDie
    {
       DeterministicDie{val: 99, count: 0}
    }
}

fn parse_input(filename: &str) -> (u64, u64)
{
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let re = Regex::new(r": (\d*)").unwrap();

    let mut caps = re.captures_iter(&contents);

    let val1: u64 = caps.next().unwrap().get(1).unwrap().as_str().parse().unwrap();
    let val2: u64 = caps.next().unwrap().get(1).unwrap().as_str().parse().unwrap();
    (val1-1, val2-1)
}

fn update_position(mut p: u64, roll: u64) -> u64
{
    p += roll;
    p %= 10;
    p
}

fn practice_game(mut p1: u64, mut p2: u64) -> u64
{
    let mut dice: DeterministicDie = Default::default();
    let mut score1 = 0;
    let mut score2 = 0;

    loop
    {

        p1 = update_position(p1, dice.roll() + dice.roll() + dice.roll());
        score1 += p1 + 1;
        if score1 >= 1000 {break;}

        p2 = update_position(p2, dice.roll() + dice.roll() + dice.roll());
        score2 += p2 + 1;
        if score2 >= 1000 {break;}
    }

    min(score1, score2) * dice.count

}

fn dirac_game(
    mut p1: u64,
    mut p2: u64,
    mut p1_score: u64,
    mut p2_score: u64,
    p1_count: &mut u64,
    p2_count: &mut u64,
    mut p1turn: bool,
    mut count: u8,
    cheatcodes: &mut HashMap<(u64, u64), (u64, u64)>)
{

    if count >= 3 {
        if p1turn
        {
            p1_score += p1 + 1;
        }
        else
        {
            p2_score += p2 + 1;
        }
        p1turn = !p1turn;
        count = 0;
    }

    if p1_score >= 21
    {
        *p1_count += 1;
        return;
    }
    if p2_score >= 21
    {
        *p2_count += 1;
        return;
    }

    if p1turn
    {
        if cheatcodes.contains_key(&(p1, p1_score)) && count == 0
        {
            let (s1, s2) = cheatcodes.get(&(p1, p1_score)).unwrap();
            *p1_count += *s1;
            *p2_count += *s2;
            return;
        }
        else {
            let p1_count_old = *p1_count;
            let p2_count_old = *p2_count;

            for i in 1..4
            {
                dirac_game(update_position(p1, i), p2,
                           p1_score, p2_score,
                           p1_count, p2_count, p1turn, count+1, cheatcodes);
            }

            if count == 0
            {
                cheatcodes.insert((p1, p1_score), (*p1_count - p1_count_old, *p2_count - p2_count_old));
            }
        }
    }
    else
    {
        for i in 1..4
        {
            dirac_game(p1, update_position(p2, i),
                       p1_score, p2_score,
                       p1_count, p2_count, p1turn, count+1, cheatcodes);
        }
    }
}

fn main()
{
    let start = Instant::now();

    // let filename = "src/bin/day21/input.txt";
    let filename = "src/bin/day21/example_input.txt";

    let (p1, p2) = parse_input(filename);
    println!("practice game result: {}", practice_game(p1, p2));

    let mut p1_count: u64 = 0;
    let mut p2_count: u64 = 0;
    let mut cheatcodes: HashMap<(u64, u64), (u64, u64)> = HashMap::new();

    dirac_game(p1, p2, 0, 0, &mut p1_count, &mut p2_count, true, 0, &mut cheatcodes);

    println!("p1 wins {}, p2 wins {}", p1_count, p2_count);

    println!("Took: {:?}", Instant::now().duration_since(start));
}