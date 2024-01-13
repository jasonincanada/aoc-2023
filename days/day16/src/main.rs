// https://adventofcode.com/2023/day/16

fn main() -> Result<(), String> {

    let sample = input_from("sample.txt")?;
    println!("Sample part 1: {}", part1(&sample)); // 46
    println!("Sample part 2: {}", part2(&sample)); // 51

    let input = input_from("input.txt")?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

struct Input {
    grid: Grid<char>
}

// a 2D-grid
#[derive(Clone)]
struct Grid<T> {
    elements: Vec<Vec<T>>
}

fn part1(input: &Input) -> usize { process(&input.grid, true) }
fn part2(input: &Input) -> usize { process(&input.grid, false) }

fn process(grid: &Grid<char>, part1: bool) -> usize
{
    grid.starting_positions()
        .into_iter()
        .filter(|(position, direction)| {

               // part 1 only starts at the top left cell facing Right
               (part1 && position.row == 0
                      && position.col == 0
                      && *direction == Right)

               // part 2 starts at all possible starting positions so let them all through
            || (!part1)

        })
        .map(|(position, direction)| {
            // tiles can be energized or not, start them off as not energized
            let mut tiles = grid.clone_for_energies();

            // recursively call this to traverse the grid, tracking which tiles become energized
            at_position(position, direction, &mut tiles, grid);

            tiles.count_energized_tiles()
        })
        .max()
        .unwrap()
}

// call this after arriving at a position
fn at_position(position : Position,
               direction: Direction,
               tiles    : &mut Grid<EnergyLevels>,
               grid     : &Grid<char>)
{
    // energize the tile in this direction. there are four possible directions and we are allowed to
    // criss-cross beams, so we'll need to track 4 bits of information at each position
    tiles.energize(&position, &direction);

    let from_direction = direction.opposite();

    let next_directions: Vec<Direction> =
        [ Right, Down, Left, Up ]
            .into_iter()
            .filter(|dir| does_light_go_from_to(&from_direction, grid[&position], dir))
            .filter(|dir| grid.can_step(&position, dir)) // stay on the grid 
            .collect();

    for dir in next_directions {
        let pos = position_in_direction(&position, &dir);

        // skip directions we've already gone in or the algorithm could loop infinitely
        if tiles[&pos].is_energized(&dir) {
            continue
        }

        // recurse at the new position
        at_position(pos, dir, tiles, grid);
    }
}

fn does_light_go_from_to(from: &Direction, tile: char, to: &Direction) -> bool {

    match (tile, from, to) {
        
        // no-mirror tiles
        ('.', Right, Left) => true,
        ('.', Down, Up) => true,
        ('.', Left, Right) => true,
        ('.', Up, Down) => true,

        // mirror tiles
        ('|', Right, Down) => true,
        ('|', Right, Up) => true,
        ('|', Down, Up) => true,
        ('|', Left, Down) => true,
        ('|', Left, Up) => true,
        ('|', Up, Down) => true,
        ('-', Right, Left) => true,
        ('-', Down, Right) => true,
        ('-', Down, Left) => true,
        ('-', Left, Right) => true,
        ('-', Up, Right) => true,
        ('-', Up, Left) => true,
        ('\\', Right, Up) => true,
        ('\\', Down, Left) => true,
        ('\\', Left, Down) => true,
        ('\\', Up, Right) => true,
        ('/', Right, Down) => true,
        ('/', Down, Right) => true,
        ('/', Left, Up) => true,
        ('/', Up, Left) => true,

        // false for all other combinations
        _ => false
    }
}

impl<T> Grid<T> {
    fn height(&self) -> usize { self.elements.len() }
    fn width(&self)  -> usize { self.elements[0].len() }

    // copy the shape (height/width) of the grid but make its elements all empty EnergyLevels structs
    fn clone_for_energies(&self) -> Grid<EnergyLevels> {
        Grid {
            elements: vec![ vec![ EnergyLevels::default(); self.width() ]; self.height() ]
        }
    }

    // are we still on the grid if we step in this direction
    fn can_step(&self, pos: &Position, dir: &Direction) -> bool {
        match dir {
            Right => pos.col < self.width() - 1,
            Down  => pos.row < self.height() - 1,
            Left  => pos.col > 0,
            Up    => pos.row > 0
        }
    }

    // get all possible starting positions and directions (part 2 tries them all)
    fn starting_positions(&self) -> Vec<(Position, Direction)> {
        let mut ps: Vec<(Position, Direction)> = vec![];

        // start left and travel right and vice versa for each row
        (0..self.height()).for_each(|row| {
            ps.push((Position { row, col: 0             }, Right));
            ps.push((Position { row, col: self.width()-1}, Left));
        });

        // start at the top and travel down and vice versa for each column
        (0..self.width()).for_each(|col| {
            ps.push((Position { row: 0, col              }, Down));
            ps.push((Position { row: self.height()-1, col}, Up));
        });

        ps
    }
}

#[derive(Clone)]
struct EnergyLevels {
    right: bool,
    down : bool,
    left : bool,
    up   : bool,
}

impl EnergyLevels {
    fn energize(&mut self, direction: &Direction) {
        match direction {
            Right => self.right = true,
            Down  => self.down  = true,
            Left  => self.left  = true,
            Up    => self.up    = true,
        }
    }

    fn is_energized(&self, direction: &Direction) -> bool {
        match direction {
            Right => self.right,
            Down  => self.down,
            Left  => self.left,
            Up    => self.up,
        }
    }

    fn is_energized_at_all(&self) -> bool {
        self.right || self.down || self.left || self.up
    }
}

impl Default for EnergyLevels {
    // start out unenergized
    fn default() -> Self {
        Self {
            right: false,
            down: false,
            left: false,
            up: false,
        }
    }
}

impl Grid<EnergyLevels> {
    fn energize(&mut self, position: &Position, direction: &Direction) {
        self.elements[position.row][position.col].energize(direction);
    }

    fn count_energized_tiles(&self) -> usize {
        self.elements.iter()
                     .flat_map(|row| row.iter())
                     .filter(|&tile| tile.is_energized_at_all())
                     .count()
    }
}

struct Position {
    row: usize,
    col: usize
}

#[derive(PartialEq)]
enum Direction {
    Right,
    Down,
    Left,
    Up
}

use Direction::*;

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Right => Left,
            Down => Up,
            Left => Right,
            Up => Down,
        }
    }
}

