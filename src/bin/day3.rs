use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    part_1()?;
    part_2()?;
    Ok(())
}

enum Direction {
    Lt,
    Rt,
    Up,
    Dn,
    At,
}

macro_rules! is_digit {
    ($b:expr) => {
        $b.is_ascii_digit()
    };
}

fn part_1() -> Result<(), Box<dyn Error>> {
    use Direction::*;
    let matrix = get_matrix()?;

    let (m, n) = (matrix.len(), matrix[0].len());

    let mut total = 0;
    let mut seen = HashSet::new();

    for i in 0..m {
        for j in 0..n {
            if is_symbol(matrix[i][j]) {
                for (dir_i, dir_j) in [
                    (Up, Lt),
                    (Up, At),
                    (Up, Rt),
                    (At, Lt),
                    (At, At),
                    (At, Rt),
                    (Dn, Lt),
                    (Dn, At),
                    (Dn, Rt),
                ] {
                    let opt_i = checked_delta(dir_i, i, m);
                    let opt_j = checked_delta(dir_j, j, n);

                    if let (Some(i2), Some(j2)) = (opt_i, opt_j) {
                        if is_digit!(matrix[i2][j2]) {
                            let (num, k, l) = grab_number(&matrix[i2], j2);

                            if seen.insert((i2, k, l)) {
                                total += num;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("Part 1 Total: {}", total);
    Ok(())
}

fn part_2() -> Result<(), Box<dyn Error>> {
    use Direction::*;
    let matrix = get_matrix()?;

    let (m, n) = (matrix.len(), matrix[0].len());

    let mut total = 0;
    let mut seen = HashSet::new();
    let mut nums = Vec::new();

    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] == b'*' {
                nums.clear();
                for (dir_i, dir_j) in [
                    (Up, Lt),
                    (Up, At),
                    (Up, Rt),
                    (At, Lt),
                    (At, At),
                    (At, Rt),
                    (Dn, Lt),
                    (Dn, At),
                    (Dn, Rt),
                ] {
                    let opt_i = checked_delta(dir_i, i, m);
                    let opt_j = checked_delta(dir_j, j, n);

                    if let (Some(i2), Some(j2)) = (opt_i, opt_j) {
                        if is_digit!(matrix[i2][j2]) {
                            let (num, k, l) = grab_number(&matrix[i2], j2);

                            if seen.insert((i2, k, l)) {
                                nums.push(num);
                            }
                        }
                    }
                }
                if nums.len() == 2 {
                    total += nums[0] * nums[1];
                }
            }
        }
    }
    println!("Part 2 Total: {}", total);
    Ok(())
}

fn get_matrix() -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);
    let matrix = reader
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|s| s.bytes().collect())
        .collect::<Vec<Vec<_>>>();
    Ok(matrix)
}

fn checked_delta(dir: Direction, i: usize, n: usize) -> Option<usize> {
    use Direction::*;
    match dir {
        Lt | Up => i.checked_sub(1),
        Rt | Dn => {
            if i < n - 1 {
                Some(i + 1)
            } else {
                None
            }
        }
        _ => Some(i),
    }
}

fn is_symbol(b: u8) -> bool {
    b != b'.' && b < b'0' || b > b'9'
}

fn grab_number(ba: &[u8], i: usize) -> (i32, usize, usize) {
    use std::str::from_utf8;
    let mut j = i;
    while j > 0 && is_digit!(ba[j - 1]) {
        j -= 1;
    }
    let mut k = i;
    while k < ba.len() && is_digit!(ba[k]) {
        k += 1;
    }
    let num = from_utf8(&ba[j..k]).unwrap().parse::<i32>().unwrap();
    (num, j, k)
}
