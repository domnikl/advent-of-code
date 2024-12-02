use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Solution {
    part1: i16,
    part2: i16,
}

fn solve() -> Solution {
    let mut count = 0;

    if let Ok(lines) = read_lines("2.input.txt") {
        for levels in lines.flatten() {
            // split the line by whitespace
            let values: Vec<i16> = levels
                .split_whitespace()
                .map(|x| x.parse::<i16>().unwrap())
                .collect();

            let a = values.clone();
            let b = &values[1..];
            let c = a.iter().zip(b).map(|(a, b)| a - b);

            let consecutive = c.clone().all(|x| x.abs() <= 3);
            let a = c.clone().all(|x| x > 0);
            let b = c.clone().all(|x| x < 0);

            if consecutive && (a || b) {
                count += 1;
            }
        }
    }

    return Solution {
        part1: count,
        part2: 0,
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
