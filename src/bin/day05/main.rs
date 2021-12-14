use libaoc::libaoc;
use regex::Regex;
use ndarray::Array;

struct Pipe
{
    stt: Pos,
    end: Pos
}

struct Pos
{
    x: u16,
    y: u16
}

fn parse_input(filename: &str) -> (Vec<Pipe>, ndarray::Array2::<u16>)
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

    let mut diagram = ndarray::Array2::<u16>::zeros((max_x as usize, max_y as usize));

    (pipes, diagram)
}

fn main()
{
    // let filename = "src/bin/day05/input.txt";
    let filename = "src/bin/day05/example_input.txt";
    let (mut pipes, mut diagram) = parse_input(filename);


}