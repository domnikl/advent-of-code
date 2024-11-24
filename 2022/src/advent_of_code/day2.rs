fn play(shapes: &str) -> i32 {
    return match shapes {
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
        _ => 0
    };
}

fn play_part2(shapes: &str) -> i32 {
     return match shapes {
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
         _ => 0
     };
}

#[allow(dead_code)]
fn day2(input: String) -> i32 {
    return input
        .lines()
        .map(|l| play(l))
        .sum();
}

#[allow(dead_code)]
fn day2_part2(input: String) -> i32 {
    return input
        .lines()
        .map(|l| play_part2(l))
        .sum();
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_day2() -> Result<(), std::io::Error> {
        assert_eq!(day2(fs::read_to_string("input/day2.txt")?), 15);
        //println!("day2 part 1: {:?}", day2(fs::read_to_string("input/day2_full.txt")?));
        Ok(())
    }

    #[test]
    fn test_day2_part2() -> Result<(), std::io::Error> {
        assert_eq!(day2_part2(fs::read_to_string("input/day2.txt")?), 12);
        //println!("day 2 part 2: {:?}", day2_part2(fs::read_to_string("input/day2_full.txt")?));
        Ok(())
    }
}