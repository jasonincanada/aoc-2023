// https://adventofcode.com/2023/day/1

fn main() -> Result<(), String> {

    let sample1 = input_from("sample-1.txt")?;
    let sample2 = input_from("sample-2.txt")?;
    println!("Sample part 1: {}", part1(&sample1)); // 142
    println!("Sample part 2: {}", part2(&sample2)); // 281

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
    fn test_split() {
        let line = "467..114..";
        let splits: Vec<&str> = line.split('.').collect();

        assert_eq!(5, splits.len());
        assert_eq!("467", splits[0]);
        assert_eq!(""   , splits[1]);
        assert_eq!("114", splits[2]);
    }
}

