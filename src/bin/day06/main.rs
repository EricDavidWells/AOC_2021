use libaoc::libaoc;
use regex::Regex;
use std::fs;

fn parse_input(filename: &str) -> Vec<u8>
{
    let mut ret: Vec<u8> = Vec::new();

    let re = Regex::new(r"[0-9]+").unwrap();
    let contents = fs::read_to_string(filename).unwrap();
    let mut mats = re.find_iter(contents.as_str());

    for mat in mats
    {
        ret.push(mat.as_str().parse::<u8>().unwrap());
    }

    ret
}

fn increase_day_brute_force(fishes: &mut Vec<u8>, num: u32) -> usize
{

    for day in 0..num
    {
        println!("day: {}", day);
        // libaoc::log_vec(&fishes);
        let mut sum = 0;

        for i in 0..fishes.len()
        {

            if fishes[i] == 0
            {
                fishes[i] = 6;
                sum += 1;
            }
            else
            {
                fishes[i] -= 1;
            }
        }

        for i in 0..sum
        {
            fishes.push(8);
        }
    }

    fishes.len()
}

fn increase_day_smart(fishes: &mut Vec<u8>, num: u32) -> u64
{

    let mut fish_tracker: Vec<u64> = vec![0; 9];

    for i in 0..fishes.len()
    {
        fish_tracker[fishes[i] as usize] += 1;
    }

    for day in 0..num
    {
        let tmp = fish_tracker.remove(0);
        fish_tracker[6] += tmp;
        fish_tracker.push(tmp);
    }

    fish_tracker.iter().sum()

}


fn neg_wrap_mod<T>(mut val: T, modu: T) -> T
where T: Copy + std::ops::Rem<Output = T> + std::ops::Add<Output = T>
{
    (((val) % modu) + modu) % modu
}

fn main()
{
    let filename = "src/bin/day06/input.txt";
    // let filename = "src/bin/day06/example_input.txt";

    let mut fishes = parse_input(filename);


    println!("{} fish after 80 days", increase_day_smart(&mut fishes, 80));
    println!("{} fish after 256 days", increase_day_smart(&mut fishes, 256));

}