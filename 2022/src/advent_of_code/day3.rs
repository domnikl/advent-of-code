use array_tool::vec::Intersect;
use itertools::Itertools;

// A -> 65 should be 27
// a -> 97 should be 1
fn map_char_to_int(c: char) -> u32 {
    return if c as u32 >= 97 {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}

fn part1(line: &str) -> u32 {
    let x: Vec<u32> = line
        .chars()
        .map(|x| map_char_to_int(x))
        .collect();

    let mut p = x.chunks(x.len() / 2);

    let a: Vec<u32> = p.next().unwrap().to_vec();
    let b: Vec<u32> = p.next().unwrap().to_vec();

    return a.intersect(b)[0];
}

fn part2(lines: Vec<&str>) -> u32 {
    let x: Vec<Vec<u32>> = lines
        .into_iter()
        .map(|l| l.chars().map(|x| map_char_to_int(x)).collect())
        .collect();

    let a = x.get(0).unwrap();
    let b = x.get(1).unwrap();
    let c = x.get(2).unwrap();

    return a.intersect(b.to_owned()).intersect(c.to_owned())[0];
}

#[allow(dead_code)]
fn day3(input: String) -> u32 {
    return input
        .lines()
        .map(|l| part1(l))
        .sum();
}

#[allow(dead_code)]
fn day3_part2(input: String) -> u32 {
    return input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|l| part2(l.collect()))
        .sum();
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_char_int() -> Result<(), std::io::Error> {
        assert_eq!(map_char_to_int('p'), 16);
        assert_eq!(map_char_to_int('L'), 38);
        Ok(())
    }

    #[test]
    fn test_day3() -> Result<(), std::io::Error> {
        assert_eq!(day3(fs::read_to_string("input/day3.txt")?), 157);
        //println!("day3 part 1: {:?}", day3(fs::read_to_string("input/day3_full.txt")?));
        Ok(())
    }

    #[test]
    fn test_day3_part2() -> Result<(), std::io::Error> {
        assert_eq!(day3_part2(fs::read_to_string("input/day3.txt")?), 70);
        //println!("day 2 part 2: {:?}", day3_part2(fs::read_to_string("input/day3_full.txt")?));
        Ok(())
    }
}