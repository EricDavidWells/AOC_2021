
use libaoc::libaoc;

enum command
{
    forward,
    down,
    up
}

fn parse_input(line_inputs: &Vec<String>, commands: &mut Vec<command>, vals: &mut Vec<i8>)
{

    for line in line_inputs
    {
        let mut contents = line.split(" ");

        match contents.next().unwrap()
        {
            "forward" => commands.push(command::forward),
            "down" => commands.push(command::down),
            "up" => commands.push(command::up),
            _ => println!("error")
        }
        vals.push(contents.next().unwrap().parse::<i8>().unwrap());
    }
}

fn get_position(commands: &Vec<command>, vals: &Vec<i8>) -> Vec<i32>
{
    let mut ret: Vec<i32> = Vec::new();

    let mut hpos: i32 = 0;
    let mut dep: i32 = 0;

    for i in 0..vals.len()
    {
        match commands[i]
        {
            command::forward => hpos += i32::from(vals[i]),
            command::down => dep += i32::from(vals[i]),
            command::up => dep -= i32::from(vals[i]),
            _ => println!("error")
        }
    }

    ret.push(hpos);
    ret.push(dep);

    ret
}

fn get_position_2(commands: &Vec<command>, vals: &Vec<i8>) -> Vec<i32>
{
    let mut ret: Vec<i32> = Vec::new();

    let mut hpos: i32 = 0;
    let mut dep: i32 = 0;
    let mut aim: i32 = 0;

    for i in 0..vals.len()
    {
        match commands[i]
        {
            command::forward => {
                hpos += i32::from(vals[i]);
                dep += i32::from(vals[i]) * aim;
            },
            command::down => {
                aim += i32::from(vals[i]);
            },
            command::up => {
                aim -= i32::from(vals[i]);
            },
            _ => println!("error")
        }
    }

    ret.push(hpos);
    ret.push(dep);
    ret.push(aim);

    ret
}


fn main()
{
    let filename = "src/bin/day02/input.txt";
    let line_inputs: Vec<String> = libaoc::parse_file::<String>(&filename);

    let mut commands: Vec<command> = Vec::new();
    let mut vals: Vec<i8> = Vec::new();

    parse_input(&line_inputs, &mut commands, &mut vals);

    let part1 = get_position(&commands, &vals);

    println!("hpos: {0}, dep: {1}, ans: {2}", part1[0], part1[1], part1[0] * part1[1]);

    let part2 = get_position_2(&commands, &vals);

    println!("hpos: {0}, dep: {1}, ans: {2}", part2[0], part2[1], part2[0] * part2[1]);

}