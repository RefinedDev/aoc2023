use std::collections::HashMap;

use aoc_2023::*;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = get_input();

    let mut sum = 0;

    for card in input {
        let mut points = 0;

        let numbs: Vec<&str> = card.split(":").collect();
        let numbers: Vec<&str> = numbs[1].split("|").collect();
        let winning: Vec<&str> = numbers[0].split_whitespace().collect();
        let numbers: Vec<&str> = numbers[1].split_whitespace().collect();

        for number in numbers {
            if winning.contains(&number) {
                if points == 0 {
                    points += 1
                } else {
                    points = 2 * points;
                }
            }
        }

        sum += points;
    }

    println!("{}", sum);
}

fn part2() {
    let input = get_input();

    let mut copies: HashMap<usize, i32> = HashMap::new();

    for (i,_) in input.iter().enumerate() {
        copies.insert(i+1, 0);
    }

    for (i,card) in input.iter().enumerate() {
        let mut matching = 0;

        let numbs: Vec<&str> = card.split(":").collect();
        let numbers: Vec<&str> = numbs[1].split("|").collect();
        let winning: Vec<&str> = numbers[0].split_whitespace().collect();
        let numbers: Vec<&str> = numbers[1].split_whitespace().collect();

        for number in numbers {
            if winning.contains(&number) {
               matching += 1;
            }
        }
        
        for x in 1..=matching {
            let previous_count = copies.get(&(x+i+1)).unwrap();
            let copies_of_that = copies.get(&(i+1)).unwrap();

            copies.insert(x+i+1, previous_count+1+copies_of_that);
        }
    }

    let sum_vec: Vec<i32> = copies.iter().map(|f|  f.1+1).collect();
    let sum: i32 = sum_vec.iter().sum();
    println!("{}", sum);

}