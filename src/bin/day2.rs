use aoc_2023::*;

#[allow(unused_variables)]
fn main() {
    let games = get_input();

    let mut sum_of_id = 0;
    let mut powers: Vec<i32> = Vec::new();

    for (index, block) in games.iter().enumerate() {
        let examples: Vec<&str> = block.split(":").collect();
        let example: Vec<&str> = examples[1].split(";").collect();

        let mut total_red = 0;
        let mut total_blue = 0;
        let mut total_green = 0;

        for i in example {
            let each_color: Vec<&str> = i.split(",").collect();

            for i in each_color {
                if i.contains(" red") {
                    total_red = total_red.max(i.replace(" red", "").trim().parse::<i32>().unwrap());
                     
                } else if i.contains(" blue") {
                    total_blue = total_blue.max(i.replace(" blue", "").trim().parse::<i32>().unwrap());
                    
                } else if i.contains(" green") {
                    total_green = total_green.max(i.replace(" green", "").trim().parse::<i32>().unwrap()); 
                }
            }
        }
        
        let power = total_red * total_blue * total_green;
        powers.push(power);
        
        if total_red <= 12 && total_blue <= 14 && total_green <= 13 {
            sum_of_id += index + 1
        }
    }
    
    // PART 1 ANSWER
    println!("{}", sum_of_id);
     
    // PART 2 ANSWER
    let total_power: i32 = powers.iter().sum();
    println!("{}", total_power);
}