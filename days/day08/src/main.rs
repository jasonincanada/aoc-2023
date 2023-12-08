// https://adventofcode.com/2023/day/8

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

fn main() -> Result<(), String> {

    let sample1 = input_from("sample-1.txt")?;
    let sample2 = input_from("sample-2.txt")?;
    println!("Sample part 1: {}", part1(&sample1));
    println!("Sample part 2: {}", part2(&sample2));

    let input = input_from("input.txt")?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

struct Input {
    directions: Vec<Direction>,
    network: Network
}

enum Direction { Left, Right }
type Network = HashMap<Label, Node>;
type Label = String;

struct Node {
    name: Label,
    left: Label,
    right: Label
}

fn part1(input: &Input) -> u32 {
    let mut counter = 1;
    let mut position: &str = "AAA";

    for direction in input.directions.iter().cycle() {
        position = match direction {
            Direction::Left  => &input.network.get(position).unwrap().left,
            Direction::Right => &input.network.get(position).unwrap().right
        };
        
        if position == "ZZZ" { break }
        counter += 1;
    }

    counter
}

// doesn't finish in time, need to find a shortcut
fn part2(input: &Input) -> u32 {
    let mut counter = 1;
    let mut positions: Vec<&str> =
        input.network.keys()
                     .filter(|name| name.ends_with('A'))
                     .map(|name| name.as_str())
                     .collect();

    for direction in input.directions.iter().cycle() {
        for p in &mut positions {
            *p = match direction {
                Direction::Left  => &input.network.get(*p).unwrap().left,
                Direction::Right => &input.network.get(*p).unwrap().right
            }
        }
        if positions.iter().all(|name| name.ends_with('Z')) {
            break
        }
        counter += 1;
    }

    counter
}


/* Parsing */

lazy_static! {
    static ref RE_NODE: Regex = Regex::new(r"^(\w+) = \((\w+), (\w+)\)$").unwrap();
}

impl FromStr for Node {
    type Err = String;

    // AAA = (BBB, CCC)
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(caps) = RE_NODE.captures(s) {
            let aaa = caps.get(1).ok_or("No AAA found")?.as_str().to_string();
            let bbb = caps.get(2).ok_or("No BBB found")?.as_str().to_string();
            let ccc = caps.get(3).ok_or("No CCC found")?.as_str().to_string();

            Ok(Node {
                name : aaa,
                left : bbb,
                right: ccc
            })
        } else {
            Err("Invalid format".into())
        }
    }
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();

        if lines.len() < 3 {
            return Err("Expected at least three lines in the input".into())
        }

        let mut nodes = Vec::new();
        for line in lines[2..].iter() {
            // Parse the line into a Node, returning early if there's an error
            let node = line.parse::<Node>()?;
            nodes.push(node);
        }
                          
        Ok(Input {
            directions: parse_instructions(lines[0])?,
            network: network_from_nodes(nodes)
        })
    }
}

fn parse_instructions(line: &str) -> Result<Vec<Direction>, String> {
    let mut directions: Vec<Direction> = Vec::new();

    for char in line.chars() {
        match char {
            'L' => directions.push(Direction::Left),
            'R' => directions.push(Direction::Right),
             _  => return Err("Unknown char".into())
        }
    }

    Ok(directions)
}

fn network_from_nodes(nodes: Vec<Node>) -> Network {
    let mut network = Network::new();
    for node in nodes {
        network.insert(node.name.clone(), node);
    }
    network
}

// Read input from a file
fn input_from(file: &str) -> Result<Input, String> {

    let input = std::fs::read_to_string(file)
        .map_err(|err| format!("Failed to read file: {}", err))?;

    input.parse()
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let input = input_from("sample-1.txt").unwrap();
        assert_eq!(2, part1(&input));
        let input = input_from("sample-2.txt").unwrap();
        assert_eq!(6, part1(&input));
    }

    #[test]
    fn test_sample_part2() {
        let input = input_from("sample-3.txt").unwrap();
        assert_eq!(6, part2(&input))
    }

    #[test]
    fn test_part1() {
        let input = input_from("input.txt").unwrap();
        assert_eq!(20569, part1(&input))
    }

    /*
    #[test]
    fn test_part2() {
        let input = input_from("input.txt").unwrap();
        assert_eq!(0, part2(&input))
    }
    */

    #[test]
    fn test_parse_node() {
        let node = "AAA = (BBB, CCC)".parse::<Node>().unwrap();
        assert_eq!("AAA", node.name);
        assert_eq!("BBB", node.left);
        assert_eq!("CCC", node.right);
    }
}
