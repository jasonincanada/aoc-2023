// https://adventofcode.com/2023/day/9

fn main() -> Result<(), String> {

    let sample = input_from("sample.txt")?;
    println!("Sample part 1: {}", part1(&sample)); // 114
    println!("Sample part 2: {}", part2(&sample)); // 2

    let input = input_from("input.txt")?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

struct Input {
    histories: Vec<History>
}

struct History {
    values: Vec<i32>
}

// a generator is a function that takes a list of values and a recursively-calculated value
// and does something with them to generate a new value
type Generator = fn(&[i32], i32) -> i32;

fn part1(input: &Input) -> i32 {
    // the difference between parts 1/2 is the generator that calculates the new number per line
    // part 1 adds the recursively-calculated value to the last value in the line
    fn generate(values: &[i32], rec: i32) -> i32 {
        rec + values[values.len()-1]
    }

    process_histories(&input.histories, generate)
}

fn part2(input: &Input) -> i32 {
    // part 2 subtracts the recursively-calculated value from the first value in the line
    fn generate(values: &[i32], rec: i32) -> i32 {
        values[0] - rec
    }

    process_histories(&input.histories, generate)
}

fn process_histories(histories: &[History], generate: Generator) -> i32 {
    histories.iter()
             .map(|history| extrapolate(&history.values, generate))
             .sum()
}

fn extrapolate(values: &[i32], generate: Generator) -> i32 {
    if values.len() <= 1 {
        panic!("Need at least 2 elements in the vector for extrapolate() to work");
    }
    
    // the base case, all values in the list are 0
    if values.iter().all(|v| *v == 0) {
        return 0
    }

    let diffs = differences(values);
    let rec = extrapolate(&diffs, generate);    // recursive call

    generate(&values, rec)
}

fn differences(nums: &[i32]) -> Vec<i32> {
    nums.windows(2)
        .map(|window| { window[1] - window[0] })
        .collect()
}


/* Parsing */

use std::str::FromStr;

impl FromStr for History {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Result<Vec<i32>, _> =
            s.split_whitespace()
             .map(|s| s.parse().map_err(|_| "Error parsing i32"))
             .collect();
        
        Ok(History {
            values: values?
        })
    }
}


// Read input from a file
fn input_from(file: &str) -> Result<Input, String> {

    let input = std::fs::read_to_string(file)
        .map_err(|err| format!("Failed to read file: {}", err))?;

    let histories: Result<Vec<_>, _> =
        input.lines()
             .map(|line| line.parse::<History>())
             .collect();

    Ok(Input {
        histories: histories?
    })
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn test_sample_part1() { let input = input_from("sample.txt").unwrap(); assert_eq!(114, part1(&input)) }
    #[test] fn test_sample_part2() { let input = input_from("sample.txt").unwrap(); assert_eq!(2, part2(&input)) }

    #[test] fn test_part1() { let input = input_from("input.txt").unwrap(); assert_eq!(2043183816, part1(&input)) }
    #[test] fn test_part2() { let input = input_from("input.txt").unwrap(); assert_eq!(1118, part2(&input)) }
}
