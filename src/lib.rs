pub mod libaoc {

    use std::fmt::Display;
    use std::fs;
    use std::str::FromStr;

    pub fn log_vec<T: Display>(vec: &Vec<T>)
    {
        for val in vec
        {
            println!("{}", val);
        }
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

}

// mod libaoc;