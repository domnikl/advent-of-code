use std::fs;

struct Solution {
    part1: i32,
    part2: i32,
}

struct Field {
    lines: Vec<String>,
}

impl Field {
    fn new(input: &str) -> Field {
        Field {
            lines: input.split("\n").map(|x| x.to_string()).collect(),
        }
    }

    fn size(&self) -> usize {
        return self.lines.clone().len();
    }

    fn has_word(&self, x: usize, y: usize, dx: i32, dy: i32, search: &[&str]) -> bool {
        let mut word = String::new();
        let size = search[0].len();

        for i in 0..size {
            let x: i32 = x as i32 + dx * i as i32;
            let y: i32 = y as i32 + dy * i as i32;

            if x < 0 || y < 0 {
                return false;
            }

            word.push(self.get_char(x.try_into().unwrap(), y.try_into().unwrap()));
        }

        return search.contains(&word.as_str());
    }

    fn get_char(&self, x: usize, y: usize) -> char {
        if y >= self.size() || x >= self.size() {
            return ' ';
        }

        return self.lines.clone().get(y).unwrap().chars().nth(x).unwrap();
    }

    fn count_xmas(&self, x: usize, y: usize) -> i32 {
        let mut count = 0;
        let search = ["XMAS", "SAMX"];

        if self.get_char(x, y) != 'X' && self.get_char(x, y) != 'S' {
            return 0;
        }

        // check if the word is horizontal
        if self.has_word(x, y, 1, 0, &search) {
            count += 1;
        }

        // check if the word is vertical
        if self.has_word(x, y, 0, 1, &search) {
            count += 1;
        }

        // check if the word is diagonal
        if self.has_word(x, y, 1, 1, &search) {
            count += 1;
        }

        // check if the word is diagonal backwards
        if self.has_word(x, y, -1, 1, &search) {
            count += 1;
        }

        return count;
    }

    fn count_x_mas(&self, x: usize, y: usize) -> i32 {
        let mut count = 0;
        let search = ["MAS", "SAM"];

        if self.get_char(x, y) != 'M' && self.get_char(x, y) != 'S' {
            return 0;
        }

        if self.has_word(x, y, 1, 1, &search) && self.has_word(x + 2, y, -1, 1, &search) {
            count += 1;
        }

        return count;
    }
}

fn solve(input: &str) -> Solution {
    let mut part1 = 0;
    let mut part2 = 0;

    let field = Field::new(input);

    for y in 0..field.size() {
        for x in 0..field.size() {
            part1 += field.count_xmas(x, y);
            part2 += field.count_x_mas(x, y);
        }
    }

    return Solution {
        part1: part1,
        part2: part2,
    };
}

fn main() {
    let input = fs::read_to_string("4.input.txt").expect("Unable to read file");
    let solution = solve(&input);

    println!("Part 1: {}", solution.part1);
    println!("Part 2: {}", solution.part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let solution = solve(input);

        assert_eq!(solution.part1, 18);
    }

    #[test]
    fn test_part2() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(solve(input).part2, 9);
    }
}
