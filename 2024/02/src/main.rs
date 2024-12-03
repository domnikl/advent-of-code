use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Solution {
    part1: i16,
    part2: i16,
}

fn level_is_valid(numbers: Vec<i16>) -> bool {
    let a = numbers.clone();
    let b = &numbers[1..];
    let c = a.iter().zip(b).map(|(a, b)| a - b);

    let consecutive = c.clone().all(|x| x.abs() <= 3);
    let a = c.clone().all(|x| x > 0);
    let b = c.clone().all(|x| x < 0);

    return consecutive && (a || b);
}

fn level_is_valid_with_removals(numbers: Vec<i16>) -> bool {
    if level_is_valid(numbers.clone()) {
        return true;
    }

    for i in 0..numbers.len() {
        let mut values = numbers.clone();
        values.remove(i);

        if level_is_valid(values.clone()) {
            return true;
        }
    }

    return false;
}

fn solve() -> Solution {
    let mut part1 = 0;
    let mut part2 = 0;

    if let Ok(lines) = read_lines("2.input.txt") {
        for levels in lines.flatten() {
            // split the line by whitespace
            let values: Vec<i16> = levels
                .split_whitespace()
                .map(|x| x.parse::<i16>().unwrap())
                .collect();

            if level_is_valid(values.clone()) {
                part1 += 1;
            } else if level_is_valid_with_removals(values.clone()) {
                // remove elements until it becomes valid
                part2 += 1;
            }
        }
    }

    return Solution {
        part1: part1,
        part2: part1 + part2,
    };
}

fn main() {
    let solution = solve();
    println!("Part 1: {}", solution.part1);
    println!("Part 2: {}", solution.part2);
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_is_valid() {
        assert_eq!(level_is_valid(vec![7, 6, 4, 2, 1]), true);
        assert_eq!(level_is_valid(vec![1, 3, 6, 7, 9]), true);
    }

    #[test]
    fn test_level_is_valid_with_removals() {
        assert_eq!(level_is_valid_with_removals(vec![1, 2, 7, 8, 9]), false);
        assert_eq!(level_is_valid_with_removals(vec![9, 7, 6, 2, 1]), false);

        assert_eq!(level_is_valid_with_removals(vec![1, 3, 2, 4, 5]), true);
        assert_eq!(level_is_valid_with_removals(vec![8, 6, 4, 4, 1]), true);
    }
}
