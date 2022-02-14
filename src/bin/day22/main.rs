use std::time::Instant;
use ndarray::Array3;
use std::collections::HashMap;
use regex::Regex;
use std::fs;
use num::abs;

fn parse_input(filename: &str) -> HashMap<(i64, i64, i64), bool>
{
    let mut ret: HashMap<(i64, i64, i64), bool> = HashMap::new();

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let re = Regex::new(r"(\w*) x=(.*?)\.{2}(.*?),y=(.*?)\.{2}(.*?),z=(.*?)\.{2}(.*?)(?:\n|$)").unwrap();

    let mut cnt: usize = 0;
    let mut caps = re.captures_iter(&contents);
    for cap in caps
    {

        let val: bool = cap.get(1).unwrap().as_str() == "on";
        let x_start: i64 = cap.get(2).unwrap().as_str().parse().unwrap();
        let x_end: i64 = cap.get(3).unwrap().as_str().parse().unwrap();
        let y_start: i64 = cap.get(4).unwrap().as_str().parse().unwrap();
        let y_end: i64 = cap.get(5).unwrap().as_str().parse().unwrap();
        let z_start: i64 = cap.get(6).unwrap().as_str().parse().unwrap();
        let z_end: i64 = cap.get(7).unwrap().as_str().parse().unwrap();

        println!("linecount: {}", cnt);
        println!("{}..{}, {}..{}, {}..{}", x_start, x_end, y_start, y_end, z_start, z_end);
        cnt += 1;

        change_cubes(&mut ret,
                    val,
                    x_start,
                    x_end,
                    y_start,
                    y_end,
                    z_start,
                    z_end);
    }

    ret
}

fn change_cubes(cube_map: &mut HashMap<(i64, i64, i64), bool>,
                val: bool,
                x_start: i64,
                x_end: i64,
                y_start: i64,
                y_end: i64,
                z_start: i64,
                z_end: i64)
{

    for x in x_start..x_end+1
    {
        for y in y_start..y_end+1
        {
            for z in z_start..z_end+1
            {
                if cube_map.contains_key(&(x, y, z))
                {
                    *cube_map.get_mut(&(x, y, z)).unwrap() = val;
                }
                else
                {
                    cube_map.insert((x, y, z), val);
                }
            }
        }
    }
}

fn main()
{

    let start = Instant::now();

    let filename = "src/bin/day22/input.txt";
    // let filename = "src/bin/day22/example_input.txt";

    let cube_map = parse_input(filename);

    let pt1: u64 = cube_map.iter().fold(0, |acc, (&x, &y)|{
        if abs(x.0) <= 50 && abs(x.1) <= 50 && abs(x.2) <= 50 && y
        {return acc + 1}
        else
        {return acc}
    });
    println!("Pt1 cubes on: {}", pt1);

    println!("Took: {:?}", Instant::now().duration_since(start));

}