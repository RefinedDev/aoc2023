use aoc_2023::*;

fn main() {
    let input = get_input();

    let times = input[0]
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|f| f.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let distances = input[1]
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|f| f.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    // part 1
    let mut i = 0;
    let mut product = 1;

    while i != times.len() {
        let ms = times[i];
        let record = distances[i];
        let mut count = 0;

        for speed in 0..ms {
            let travel = ms - speed;
            let total_distance = travel * speed;
            if total_distance > record {
                count += 1;
            }
        }

        product *= count;
        i += 1;
    }

    println!("{}", product);

    // part2 
    let mut kerned_time_str = String::new();
    let mut kerned_distance_str = String::new();

    for i in times.iter() {
        kerned_time_str.push_str(i.to_string().as_str());
    }

    for i in distances.iter() {
        kerned_distance_str.push_str(i.to_string().as_str());
    }

    let kerned_time = kerned_time_str.parse::<i64>().unwrap();
    let kerned_distance = kerned_distance_str.parse::<i64>().unwrap();
    
    let mut count = 0;
    for speed in 0..kerned_time {
        let travel = kerned_time - speed;
        let total_distance = travel * speed;
        if total_distance > kerned_distance {
            count += 1;
        }
    }    
    
    println!("{}", count);
}

