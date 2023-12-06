// https://adventofcode.com/2023/day/5

use std::str::FromStr;

fn main() -> Result<(), String> {

    let sample = input_from("sample.txt")?;
    println!("Sample part 1: {}", part1(&sample)); // 35
    println!("Sample part 2: {}", part2(&sample)); //

    let input = input_from("input.txt")?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

struct Input {
    seeds: Vec<usize>,
    maps: Vec<MappingStep>
}

// one of the "from-to map" blocks
struct MappingStep {
    ranges: Vec<Range>
}

struct Range {
    dest: usize,
    source: usize,
    size: usize
}

impl MappingStep {
    fn map_number(&self, n: usize) -> usize {
        self.ranges
            .iter()
            .find_map(|range| range.map_number(n))
            .unwrap_or(n)
    }
}

impl Range {
    fn map_number(&self, n: usize) -> Option<usize> {
        if n >= self.source && n < self.source + self.size {
            Some(self.dest + (n - self.source))
        } else {
            None
        }
    }
}

// map each of the top seed numbers through the mapping steps, one at a time
fn part1(input: &Input) -> usize {
    input.seeds.iter()
               .map(|&seed| {
                   input.maps.iter()
                             .fold(seed, |acc, map| map.map_number(acc))
               })
               .min()
               .unwrap()
}

// the seed numbers were actually ranges... about 10^9 now to check
fn part2(input: &Input) -> u32 {
    0
}


/*  Parsing  */

impl FromStr for Range {
    type Err = String;
    
    // 50 98 2
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        if parts.len() != 3 {
            return Err("Range does not contain exactly three integers".to_string())
        }

        let dest   = parts[0].parse().map_err(|_| "First value is not a valid integer".to_string())?;
        let source = parts[1].parse().map_err(|_| "Second value is not a valid integer".to_string())?;
        let size   = parts[2].parse().map_err(|_| "Third value is not a valid integer".to_string())?;

        Ok(Range { dest, source, size })
    }
}

impl FromStr for MappingStep {
    type Err = String;

    /*
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    */
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();

        if lines.len() < 2 {
            return Err("Need at least 2 lines, one for the map title, the rest for ranges".to_string())
        }
        
        let mut ranges = Vec::new();

        for line in lines.iter().skip(1) {
            let range = line.parse::<Range>()
                .map_err(|e| format!("Error parsing range '{}': {}", line, e))?;
            ranges.push(range);
        }

        Ok(MappingStep { ranges })
    }
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments: Vec<&str> = s.split("\n\n").collect();

        if segments.len() < 2 {
            return Err("Expected at least 2 segments separated by a blank line".to_string())
        }

        let mut maps: Vec<MappingStep> = Vec::new();
        for segment in segments[1..].iter() {
            maps.push(segment.parse::<MappingStep>()?)
        }

        Ok(Input {
            seeds: parse_seeds(segments[0])?,
            maps
        })
    }
}

// seeds: 79 14 55 13
fn parse_seeds(s: &str) -> Result<Vec<usize>, String> {
    let seeds = s.strip_prefix("seeds: ")
                 .ok_or("Line doesn't start with 'seeds: '")?;

    seeds.split_whitespace()
         .map(|s| s.parse::<usize>()
                   .map_err(|_| "Expected a positive number".to_string()))
         .collect()
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
        assert_eq!(35, part1(&input))
    }

    #[test]
    fn test_sample_part2() {
        let input = input_from("sample.txt").unwrap();
        assert_eq!(46, part2(&input))
    }

    #[test]
    fn test_part1() {
        let input = input_from("input.txt").unwrap();
        assert_eq!(318728750, part1(&input))
    }

    #[test]
    fn test_part2() {
        let input = input_from("input.txt").unwrap();
        assert_eq!(0, part2(&input))
    }
}
