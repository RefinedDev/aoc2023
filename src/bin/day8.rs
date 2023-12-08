use aoc_2023::*;
use std::collections::HashMap;

fn main() {
    let mut nodes: HashMap<String, String> = HashMap::new();

    let mut input = get_input();
    let directions: Vec<String> = input[0].chars().map(|f| String::from(f)).collect();

    input.retain(|f| f.contains('('));

    let mut starting_points: Vec<String> = Vec::new();
    let mut answers: Vec<i32> = Vec::new();

    for node in input {
        let split: Vec<String> = node
            .split('=')
            .map(|f| f.trim().replace("(", "").replace(")", ""))
            .collect();
        nodes.insert(split[0].to_string(), split[1].to_string());
        if split[0].ends_with("A") {
            starting_points.push(split[0].to_string());
        }
    }

    let mut steps = 0;
    let mut direction_iter = directions.iter();
    
    for i in starting_points {
        let mut node_point = i;

        while !node_point.ends_with("Z") {
            if let Some(e) = direction_iter.next() {
                let node: Vec<&str> = nodes
                    .get(&node_point)
                    .unwrap()
                    .split(",")
                    .map(|f| f.trim())
                    .collect();
                
                let mut next_node = String::new();
                if e == "R" {
                    next_node.push_str(node[1]);
                } else if e == "L" {
                    next_node.push_str(node[0]);
                }
                node_point = next_node;
    
                steps += 1;
            } else {
                direction_iter = directions.iter();
            }
        }
        answers.push(steps);
        steps = 0;
    }

    let mut lcm = answers.pop().unwrap() as i128;
    for num in answers {
        lcm = lcm * num as i128 / hcf(lcm, num as i128);
    }
    println!("{}", lcm);
}

