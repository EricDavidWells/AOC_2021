pub mod libaoc {

    use std::fmt::Display;
    use std::fs;
    use std::str::FromStr;
    use bitvec::prelude::*;
    use ndarray::{Array2, ArrayView, Axis};
    // extern crate num;

    pub fn log_vec<T: Display>(vec: &Vec<T>)
    {
        let mut msg = String::new();
        for val in vec
        {
            msg.push_str(val.to_string().as_str());
        }
        println!("{}", msg);
    }

    pub fn parse_str<T>(string: &str) -> T
        where T: FromStr,  <T as std::str::FromStr>::Err : std::fmt::Debug
    {
        let ret = string.parse::<T>().unwrap();
        ret
    }

    pub fn parse_file<T>(filename: &str) -> Vec<T>
        where T: FromStr,  <T as std::str::FromStr>::Err : std::fmt::Debug
    {

        let mut ret: Vec<T> = Vec::new();
        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
        for substring in contents.split('\n')
        {
            ret.push(parse_str::<T>(&substring));
        }

        ret
    }
    
    pub fn convert_string_to_bitset(line: &String) -> BitVec::<Msb0>
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

    pub fn convert_strings_to_bitset(lines: &Vec<String>) -> Vec<BitVec::<Msb0>>
    {
        let mut ret = Vec::<BitVec::<Msb0>>::new();

        for line in lines.iter()
        {
            ret.push(convert_string_to_bitset(line));
        }

        ret
    }

    fn neg_wrap_mod<T>(mut val: T, modu: T) -> T
        where T: Copy + std::ops::Rem<Output = T> + std::ops::Add<Output = T>
    {
        (((val) % modu) + modu) % modu
    }

    pub fn log_array<T>(arr: &Array2<T>)
        where T: Display
    {
        for row in arr.rows()
        {
            let mut msg: String = String::new();

            for val in row
            {
                msg.push_str(&val.clone().to_string());

                msg.push(',');
            }
            println!("{}", msg);
        }
    }

    pub fn find_n_max<T>(vec: Vec<T>, n: usize) -> Vec<T>
        where T: std::clone::Clone + std::cmp::PartialOrd  + std::cmp::Ord + num::NumCast
    {
        let mut ret: Vec<T> = Vec::new();
        for _ in 0..n{ret.push(num::cast(0).unwrap());}

        if vec.len() < n
        {
            panic!("vector size too small");
        }

        for val in vec
        {
            if val > ret[0]
            {
                ret.remove(0);
                ret.push(val);
                ret.sort();
            }
        }

        ret
    }

    pub fn int_to_boolvec(mut hex: u8) -> Vec<bool>
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

    pub fn boolvec_to_int(mut bvec: &Vec<bool>) -> u64
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
        ret
    }

    pub fn boolvec_from_iter(it: &mut std::slice::Iter<bool>) -> Vec<bool>
    {
        let mut ret: Vec<bool> = Vec::new();

        for val in it
        {
            ret.push(*val);
        }

        ret
    }

}

// mod libaoc;