// https://adventofcode.com/2023/day/1

use std::str::FromStr;

fn main() -> Result<(), String> {

    let sample = input_from("sample.txt")?;
    println!("Sample part 1: {}", part1(&sample));
    println!("Sample part 2: {}", part2(&sample));

    let input = input_from("input.txt")?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

struct Input {
    lines: Vec<String>
}

fn part1(input: &Input) -> u32 {
    0
}

fn part2(input: &Input) -> u32 {
    0
}


/* Parsing */

// Read input from a file
fn input_from(file: &str) -> Result<Input, String> {

    let input = std::fs::read_to_string(file)
        .map_err(|err| format!("Failed to read file: {}", err))?;

    Ok(Input {
        lines: input.lines().map(|s| s.to_string()).collect::<Vec<String>>()
    })
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let input = input_from("sample.txt").unwrap();
        assert_eq!(0, part1(&input))
    }

    #[test]
    fn test_sample_part2() {
        let input = input_from("sample.txt").unwrap();
        assert_eq!(0, part2(&input))
    }

    #[test]
    fn test_part1() {
        let input = input_from("input.txt").unwrap();
        assert_eq!(0, part1(&input))
    }

    #[test]
    fn test_part2() {
        let input = input_from("input.txt").unwrap();
        assert_eq!(0, part2(&input))
    }
}

