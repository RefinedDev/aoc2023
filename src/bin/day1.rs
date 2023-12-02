use aoc_2023::*;

pub fn main() {
    let input = get_input();

    let mut numbers: Vec<String> = Vec::new();

    for block in input {
        let block = block // not including last alphabet of each letter because there is an edge case with the input
            .replace("eigh", "8")
            .replace("thre", "3")
            .replace("seve", "7")
            .replace("fou", "4")
            .replace("nin", "9")
            .replace("fiv", "5")
            .replace("si", "6")
            .replace("tw", "2")
            .replace("on", "1");

        let mut first = String::new();
        let mut last = String::new();

        for i in block.chars() {
            if i.is_numeric() {
                first = i.to_string();
            }
        }
        for i in block.chars().rev() {
            if i.is_numeric() {
                last = i.to_string();
            }
        }
        let result = last + first.as_str(); // opposite because rust concatenates from the beggining -_-
        numbers.push(result);
    }

    let result_vec: Vec<i32> = numbers.iter().map(|i| i.parse().unwrap()).collect();
    let sum: i32 = result_vec.iter().sum();
    println!("{}", sum);
}
