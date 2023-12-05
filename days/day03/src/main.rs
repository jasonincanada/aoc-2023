// https://adventofcode.com/2023/day/3

use std::str::FromStr;
use regex::Regex;

fn main() -> Result<(), String> {

    let sample = input_from("sample.txt")?;
    println!("Sample part 1: {}", part1(&sample)); // 4361
    println!("Sample part 2: {}", part2(&sample)); // 467835

    let input = input_from("input.txt")?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

struct Input {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

struct Number {
    position: Position,
    value: u32,
    length: usize
}

struct Symbol {
    position: Position,
    symbol: char
}

#[derive(PartialEq)]
struct Position {
    row: usize,
    col: usize
}

fn part1(input: &Input) -> u32 {

    // first gather the coordinates of all the symbols
    let symbols_at: Vec<&Position> =
        input.symbols.iter()
                     .map(|symbol| &symbol.position)
                     .collect();

    // find the numbers that have a symbol somewhere in their neighbourhood
    let part_numbers: Vec<&Number> =
        input.numbers.iter()
                     .filter(|&number| {
                        let nd = get_neighbourhood_of_number(number);

                        // look for intersections of the neighbourhood and symbol positions
                        return nd.iter().any(|pos| symbols_at.contains(&pos))
                     })
                     .collect();

    part_numbers.iter()
                .map(|pn| pn.value)
                .sum()
}

fn part2(input: &Input) -> u32 {
    
    let gears =
        input.symbols.iter()
                     .filter(|symbol| symbol.symbol == '*')
                     .map(|symbol| get_numbers_around_position(&symbol.position, &input.numbers))
                     .filter(|numbers| numbers.len() == 2);
    
    // calculate the sum of the gear ratios (actually products)
    gears.map(|pair| pair[0].value * pair[1].value)
         .sum()
}

fn get_numbers_around_position<'a>(position: &Position, numbers: &'a [Number]) -> Vec<&'a Number> {
    numbers.iter()
           .filter(|&number| {
                // look for intersections of this number's neighbourhood and the symbol's position
                get_neighbourhood_of_number(number)
                    .iter()
                    .any(|pos| pos == position)
           })
           .collect()
}

// chatgpt 4.0
fn get_neighbourhood_of_number(number: &Number) -> Vec<Position> {
    let start_row = if number.position.row > 0 { number.position.row - 1 } else { 0 };
    let end_row = number.position.row + 1; // chatgpt wrote 2 for this

    let start_col = if number.position.col > 0 { number.position.col - 1 } else { 0 };
    let end_col = number.position.col + number.length;

    (start_row..=end_row)
        .flat_map(|row| {
            (start_col..=end_col).map(move |col| Position { row, col })
        })
        .collect()
}


/*  Parsing  */

impl FromStr for Input {
    type Err = String;

    /*
        467..114..
        ...*......
        ..35..633.
        ......#...
    */
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers: Vec<Number> = Vec::new();
        let mut symbols: Vec<Symbol> = Vec::new();
        
        for (row, line) in s.lines().enumerate() {
            let (mut nums, mut syms) = line_to_gridthings(line, row);
            numbers.append(&mut nums);
            symbols.append(&mut syms);
        }

        Ok(Input { 
            numbers,
            symbols
        })
    }
}

fn line_to_gridthings(line: &str, row: usize) -> (Vec<Number>, Vec<Symbol>) {
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    // match a string of digits or any of these cool symbols
    let re = Regex::new(r"(\d+|[#$&%*+/=@-])").unwrap();

    for mat in re.find_iter(line) {
        let value = &line[mat.start()..mat.end()]; // Extract the matched string
        let col = mat.start(); // Get the offset

        // if it starts with a digit it's a number
        if value.chars().next().unwrap().is_ascii_digit() {
            numbers.push(Number {
                position: Position { row, col },
                value: value.parse().unwrap(),
                length: value.len()
            })
        } 
        // otherwise it's a symbol
        else
        {
            symbols.push(Symbol {
                position: Position { row, col },
                symbol: value.chars().next().unwrap()
            })
        }
    }

    (numbers, symbols)
}

// Read input from a file
fn input_from(file: &str) -> Result<Input, String> {

    let input = std::fs::read_to_string(file)
        .map_err(|err| format!("Failed to read file: {}", err))?;

    input.parse::<Input>()
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let input = input_from("sample.txt").unwrap();
        assert_eq!(4361, part1(&input))
    }

    #[test]
    fn test_sample_part2() {
        let input = input_from("sample.txt").unwrap();
        assert_eq!(467835, part2(&input))
    }

    #[test]
    fn test_part1() {
        let input = input_from("input.txt").unwrap();
        assert_eq!(554003, part1(&input))
    }

    #[test]
    fn test_part2() {
        let input = input_from("input.txt").unwrap();
        assert_eq!(87263515, part2(&input))
    }
}
