// https://adventofcode.com/2023/day/1

use std::collections::HashMap;

fn main() -> Result<(), String> {

    let number_pairs = [
        ("1", 1), ("one",   1),
        ("2", 2), ("two",   2),
        ("3", 3), ("three", 3),
        ("4", 4), ("four",  4),
        ("5", 5), ("five",  5),
        ("6", 6), ("six",   6),
        ("7", 7), ("seven", 7),
        ("8", 8), ("eight", 8),
        ("9", 9), ("nine",  9),
    ];

    let number_map: NumberMap = number_pairs.into_iter().collect();

    // TODO: parallelize these 4 functions to let the quicker ones display their output first
    let sample1 = input_from("sample-1.txt")?;
    let sample2 = input_from("sample-2.txt")?;
    println!("Sample part 1: {}", part1(&sample1, &number_map)); // 142
    println!("Sample part 2: {}", part2(&sample2, &number_map)); // 281

    let input = input_from("input.txt")?;
    println!("Part 1: {}", part1(&input, &number_map));
    println!("Part 2: {}", part2(&input, &number_map));

    Ok(())
}

type NumberMap<'a> = HashMap<&'a str, u32>;

struct Input {
    lines: Vec<String>
}

fn part1(input: &Input, number_map: &NumberMap) -> u32 {

    // for part 1 we only want the digit digits so create a new map without the word digits
    let number_map: NumberMap =
        number_map.iter()
                  .filter(|&(key, _)| key.len() == 1)
                  .map(|(&key, &value)| (key, value))
                  .collect();
    
    // compute and sum the calibration values of each line with the filtered number map
    input.lines.iter()
               .map(|line| get_calibration_value(line, &number_map))
               .sum()
}

fn part2(input: &Input, number_map: &NumberMap) -> u32 {
    // compute and sum the calibration values of each line with the full number map
    input.lines.iter()
               .map(|line| get_calibration_value(line, &number_map))
               .sum()
}

fn get_calibration_value(line: &str, number_map: &NumberMap) -> u32 {
    let mut numbers_found: Vec<u32> = Vec::new();

    // Iterate over each character index in the string
    for (i, _) in line.char_indices() {
        let suffix = &line[i..];
        
        for (number, &value) in number_map.iter() {
            if suffix.starts_with(number) {
                numbers_found.push(value);
                break
            }
        }
    }

    let first = numbers_found.first().unwrap();
    let last  = numbers_found.last().unwrap();
    
    first*10 + last
}

// Read input from a file
fn input_from(file: &str) -> Result<Input, String> {

    let input = std::fs::read_to_string(file)
        .map_err(|err| format!("Failed to read file: {}", err))?;

    Ok(Input {
        lines: input.lines().map(|s| s.to_string()).collect::<Vec<String>>()
    })
}
