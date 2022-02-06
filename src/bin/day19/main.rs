use std::time::Instant;
use std::fs;
use regex::Regex;
use std::collections::HashMap;
use std::ops::Mul;

fn cross_prod<T>(v1: &Vec<T>, v2: &Vec<T>) -> Vec<T>
where T: Copy + std::ops::Mul<Output = T> + std::ops::Sub<Output = T>
{
    let val1: T = v1[1]*v2[2] - v1[2]*v2[1];
    let val2: T = v1[2]*v2[0] - v1[0]*v2[2];
    let val3: T = v1[0]*v2[1] - v1[1]*v2[0];

    let mut ret: Vec<T> = vec![val1, val2, val3];

    ret
}

#[derive(Default, Clone)]
struct Scanner
{
    axis: [i8; 3],
    axis_neg: [i8; 3],
    beacons_rel: Vec<[i64; 3]>,
    beacons_abs: Option<Vec<[i64; 3]>>,
    scanner_abs_pos: Option<[i64; 3]>
}

fn is_z_neg(inds: [i8; 3], negs: [i8; 3]) -> bool
{
    let mut ret: bool = false;

    let mut x: Vec<i8> = vec![0; 3];
    let mut y: Vec<i8> = vec![0; 3];
    let mut z: Vec<i8> = vec![0; 3];

    let xind = inds.iter().position(|&x| x == 0).unwrap() as usize;
    x[xind] = 1 * negs[xind];

    let yind = inds.iter().position(|&x| x == 1).unwrap() as usize;
    y[yind] = 1 * negs[yind];

    let wtf: Vec<i8> = Vec::from(x);
    let wtf2: Vec<i8> = Vec::from(y);

    let z: Vec<i8> = cross_prod(&wtf, &wtf2);

    if z.contains(&-1) {ret = true}

    ret
}

fn ind_map_to_vecs(inds: [i8; 3], negs: [i8; 3]) -> (Vec<i8>, Vec<i8>, Vec<i8>)
{
    let mut x: Vec<i8> = vec![0; 3];
    let mut y: Vec<i8> = vec![0; 3];
    let mut z: Vec<i8> = vec![0; 3];

    let xind = inds.iter().position(|&x| x == 0).unwrap() as usize;
    x[xind] = 1 * negs[xind];

    let yind = inds.iter().position(|&x| x == 1).unwrap() as usize;
    y[yind] = 1 * negs[yind];

    let zind = inds.iter().position(|&x| x == 2).unwrap() as usize;
    z[zind] = 1 * negs[zind];

    (x, y, z)
}

fn get_all_configs() -> (Vec<[i8; 3]>, Vec<[i8; 3]>)
{

    let mut ret_comb: Vec<[i8; 3]> = Vec::new();
    let mut ret_neg: Vec<[i8; 3]> = Vec::new();

    for i in 0..3
    {
        let mut comb: [i8; 3] = [0, 0, 0];
        let mut neg: [i8; 3] = [0, 0, 0];

        for i_n in [-1, 1]
        {
            comb[i] = 0;    // place x
            neg[i] = i_n;

            for j in 0..3
            {
                if j == i {continue;}

                for j_n in [-1, 1]
                {
                    comb[j] = 1;    // place y
                    neg[j] = j_n;

                    for k in 0..3
                    {
                        if k == i || k == j {continue;}

                        comb[k] = 2;
                        neg[k] = if is_z_neg(comb, neg) {-1} else {1};

                        ret_comb.push(comb);
                        ret_neg.push(neg);

                    }
                }
            }
        }
    }

    (ret_comb, ret_neg)
}

fn parse_input(filename: &str) -> HashMap<u8, Scanner>
{

    let mut ret: HashMap<u8, Scanner> = HashMap::new();

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let re1 = Regex::new(r"(?:--- scanner (\d*) ---\n((?:.+[\n])*)\n)").unwrap();
    let re2 = Regex::new(r"(-*\d*),(-*\d*),(-*\d*)\n*").unwrap();

    for cap in re1.captures_iter(&contents)
    {
        if cap.len() != 3 {panic!("wtf is regex doing")}

        let id: u8 = cap.get(1).unwrap().as_str().parse().unwrap();
        let mut scanner = Scanner::default();

        for cap2 in re2.captures_iter(cap.get(2).unwrap().as_str())
        {
            let x: i64 = cap2.get(1).unwrap().as_str().parse().unwrap();
            let y: i64 = cap2.get(2).unwrap().as_str().parse().unwrap();
            let z: i64 = cap2.get(3).unwrap().as_str().parse().unwrap();
            scanner.beacons_rel.push([x, y, z]);
        }
        ret.insert(id, scanner);
    }

    ret
}

