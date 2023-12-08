use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_input() -> Vec<String> {
    let file = File::open("src/input.txt").expect("no such file");
    let buf = BufReader::new(file);

    buf.lines().map(|i| i.unwrap()).collect()
}

pub fn hcf(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}
