fn to_range(pair: &str) -> Range {
    let x: Vec<i32> = pair.split("-").map(|x|x.parse().unwrap()).collect();

    return Range { start: x[0], end: x[1] }
}

fn part1(line: &str) -> i32 {
    let ranges: Vec<Range> = line
        .split(",")
        .map(|r| to_range(r))
        .collect();

    let a: &Range = ranges.get(0).unwrap();
    let b: &Range = ranges.get(1).unwrap();

    return if overlaps_completely(a, b) {
        1
    } else {
        0
    }
}

fn part2(line: &str) -> i32 {
    let ranges: Vec<Range> = line
        .split(",")
        .map(|r| to_range(r))
        .collect();

    let a: &Range = ranges.get(0).unwrap();
    let b: &Range = ranges.get(1).unwrap();

    return if overlaps(a, b) {
        1
    } else {
        0
    }
}

#[allow(dead_code)]
fn day4(input: String) -> i32 {
    return input
        .lines()
        .map(|l| part1(l))
        .sum();
}

#[allow(dead_code)]
fn day4_part2(input: String) -> i32 {
    return input
        .lines()
        .map(|l| part2(l))
        .sum();
}

#[derive(Debug)]
struct Range {
    start: i32,
    end: i32
}

fn overlaps_completely(a: &Range, b: &Range) -> bool {
    return (b.start >= a.start && b.end <= a.end) || (a.start >= b.start && a.end <= b.end);
}

fn overlaps(a: &Range, b: &Range) -> bool {
    return (a.start >= b.start && a.start <= b.end) || (b.start >= a.start && b.start <= a.end);
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_overlaps_completely() -> Result<(), std::io::Error> {
        assert_eq!(overlaps_completely(&Range { start: 2, end: 8 }, &Range { start: 3, end: 7}), true);
        assert_eq!(overlaps_completely(&Range { start: 6, end: 6 }, &Range { start: 4, end: 6 }), true);
        assert_eq!(overlaps_completely(&Range { start: 3, end: 4 }, &Range {start: 6, end: 8}), false);

        Ok(())
    }

    #[test]
    fn test_overlaps() -> Result<(), std::io::Error> {
        assert_eq!(overlaps(&Range { start: 2, end: 8 }, &Range { start: 3, end: 7}), true);
        assert_eq!(overlaps(&Range { start: 6, end: 6 }, &Range { start: 4, end: 6 }), true);
        assert_eq!(overlaps(&Range { start: 5, end: 7 }, &Range { start: 7, end: 9 }), true);
        assert_eq!(overlaps(&Range { start: 3, end: 4 }, &Range {start: 6, end: 8}), false);
        assert_eq!(overlaps(&Range { start: 9, end: 10 }, &Range {start: 3, end: 8}), false);

        Ok(())
    }

    #[test]
    fn test_day4() -> Result<(), std::io::Error> {
        assert_eq!(day4(fs::read_to_string("input/day4.txt")?), 2);
        //println!("day4 part 1: {:?}", day4(fs::read_to_string("input/day4_full.txt")?));
        Ok(())
    }

    #[test]
    fn test_day4_part2() -> Result<(), std::io::Error> {
        assert_eq!(day4_part2(fs::read_to_string("input/day4.txt")?), 4);
        //println!("day 2 part 2: {:?}", day4_part2(fs::read_to_string("input/day4_full.txt")?));
        Ok(())
    }
}