fn compare_scanners(scanner1: &Scanner, scanner2: &Scanner) -> Option<[i64; 3]>
{

    let mut ret: Option<[i64; 3]> = None;

    // choose root beacon from scanner 1
    'outer: for i in 0..scanner1.beacons_rel.len()
    {
        // allign scanner 2 beacon onto root beacon
        for j in 0..scanner2.beacons_rel.len()
        {
            // get offset value for moving scanner 2 beacon to scanner 1 beacon
            let mut offset: [i64; 3] = [0, 0, 0];

            for k in 0..3
            {
                offset[k] = scanner2.beacons_rel[j][k] - (scanner1.beacons_rel[i][k]);
            }

            let mut sc2_beacons_shifted = scanner2.beacons_rel.clone();
            let mut count: u64 = 0;

            // shift all beacons
            for beacon in sc2_beacons_shifted.iter_mut()
            {
                for k in 0..3
                {
                    beacon[k] = beacon[k] - offset[k];
                }

                if scanner1.beacons_rel.contains(beacon)
                {
                    count += 1;
                }
            }

            if count >= 12
            {
                ret = Some(offset);
                break 'outer;
            }
        }
    }

    ret
}

fn convert_scanner(scanner: &Scanner, inds: [i8; 3], negs: [i8; 3]) -> Scanner
{
    let mut ret = scanner.clone();

    for i in 0..scanner.beacons_rel.len()
    {
        for j in 0..3
        {
            ret.beacons_rel[i][j] = scanner.beacons_rel[i][inds[j] as usize] * negs[j] as i64;
        }
    }

    ret.axis = inds;
    ret.axis_neg = negs;

    ret
}

fn calc_abs_beacons(scanner: &mut Scanner)
{

    let mut abs_vals: Vec<[i64; 3]> = Vec::new();

    for i in 0..scanner.beacons_rel.len()
    {
        let mut abs_val: [i64; 3] = [0, 0, 0];

        for j in 0..3
        {
            abs_val[j] = scanner.scanner_abs_pos.unwrap()[j] + scanner.beacons_rel[i][j];
        }
        abs_vals.push(abs_val);
    }

    scanner.beacons_abs = Some(abs_vals);
}

fn solve(inputs: &HashMap<u8, Scanner>) -> (Vec<[i64; 3]>, Vec<[i64; 3]>)
{
    let mut ret: Vec<[i64; 3]> = Vec::new();
    let mut ret_scanners: Vec<[i64; 3]> = Vec::new();

    let (configs, negs) = get_all_configs();

    // build outputs
    let mut outputs: HashMap<u8, Scanner> = HashMap::new();
    outputs.insert(0, inputs.get(&0_u8).unwrap().clone());
    outputs.get_mut(&0_u8).unwrap().scanner_abs_pos = Some([0, 0, 0]);
    calc_abs_beacons(outputs.get_mut(&0_u8).unwrap());

    let mut inputs_mut = inputs.clone();
    inputs_mut.remove(&0_u8).unwrap();

    'outer: loop
    {
        println!("scanners remaining {}", inputs_mut.len());
        for (id_abs, scanner_abs) in outputs.iter_mut()
        {
            for key in inputs_mut.clone().keys()
            {
                let id_rel = key;
                let scanner_rel = inputs_mut.get(key).unwrap();

                for (config, negs) in configs.iter().zip(negs.iter())
                {
                    let mut scanner_candidate = convert_scanner(&scanner_rel, *config, *negs);

                    let offset = compare_scanners(&scanner_abs, &scanner_candidate);

                    if offset.is_some()
                    {
                        let mut tmp: [i64; 3] = [0, 0, 0];
                        for i in 0..3
                        {
                            tmp[i] = scanner_abs.scanner_abs_pos.unwrap()[i] - offset.unwrap()[i];
                        }
                        scanner_candidate.scanner_abs_pos = Some(tmp);
                        calc_abs_beacons(&mut scanner_candidate);

                        outputs.insert(*id_rel, scanner_candidate.clone());
                        inputs_mut.remove(id_rel);

                        continue 'outer;
                    }
                }
            }
        }

        if inputs_mut.len() == 0
        {
            break 'outer;
        }
    }

    for (id_abs, scanner_abs) in outputs.iter_mut()
    {
        for beacon in scanner_abs.beacons_abs.as_ref().unwrap()
        {
            if !ret.contains(beacon)
            {
                ret.push(*beacon);
            }

        }
        ret_scanners.push(scanner_abs.scanner_abs_pos.unwrap());
    }

    (ret, ret_scanners)
}

fn find_max_manhattan_distance(outputs: &Vec<[i64; 3]>) -> i64
{
    let mut ret: i64 = 0;

    for i in 0..outputs.len()
    {
        for j in 1..outputs.len()
        {
            let mut dis: i64 = 0;

            for k in 0..3
            {
                dis += num::abs(outputs[i][k] - outputs[j][k]);
            }

            if dis > ret
            {
                ret = dis;
            }
        }
    }

    ret
}

fn main()
{
    let start = Instant::now();

    let filename = "src/bin/day19/input.txt";
    // let filename = "src/bin/day19/example_input.txt";

    let inputs = parse_input(filename);

    let (beacons, scanners) = solve(&inputs);
    let distance = find_max_manhattan_distance(&scanners);

    println!("# of beacons: {}", beacons.len());

    println!("manhattan distance: {}", distance);

    println!("Took: {:?}", Instant::now().duration_since(start));
}