use regex::Regex;
use std::fs;

struct Solution {
    part1: i32,
    part2: i32,
}

fn solve(input: &str) -> Solution {
    let mut part1 = 0;
    let mut part2 = 0;
    let mut enabled = true;

    let re = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();
    let instructions = re.captures_iter(&input);

    for instruction in instructions {
        match instruction[1].to_string() {
            s if s.starts_with("mul") => {
                let a = instruction[2].parse::<i32>().unwrap();
                let b = instruction[3].parse::<i32>().unwrap();

                part1 += a * b;

                if enabled {
                    part2 += a * b;
                }
            }
            s if s.starts_with("don't") => {
                enabled = false;
            }
            s if s.starts_with("do") => {
                enabled = true;
            }

            _ => {}
        }
    }

    return Solution {
        part1: part1,
        part2: part2,
    };
}

fn main() {
    let input = fs::read_to_string("3.input.txt").expect("Unable to read file");
    let solution = solve(&input);

    println!("Part 1: {}", solution.part1);
    println!("Part 2: {}", solution.part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let solution = solve(input);

        assert_eq!(solution.part1, 161);
    }

    #[test]
    fn test_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(solve(input).part2, 48);
    }
}
