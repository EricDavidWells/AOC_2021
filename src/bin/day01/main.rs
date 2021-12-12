
use libaoc::libaoc;

fn count_increases(inputs: &Vec<i32>) -> i32
{
    let mut count = 0_i32;
    // let mut prev = inputs.get(0);

    for i in 1..inputs.len()
    {
        if inputs.get(i-1) < inputs.get(i)
        {
            count += 1;
        }
    }

    count
}

fn count_3sum_increases(inputs: &Vec<i32>) -> i32
{
    let mut count = 0_i32;

    for i in 3..inputs.len()
    {
        let trailing = inputs.get(i-3);
        let leading = inputs.get(i);

        if leading > trailing
        {
            count += 1;
        }
    }

    count
}

fn main()
{
    let filename = "src/bin/day01/input.txt";
    let inputs: Vec<i32> = libaoc::parse_file::<i32>(&filename);

    let count = count_increases(&inputs);
    println!("part 1: {}", count);

    let count_2 = count_3sum_increases(&inputs);
    println!("part 2: {}", count_2);
}
