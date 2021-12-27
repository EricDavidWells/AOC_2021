pub mod libaoc {

    use std::fmt::Display;
    use std::fs;
    use std::str::FromStr;
    use bitvec::prelude::*;
    use ndarray::{Array2, ArrayView, Axis};

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

}

// mod libaoc;