// https://adventofcode.com/2023/day/2

use std::str::FromStr;

fn main() -> Result<(), String> {

    // TODO: parallelize these 4 functions to let the quicker ones display their output first
    let sample = input_from("sample.txt")?;
    println!("Sample part 1: {}", part1(&sample));
    println!("Sample part 2: {}", part2(&sample));

    let input = input_from("input.txt")?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

struct Input {
    games: Vec<Game>
}

struct Game {
    game_id: u32,
    handfuls: Vec<Handful>,
}

#[derive(Default)]   // derive Default because we'll be using a mutable Handful instance
struct Handful {     // in part 2 and we need to start off with zeroes
    red: u32,
    green: u32,
    blue: u32,
}

fn part1(input: &Input) -> u32 {
    
    // maximums defined in the problem description
    const CUBES_RED  : u32 = 12;
    const CUBES_GREEN: u32 = 13;
    const CUBES_BLUE : u32 = 14;

    let game_ids =
        input.games.iter()
                   .filter(|game| {
                        // keep only the games whose handfuls are all less than the maximums
                        game.handfuls.iter().all(
                            |handful| {
                                   handful.red <= CUBES_RED
                                && handful.green <= CUBES_GREEN
                                && handful.blue <= CUBES_BLUE
                            })
                   })
                   .map(|game| game.game_id);

    game_ids.sum::<u32>()
}

fn part2(input: &Input) -> u32 {
    let mut powers: Vec<u32> = Vec::new();

    for game in &input.games {
        let mut handful = Handful::default();
        for h in game.handfuls.iter() {
            handful.red   = handful.red.max(h.red);
            handful.green = handful.green.max(h.green);
            handful.blue  = handful.blue.max(h.blue);
        }
        powers.push(handful.red * handful.green * handful.blue)
    }
    powers.iter().sum::<u32>()
}


/*  Parsing  */

// proper parsing this year with respect to error handling. chatgpt 4.0 wrote most of this
impl std::str::FromStr for Handful {
    type Err = String;

    // "3 blue, 4 red, 2 green"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for pair in s.split(',') {
            let parts: Vec<&str> = pair.trim().split_whitespace().collect();
            if parts.len() != 2 {
                return Err(format!("Invalid format: {}", pair));
            }
            let count: u32 = parts[0].parse().map_err(|_| format!("Invalid number: {}", parts[0]))?;
            match parts[1] {
                "red" => red += count,
                "green" => green += count,
                "blue" => blue += count,
                _ => return Err(format!("Invalid color: {}", parts[1])),
            }
        }

        Ok(Handful { red, green, blue })
    }
}

impl FromStr for Game {
    type Err = String;

    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err("Invalid game line format".to_string());
        }

        let label = parts[0].trim();
        let data = parts[1].trim();

        // Extracting the numeric part from the label, assuming the format "Game <number>"
        let game_number = label.split_whitespace().last()
            .ok_or_else(|| "No game number found in label".to_string())?
            .parse::<u32>()
            .map_err(|_| "Invalid game number format".to_string())?;

        let mut handfuls = Vec::new();
        for segment in data.split(';') {
            let handful = segment.trim().parse::<Handful>()
                .map_err(|e| format!("Error parsing handful: {}", e))?;
            handfuls.push(handful);
        }

        Ok(Game {
            game_id: game_number,
            handfuls
        })
    }
}

// Read input from a file
fn input_from(file: &str) -> Result<Input, String> {

    let input = std::fs::read_to_string(file)
        .map_err(|err| format!("Failed to read file: {}", err))?;

    let games: Result<Vec<Game>, String> =
        input.lines()
             .map(|line| line.parse::<Game>()
                             .map_err(|err| format!("Failed to parse line: {}\nError: {}", line, err)))
             .collect();

     let games = games?;

    Ok(Input { games })
}
