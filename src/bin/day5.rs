use aoc_2023::*;

fn main() {
    let mut input = get_input();

    let seeds: Vec<i32> = input[0]
        .split_at(7)
        .1
        .split_whitespace()
        .map(|f| f.parse::<i32>().unwrap())
        .collect();

    input.remove(0);
    input.retain(|f| !f.is_empty());

    let mut ranges: Vec<&String> = Vec::new();

    for (i, block) in input.iter().enumerate() {
        if block.ends_with(":") {
            let mut index = i + 1;

            while !input[index].ends_with(":") {
                ranges.push(&input[index]);

                if index != input.len() - 1 {
                    index += 1;
                } else {
                    break;
                }
            }
        }
    }

    ranges.retain(|f| !f.is_empty());

    let mut answers: Vec<i32> = Vec::new();

    for numb in seeds {
        for range in &ranges {
            let map = range
                .split_whitespace()
                .map(|f| f.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();


            let a = map[0];
            let b = map[1];
            let c = map[2];

            if numb >= b && numb < b + c {
                answers.push(a + (numb - b));
                break;
            } else {
                answers.push(numb);
            }
        }
    }

    println!("{:?}", answers);
    // println!("{:?}", ranges);
}
