use itertools::Itertools;

fn top3_calories(input: String) -> Vec<u32> {
    return input.split("\n\n")
        .map(|x| x.lines().map(|l| l.parse::<u32>().unwrap()).sum())
        .sorted()
        .rev()
        .take(3)
        .collect()
}

#[allow(dead_code)]
fn day1(input: String) -> u32 {
    return top3_calories(input)[0];
}

#[allow(dead_code)]
fn day1_part2(input: String) -> u32 {
    return top3_calories(input)[0..3]
        .iter()
        .sum();
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_day1() -> Result<(), std::io::Error> {
        assert_eq!(day1(fs::read_to_string("input/day1.txt")?), 24000);
        //println!("{:?}", day1(fs::read_to_string("input/day1_full.txt")?));

        Ok(())
    }

    #[test]
    fn test_day1_part2() -> Result<(), std::io::Error> {
        assert_eq!(day1_part2(fs::read_to_string("input/day1.txt")?), 45000);
        //println!("{:?}", day1_part2(fs::read_to_string("input/day1_full.txt")?));
        Ok(())
    }
}