// https://adventofcode.com/2023/day/6

use std::str::FromStr;

fn main() -> Result<(), String> {

    let sample = input_from("sample.txt")?;
    println!("Sample part 1: {}", part1(&sample)); // 288
    println!("Sample part 2: {}", part2(&sample)); // 71503

    let input = input_from("input.txt")?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

struct Input {
    races: Vec<Race>,      // the input is parsed twice, part2 requires all the numbers 
    squished_race: Race    // to be squished together into one combined time/distance 
}

struct Race {
    time: u64,
    distance: u64
}

fn part1(input: &Input) -> usize {
    let mut ways_to_beat: Vec<usize> = Vec::new();

    for race in &input.races {
        
        // brute force it. a couple O(log n) searches would be more efficient here
        // (i think the delay/distance graph would form an inverted parabola)
        // but we can do this in about a tenth of a second the easy way
        let distances: Vec<u64> =
            (0..race.time)
                .map(|t| find_distance_given_delay(&race, t))
                .collect();

        let count_winners: usize =
            distances.into_iter()
                     .filter(|d| d > &race.distance)
                     .count();

        ways_to_beat.push(count_winners);
    }

    ways_to_beat.into_iter()
                .product()
}

fn part2(input: &Input) -> usize {

    let distances: Vec<u64> =
        (0..input.squished_race.time)
            .map(|t| find_distance_given_delay(&input.squished_race, t))
            .collect();

    distances.into_iter()
             .filter(|d| d > &input.squished_race.distance)
             .count()
}

// chatgpt 4.0
fn find_distance_given_delay(race: &Race, hold_delay: u64) -> u64 {
    // Check if the hold delay is equal to or greater than the race time
    if hold_delay >= race.time {
        0
    } else {
        // Calculate the travel time
        let travel_time = race.time - hold_delay;

        // Calculate the distance
        hold_delay * travel_time
    }
}


/* Parsing */

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();

        if lines.len() != 2 {
            return Err("Expected two lines in input".to_string())
        }

        // first parse the individual races, do the combined one after
        let times     = line_to_ints(lines[0])?;
        let distances = line_to_ints(lines[1])?;

        let mut races: Vec<Race> = Vec::new();

        for (i, &time) in times.iter().enumerate() {
            races.push(Race {
                time,
                distance: distances[i]
            })
        }

        // parse the combined race by removing the whitespace between numbers
        let squished_time     = line_to_squished_int(lines[0]);
        let squished_distance = line_to_squished_int(lines[1]);

        Ok(Input {
            races,
            squished_race: Race {
                time: squished_time?,
                distance: squished_distance?
            }}
        )
    }
}

fn line_to_ints(line: &str) -> Result<Vec<u64>, String> {
    let split: Vec<&str> = line.split_whitespace().collect();

    let ints: Result<Vec<u64>, _> =
        split[1..].iter()
                  .map(|s| s.parse::<u64>().map_err(|_| "Couldn't parse u64"))
                  .collect();

    let ints: Vec<u64> = ints?;

    Ok(ints)
}

fn line_to_squished_int(line: &str) -> Result<u64, String> {
    let combined = line.chars()
                       .filter(|&c| c.is_ascii_digit())
                       .collect::<String>();

    combined.parse::<u64>()
            .map_err(|e| format!("Error parsing combined integer: {}", e.to_string()))
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
        assert_eq!(288, part1(&input))
    }

    #[test]
    fn test_sample_part2() {
        let input = input_from("sample.txt").unwrap();
        assert_eq!(71503, part2(&input))
    }

    #[test]
    fn test_part1() {
        let input = input_from("input.txt").unwrap();
        assert_eq!(131376, part1(&input))
    }

    #[test]
    fn test_part2() {
        let input = input_from("input.txt").unwrap();
        assert_eq!(34123437, part2(&input))
    }
}

/*
    $ time target/release/day06.exe 
    Sample part 1: 288
    Sample part 2: 71503
    Part 1: 131376
    Part 2: 34123437

    real    0m0.166s
    user    0m0.015s
    sys     0m0.000s
*/
