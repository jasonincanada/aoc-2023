// https://adventofcode.com/2023/day/10

use std::ops::Index;

fn main() -> Result<(), String> {

    let sample = input_from("sample-1.txt")?;
    println!("Sample part 1: {}", part1(&sample)); // 8
    println!("Sample part 2: {}", part2(&sample)); // 4

    let input = input_from("input.txt")?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

struct Input {
    grid: Grid<char>,
    s_position: Position
}

// a 2D-grid of elements, all of the same type T
struct Grid<T> {
    elements: Vec<Vec<T>>
}

impl<T> Grid<T> {
    // assume all rows are the same length
    fn width(&self)  -> usize { self.elements[0].len() }
    fn height(&self) -> usize { self.elements.len() }

    // get the available directions from this position, considering only the grid boundaries and not the
    // underlying pipes (the <T> on Grid is generic right now so we don't yet have chars to inspect)
    fn directions_from(&self, position: &Position) -> Vec<Direction> {
        let mut directions = Vec::new();

        if position.row > 0                 { directions.push(Direction::Up) }
        if position.row < self.height() - 1 { directions.push(Direction::Down) }
        if position.col > 0                 { directions.push(Direction::Left) }
        if position.col < self.width() - 1  { directions.push(Direction::Right) }

        directions
    }
}

#[derive(Clone, PartialEq)]
struct Position {
    row: usize,
    col: usize
}

// make Grid indexable with a Position
impl<T> Index<&Position> for Grid<T> {
    type Output = T;

    fn index(&self, pos: &Position) -> &Self::Output {
        if pos.row >= self.elements.len()    { panic!("Index outside of row bounds") }
        if pos.col >= self.elements[0].len() { panic!("Index outside of col bounds") }
        
        &self.elements[pos.row][pos.col]
    }
}

impl Grid<char>
{
    // which of the possible directions from this position are valid ones, ie the pipe
    // in that direction is lined up properly for us to get to it from this position's pipe
    fn get_valid_directions(&self, p: &Position) -> Vec<Direction> {
        let this_char = &self[p];

        self.directions_from(p)
            .into_iter()
            .filter(|dir| {
                let next_char = self[ &position_in_direction(p, dir) ];

                // contains: [T] -> T -> bool
                match dir {
                    Direction::Right => "J-7".contains(next_char) && "SL-F".contains(*this_char),
                    Direction::Down  => "L|J".contains(next_char) && "SF|7".contains(*this_char),
                    Direction::Left  => "F-L".contains(next_char) && "SJ-7".contains(*this_char),
                    Direction::Up    => "F|7".contains(next_char) && "SL|J".contains(*this_char),
                }
            })
            .collect()
    }
}

fn position_in_direction(p: &Position, direction: &Direction) -> Position {
    use Direction::*;

    match direction {
        Down  => Position { row: p.row + 1, col: p.col     },
        Up    => Position { row: p.row - 1, col: p.col     },
        Right => Position { row: p.row    , col: p.col + 1 },
        Left  => Position { row: p.row    , col: p.col - 1 },
    }
}

#[derive(PartialEq, PartialOrd)]
enum Direction {
    Right,
    Down,
    Left,
    Up
}

// count half the steps to get all the way around the loop of pipes. chatgpt 4.0 helped make this slick
fn part1(input: &Input) -> u32 {
    
    // use a position cursor to traverse the pipe in one direction, counting each step
    let mut position = input.s_position.clone();
    let mut last_position = position.clone();
    let mut count_steps = 0;

    loop {
        // see where we can go from the current position
        let directions = input.grid.get_valid_directions(&position);
        if directions.len() != 2 {
            panic!("The problem description said there would be 2 valid pipe directions if we're on the pipe")
        }

        // take the first pipe that isn't the one we arrived from (to make sure we don't go backwards)
        let new_direction: Direction =
            directions.into_iter()
                      .find(|dir| last_position != position_in_direction(&position, dir))
                      .unwrap();
                   
        // Update last_position to the current position and move to the new position
        std::mem::swap(&mut last_position, &mut position);
        position = position_in_direction(&last_position, &new_direction);

        count_steps += 1;

        // break out of the loop when we arrive back at the starting position
        if position == input.s_position { break }
    }

    count_steps / 2
}

// count the number of cells in the interior of the loop
fn part2(input: &Input) -> u32 {
    0
}


/* Parsing */

// Read input from a file
fn input_from(file: &str) -> Result<Input, String> {

    let input = std::fs::read_to_string(file)
        .map_err(|err| format!("Failed to read file: {}", err))?;

    // mutable because we'll replace the S with the proper pipe segment before wrapping up the parse
    let mut grid = Grid {
        elements: input.lines()
                       .map(|line| line.chars().collect::<Vec<char>>())
                       .collect()
    };
    
    // the question mark ? here is almost unnoticed but it's doing so much. if replace_s_in_grid() returns Ok(x),
    // rust unwraps the x and assigns it to s_position and continues on with the code. but if it's an Err(str),
    // rust returns from the overall function right away, returning the Err(str)
    let s_position = replace_s_in_grid(&mut grid)?;

    Ok(Input {
        grid,
        s_position
    })
}

// replace the S in the grid with the proper pipe and return the position the S was at
fn replace_s_in_grid(grid: &mut Grid<char>) -> Result<Position, String> {

    let s_position = grid.find('S')
                         .ok_or("Couldn't find the S".to_string())?;
    let directions = grid.get_valid_directions(&s_position);
    
    if directions.len() != 2 {
        return Err("Expected two valid directions from the S".to_string())
    }

    // figure out the shape of pipe that connects these two directions and replace the S with it
    let segment = pipe_segment_must_be(&directions[0], &directions[1]);
    grid.elements[s_position.row][s_position.col] = segment;

    Ok(s_position)
}

// given two distinct Directions, conclude which shape the pipe segment connecting the two must be
fn pipe_segment_must_be(dir1: &Direction, dir2: &Direction) -> char {
    assert!(dir1 != dir2);

    // sort the directions to make the match arms simpler
    let (dir1, dir2) = if dir1 < dir2 { (dir1, dir2) }
                       else           { (dir2, dir1) };

    use Direction::*;

    // Right < Down < Left < Up
    match (dir1, dir2) {
        (Right, Down) => 'F',
        (Right, Left) => '-',
        (Right, Up)   => 'L',
        (Down, Left)  => '7',
        (Down, Up)    => '|',
        (Left, Up)    => 'J',
        _             => panic!("Shouldn't get here")
    }
}

impl<T: PartialEq> Grid<T>
{
    // look something up in the grid and return the position it's at, or None if it doesn't find it
    fn find(&self, target: T) -> Option<Position>
    {
        for (r, row) in self.elements.iter().enumerate() {
        for (c, element) in row.iter().enumerate() {
            if *element == target {
                return Some(Position {
                    row: r,
                    col: c
                })
            }
        }}
        None
    }
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let input = input_from("sample-1.txt").unwrap();
        assert_eq!(8, part1(&input))
    }

    #[test]
    fn test_sample_part2() {
        let input = input_from("sample-2.txt").unwrap();
        assert_eq!(4, part2(&input))
    }

    #[test]
    fn test_part1() {
        let input = input_from("input.txt").unwrap();
        assert_eq!(6828, part1(&input))
    }

    #[test]
    fn test_part2() {
        let input = input_from("input.txt").unwrap();
        assert_eq!(0, part2(&input))
    }
}
