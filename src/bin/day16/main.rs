use std::time::{Duration, Instant};
use std::fs;
use bitvec::prelude::*;

use std::slice::IterMut;
use std::iter::Peekable;
use std::ops::{Add, Mul};
use std::cmp::{min, max, PartialOrd, Ord, PartialEq, Eq};

fn add(x1: u64, x2: u64) -> u64
{
    x1 + x2
}
fn prod(x1: u64, x2: u64) -> u64
{
    x1*x2
}
fn min_(x1: u64, x2: u64) -> u64
{
    min(x1, x2)
}
fn max_(x1: u64, x2: u64) -> u64
{
    max(x1, x2)
}
fn gt(x1: u64, x2: u64) -> u64
{
    if x1 > x2
    {
        return 1
    }
    return 0
}
fn lt(x1: u64, x2: u64) -> u64
{
    if x1 < x2
    {
        return 1
    }
    return 0
}
fn eq(x1: u64, x2: u64) -> u64
{
    if x1 == x2
    {
        return 1
    }
    return 0
}

fn parse_input(filename: &str) -> Vec<bool>
{
    let mut ret: Vec<bool> = Vec::new();

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    for var in contents.bytes()
    {
        let var_hex = hex_ascii_to_int(var);
        let mut bits = int_to_boolvec(var_hex);
        ret.append(&mut bits);
    }

    ret
}

fn hex_ascii_to_int(mut hex: u8) -> u8
{
    if hex > 47 && hex < 58
    {
        return hex - 48
    }
    else if hex > 64 && hex < 71
    {
        return hex - 55
    }
    else
    {
        panic!("not a proper char");
    }
}

fn int_to_boolvec(mut hex: u8) -> Vec<bool>
{
    let mut ret: Vec<bool> = vec![false; 4];

    let mut ind: usize = 0;
    while hex > 0
    {
        ret[ind] = (hex % 2) == 1;
        hex /= 2;
        ind+= 1;
    }

    ret.reverse();
    ret
}

fn boolvec_to_int(mut bvec: &Vec<bool>) -> u64
{
    let mut ret: u64 = 0;
    let mut tmp = bvec.clone();
    tmp.reverse();
    for (i, val) in tmp.iter().enumerate()
    {
        if *val
        {
            ret += (2_u64.pow(i as u32));
        }
    }
    println!("{}", ret);
    ret
}

fn vec_from_iter(it: &mut Peekable<IterMut<bool>>, n: usize) -> Vec<bool>
{
    let mut ret: Vec<bool> = Vec::new();

    for _ in 0..n
    {
        ret.push(*it.next().unwrap());
    }

    ret
}

fn parse_packets(msg_it: &mut Peekable<IterMut<bool>>, bit_count_global: &mut usize, sum: &mut u64) -> u64
{
    let mut ops: Vec<fn(u64, u64)->u64> = Vec::new();
    ops.push(add);
    ops.push(prod);
    ops.push(min_);
    ops.push(max_);
    ops.push(max_);
    ops.push(gt);
    ops.push(lt);
    ops.push(eq);

    let mut ret: u64 = 0;

    // handle base case
    if !msg_it.peek().is_some()
    {
        return 0;
    }

    let version = boolvec_to_int(&mut vec_from_iter(msg_it, 3));
    *sum += version as u64;
    let type_id = boolvec_to_int(&mut vec_from_iter(msg_it, 3));
    *bit_count_global += 6;

    match type_id
    {
        4 => {
            // handle literal
            let mut literal: Vec<bool> = Vec::new();

            while *msg_it.next().unwrap() == true
            {
                literal.append(&mut vec_from_iter(msg_it, 4));
                *bit_count_global += 5;
            }
            literal.append(&mut vec_from_iter(msg_it, 4));
            *bit_count_global += 5;

            let val = boolvec_to_int(&literal);
            ret = val;
        },
        _ => {
            *bit_count_global += 1;
            let mut vals: Vec<u64> = Vec::new();

            if *msg_it.next().unwrap()
            {
                let num_subpackets = boolvec_to_int(&mut vec_from_iter(msg_it, 11));
                *bit_count_global += 11;

                let mut val: u64 = 0;
                for _ in 0..num_subpackets
                {
                    val = parse_packets(msg_it, bit_count_global, sum);
                    vals.push(val);
                }
            } else {
                let subpacket_tot_len = boolvec_to_int(&mut vec_from_iter(msg_it, 15)) as usize;
                *bit_count_global += 15;

                let mut bit_count_start: usize = *bit_count_global;
                while *bit_count_global - bit_count_start != subpacket_tot_len
                {
                    let val = parse_packets(msg_it, bit_count_global, sum);
                    vals.push(val);
                }
            }

            ret = *vals.get(0).unwrap();
            for i in (1..vals.len())
            {
                ret = ops.get(type_id as usize).unwrap()(ret, *vals.get(i).unwrap());
            }
        }
    }

    ret
}

fn main()
{
    let start = Instant::now();

    let filename = "src/bin/day16/input.txt";
    // let filename = "src/bin/day16/example_input.txt";

    let mut msg = parse_input(filename);
    let mut msg_it = msg.iter_mut().peekable();
    let mut sum: u64 = 0;
    let mut bit_count_global: usize = 0;

    let val = parse_packets(&mut msg_it, &mut bit_count_global, &mut sum);
    println!("sum {}", sum);
    println!("val {}", val);
    let end = Instant::now();
    println!("Took: {:?}", end.duration_since(start));
}