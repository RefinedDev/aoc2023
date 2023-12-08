use aoc_2023::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Types {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn get_kind(card: &String) -> Types {
    let mut counts: HashMap<char, i32> = HashMap::new();

    for x in card.chars() {
        let count = counts.get(&x).unwrap_or(&0);
        counts.insert(x, count + 1);
    }

    let values = counts.values().map(|f| *f).collect::<Vec<i32>>();

    if values.contains(&5) {
        return Types::FiveOfAKind;
    } else if values.contains(&4) {
        return Types::FourOfAKind;
    } else if values.contains(&3) {
        if counts.len() == 2 {
            return Types::FullHouse;
        } else {
            return Types::ThreeOfAKind;
        }
    } else if values.contains(&2) {
        if counts.len() == 3 {
            return Types::TwoPair;
        } else {
            return Types::OnePair;
        }
    }

    return Types::HighCard;
}

fn rank(kinds: Vec<(&&String, &Types)>) -> Vec<String> {
    let chars = ['A', 'K', 'Q', 'J', 'T'];

    let mut results: Vec<String> = Vec::new();

    for (i, kind) in kinds.iter().enumerate() {
        let mut index = 1;
        loop {
            if let Some(kind2) = kinds.get(i + index) {
                if kind.1 == kind2.1 {
                    let kind1_chars = kind.0.chars().collect::<Vec<char>>();
                    let kind2_chars = kind2.0.chars().collect::<Vec<char>>();
                    let mut char_index = 0;

                    while char_index != 5 {
                        let char = kind1_chars[char_index];
                        let char2 = kind2_chars[char_index];
                        if char.is_alphabetic() && char2.is_alphabetic() {
                            let charindex = chars.iter().position(|&x| x == char).unwrap();
                            let char2index = chars.iter().position(|&x| x == char2).unwrap();

                            if charindex < char2index {
                                results.push(kind.0.to_owned().to_owned());
                                results.push(kind2.0.to_owned().to_owned());
                            } else {
                                results.push(kind2.0.to_owned().to_owned());
                                results.push(kind.0.to_owned().to_owned());
                            }
                            break;
                        } else if char.is_numeric() && char2.is_numeric() {
                            if char > char2 {
                                results.push(kind.0.to_owned().to_owned());
                                results.push(kind2.0.to_owned().to_owned());
                            } else {
                                results.push(kind2.0.to_owned().to_owned());
                                results.push(kind.0.to_owned().to_owned());
                            }
                            break;
                        } else {
                            char_index += 1;
                        }
                    }

                    index += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    for kind in kinds {
        if !results.contains(&kind.0) {
            results.push(kind.0.to_owned().to_owned());
        }
    }

    results.reverse();
    results
}

fn main() {
    let input = get_input();

    let mut hands: Vec<String> = Vec::new();
    let mut bids: HashMap<String, i64> = HashMap::new();

    for i in &input {
        let split: Vec<&str> = i.split_whitespace().collect();
        hands.push(split[0].to_string());
        bids.insert(split[0].to_string(), split[1].parse().unwrap());
    }

    let mut kinds: HashMap<&String, Types> = HashMap::new();

    for card in &hands {
        kinds.insert(card, get_kind(card));
    }

    let mut sorted: Vec<_> = kinds.iter().collect();
    sorted.sort_by_key(|f| f.1);

    let results = rank(sorted);

    let mut sum: i64 = 0;

    for (i,v) in results.iter().enumerate() {
        sum += bids.get(v).unwrap() * (i as i64 +1);
    }

    println!("{}", sum);
    // println!("{:?}", kinds);
}