// make Grid indexable with a Position
impl<T> std::ops::Index<&Position> for Grid<T> {
    type Output = T;

    fn index(&self, pos: &Position) -> &Self::Output {
        if pos.row >= self.elements.len()    { panic!("Index outside of row bounds") }
        if pos.col >= self.elements[0].len() { panic!("Index outside of col bounds") }
        
        &self.elements[pos.row][pos.col]
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


/* Parsing */

// Read input from a file
fn input_from(file: &str) -> Result<Input, String> {

    let input = std::fs::read_to_string(file)
        .map_err(|err| format!("Failed to read file: {}", err))?;

    Ok(Input {
        grid: Grid {
            elements: input.lines()
                           .map(|line| line.chars().collect())
                           .collect()
        }
    })
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let input = input_from("sample.txt").unwrap();
        assert_eq!(46, part1(&input))
    }

    #[test]
    fn test_sample_part2() {
        let input = input_from("sample.txt").unwrap();
        assert_eq!(51, part2(&input))
    }

    #[test]
    fn test_part1() {
        let input = input_from("input.txt").unwrap();
        assert_eq!(8116, part1(&input))
    }

    #[test]
    fn test_part2() {
        let input = input_from("input.txt").unwrap();
        assert_eq!(8383, part2(&input))
    }
}

/*
    $ time target/release/day16.exe 
    Sample part 1: 46
    Sample part 2: 51
    Part 1: 8116
    Part 2: 8383

    real    0m0.360s
    user    0m0.000s
    sys     0m0.015s
*/
