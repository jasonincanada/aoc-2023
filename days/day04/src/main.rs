// https://adventofcode.com/2023/day/4

use std::str::FromStr;
use std::num::ParseIntError;

fn main() -> Result<(), String> {

    let sample = input_from("sample.txt")?;
    println!("Sample part 1: {}", part1(&sample)); // 13
    println!("Sample part 2: {}", part2(&sample)); // 30

    let input = input_from("input.txt")?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

struct Input {
    cards: Vec<Card>
}

struct Card {
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>
}

impl Card {

    // count the number of my_numbers that are found in winning_numbers
    fn count_matches(&self) -> usize {
        self.my_numbers.iter()
                       .filter(|my_number| {
                           self.winning_numbers.contains(my_number)
                       })
                       .count()
    }
}

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
fn part1(input: &Input) -> u32 {
    
    let winning_counts =
        input.cards.iter()
                   .map(|card| card.count_matches())
                   .filter(|count| *count > 0);

    winning_counts.map(|count| 2u32.pow(count as u32 - 1))
                  .sum()
}

fn part2(input: &Input) -> u32 {

    // start out with 1 copy of every scratchcard
    let mut copies: Vec<u32> = vec![ 1; input.cards.len() ];

    for (i, card) in input.cards.iter().enumerate() {
        let matching_count = card.count_matches();
        
        // add copies to the next matching_count piles
        for j in i+1 ..= i+matching_count {
            copies[j] += copies[i]
        }
    }

    copies.iter().sum()
}


/*  Parsing  */

fn input_from(file: &str) -> Result<Input, String> {

    let input = std::fs::read_to_string(file)
        .map_err(|err| format!("Failed to read file: {}", err))?;

    let cards = input.lines()
        .map(|l| l.parse::<Card>().map_err(|e| e.to_string())) // Assuming parse returns a Result<Card, SomeErrorType>
        .collect::<Result<Vec<Card>, _>>()?; // Collect into a Result<Vec<Card>, String>, return early on errors

    Ok(Input {
        cards
    })
}

impl FromStr for Card {
    type Err = String;

    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        // Check and remove the "Card n: " prefix
        let parts = line.splitn(2, ": ").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err("Invalid format: missing 'Card n: '".to_string());
        }
        let numbers_part = parts[1];

        // Split the numbers part at the pipe symbol
        let number_sets: Vec<&str> = numbers_part.split('|').collect();
        if number_sets.len() != 2 {
            return Err("Invalid format: missing '|'".to_string());
        }

        // Parse each set of numbers
        let parse_numbers = |s: &str| -> Result<Vec<u32>, ParseIntError> {
            s.trim().split_whitespace().map(|num| num.parse::<u32>()).collect()
        };

        let winning_numbers = parse_numbers(number_sets[0]).map_err(|e| e.to_string())?;
        let my_numbers      = parse_numbers(number_sets[1]).map_err(|e| e.to_string())?;

        // Construct and return a Card instance
        Ok(Card {
            winning_numbers,
            my_numbers,
        })
    }
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let input = input_from("sample.txt").unwrap();
        assert_eq!(13, part1(&input));
    }

    #[test]
    fn test_sample_part2() {
        let input = input_from("sample.txt").unwrap();
        assert_eq!(30, part2(&input));
    }

    #[test]
    fn test_part1() {
        let input = input_from("input.txt").unwrap();
        assert_eq!(33950, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = input_from("input.txt").unwrap();
        assert_eq!(14814534, part2(&input));
    }

    #[test]
    fn test_valid_card_parsing() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = input.parse::<Card>().unwrap();

        assert_eq!(card.winning_numbers, vec![41, 48, 83, 86, 17]);
        assert_eq!(card.my_numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);
    }

    #[test]
    fn test_invalid_format_missing_pipe() {
        let input = "Card 2: 41 48 83 86 17 83 86  6 31 17  9 48 53";
        let result = input.parse::<Card>();
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_format_no_card_prefix() {
        let input = "41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let result = input.parse::<Card>();
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_number_format() {
        let input = "Card 3: 41 48 83 86 17 | 83 86  6 31 17  9 48 abc";
        let result = input.parse::<Card>();
        assert!(result.is_err());
    }
}
