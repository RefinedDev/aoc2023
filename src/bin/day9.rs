use aoc_2023::*;
fn main() {
    let input = get_input();

    let mut extrapolated_values_part1: Vec<i32> = Vec::new();
    let mut extrapolated_values_part2: Vec<i32> = Vec::new();


    for history in input {
        let _history = history
            .split_whitespace()
            .map(|f| f.parse().unwrap())
            .collect::<Vec<i32>>();

        let mut differences: Vec<Vec<i32>> = vec![_history.clone()];
        let mut difference = get_differnece(_history);
        differences.push(difference.clone());

        loop {
            let sum: i32 =  difference.iter().sum();
            if sum == 0 {   
                break; 
            }  else {
                difference = get_differnece(difference);
                differences.push(difference.clone());
            }
        }
        
        let e = extrapolate(differences);
        extrapolated_values_part1.push(e.0);
        extrapolated_values_part2.push(e.1);
    }

   let sum1: i32 = extrapolated_values_part1.iter().sum();
   let sum2: i32 = extrapolated_values_part2.iter().sum();

   println!("{} {}", sum1, sum2);
}

fn extrapolate(history: Vec<Vec<i32>>) -> (i32,i32) {
    let mut index = history.len() - 1;

    let a = &history[index];

    // part1
    let mut before_last = a[a.len() - 1] + (a[1] - a[0]);
    let mut answer_part1 = 0;

    //part 2 
    let mut before_beggining = a[0] - (a[1] - a[0]);
    let mut answer_part2 = 0;
    
    index -= 1;

    loop {      
        let b = &history[index];

        answer_part1 = b[b.len() - 1] + before_last;
        before_last = answer_part1;
        
        answer_part2 = b[0] - before_beggining;
        before_beggining = answer_part2;

        if index == 0 { break; }    
        index -= 1;
    }  

    (answer_part1,answer_part2)
}


fn get_differnece(history: Vec<i32>) -> Vec<i32> {
    let mut differences: Vec<i32> = Vec::new(); 
    for (i, v) in history.iter().enumerate() {
        if let Some(x) = history.get(i + 1) {
            differences.push(x - v);
        }
    }

    differences
}
