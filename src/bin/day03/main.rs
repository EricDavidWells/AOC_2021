use libaoc::libaoc;
use bitvec::prelude::*;

fn convert_string_to_bitset(line: &String) -> BitVec::<Msb0>
{
    let mut ret = BitVec::<Msb0>::with_capacity(line.len());

    for char in line.chars()
    {
        match char
        {
            '0' => ret.push(false),
            '1' => ret.push(true),
            _ => panic!("can only convert 0's or 1's to true/false")
        }
    }

    ret
}

fn convert_strings_to_bitset(lines: &Vec<String>) -> Vec<BitVec::<Msb0>>
{
    let mut ret = Vec::<BitVec::<Msb0>>::new();

    for line in lines.iter()
    {
        ret.push(convert_string_to_bitset(line));
    }

    ret
}

fn find_gamma_and_epsilon(bit_inputs: &Vec<BitVec::<Msb0>>) -> (BitVec::<Msb0>, BitVec::<Msb0>)
{
    let mut gamma= BitVec::<Msb0>::with_capacity(bit_inputs.get(0).unwrap().len());
    let mut epsilon = BitVec::<Msb0>::with_capacity(bit_inputs.get(0).unwrap().len());

    let mut gamma_sum: Vec<u32> = Vec::with_capacity(bit_inputs.get(0).unwrap().len());
    gamma_sum.resize(bit_inputs.get(0).unwrap().len(), 0);

    for bv in bit_inputs
    {
        for i in 0..bv.len()
        {
            gamma_sum[i] += bv[i] as u32;
        }
    }

    for sum in gamma_sum
    {
        gamma.push(sum >= (f32::ceil(bit_inputs.len() as f32/ 2_f32) as u32));
        epsilon.push(sum < (f32::ceil(bit_inputs.len() as f32/ 2_f32) as u32));
    }

    (gamma, epsilon)
}

fn find_oxygen_rating(bit_inputs: &Vec<BitVec::<Msb0>>) -> BitVec::<Msb0>
{
    let mut oxygen_rating= BitVec::<Msb0>::with_capacity(bit_inputs.get(0).unwrap().len());
    let mut bit_inputs_mut: Vec<BitVec::<Msb0>> = bit_inputs.clone();

    for i in (0..bit_inputs[0].len())
    {
        let (gamma, epsilon) = find_gamma_and_epsilon(&bit_inputs_mut);
        bit_inputs_mut.retain(|x| x[i] == gamma[i]);

        // println!("gamma: {}", gamma);
        // log_bitvec(&bit_inputs_mut);

        if bit_inputs_mut.len() == 1 {break;}
    }

    if bit_inputs_mut.len() != 1
    {
        panic!("algorithm failed");
    }

    oxygen_rating = bit_inputs_mut[0].clone();
    oxygen_rating
}

fn find_co2_rating(bit_inputs: &Vec<BitVec::<Msb0>>) -> BitVec::<Msb0>
{
    let mut co2_rating= BitVec::<Msb0>::with_capacity(bit_inputs.get(0).unwrap().len());
    let mut bit_inputs_mut: Vec<BitVec::<Msb0>> = bit_inputs.clone();

    for i in (0..bit_inputs[0].len())
    {
        let (gamma, epsilon) = find_gamma_and_epsilon(&bit_inputs_mut);
        bit_inputs_mut.retain(|x| x[i] == epsilon[i]);

        // println!("gamma: {}", gamma);
        // log_bitvec(&bit_inputs_mut);

        if bit_inputs_mut.len() == 1 {break;}
    }

    if bit_inputs_mut.len() != 1
    {
        panic!("algorithm failed");
    }

    co2_rating = bit_inputs_mut[0].clone();
    co2_rating
}

fn log_bitvec_val(bit_inputs: &Vec<BitVec::<Msb0>>)
{
    for bv in bit_inputs
    {
        println!("{}", bv.load_be::<u32>());
    }
    println!("\n");
}

fn log_bitvec(bit_inputs: &Vec<BitVec::<Msb0>>)
{
    for bv in bit_inputs
    {
        println!("{}", bv);
    }
    println!("\n");
}

fn main()
{
    let filename = "src/bin/day03/input.txt";

    let line_inputs: Vec<String> = libaoc::parse_file::<String>(&filename);
    let bit_inputs = convert_strings_to_bitset(&line_inputs);
    let (gamma, epsilon) = find_gamma_and_epsilon(&bit_inputs);
    println!("gamma: {} ({}), epsilon: {} ({}), power consumption: {}",gamma, gamma.load_be::<u16>(), epsilon, epsilon.load_be::<u16>(), gamma.load_be::<u32>() * epsilon.load_be::<u32>());

    let oxygen_rating = find_oxygen_rating(&bit_inputs);
    println!("oxygen: {} ({})", oxygen_rating, oxygen_rating.load_be::<u16>());
    let co2_rating = find_co2_rating(&bit_inputs);
    println!("co2: {} ({})", co2_rating, co2_rating.load_be::<u16>());
    println!("life support: {}", oxygen_rating.load_be::<u32>() * co2_rating.load_be::<u32>());

}