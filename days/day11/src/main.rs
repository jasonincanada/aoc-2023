// https://adventofcode.com/2023/day/11

fn main() -> Result<(), String> {

    let sample = input_from("sample.txt")?;
    println!("Sample part 1: {}", part1(&sample)); // 374
    println!("Sample part 2: {}", part2(&sample)); // 82000210

    let input = input_from("input.txt")?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

struct Input {
    galaxies: Grid2D<char>
}

// simulate expanding space by remembering how many times an originally-empty row or column
// has been doubled. the galaxies themselves stay immutable and their actual positions are
// computed on demand by accumulating the distances (in row_multiples and col_multiples)
struct ExpandingSpace {

    // where we found galaxies in the original input
    galaxy_positions: Vec<Position>,
    
    // the indices of the empty rows/columns
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,

    // keep track of the counts of empty rows/columns as space expands. they are just
    // multiples of the original 1 row or column
    row_multiples: Vec<usize>,
    col_multiples: Vec<usize>,
}

impl ExpandingSpace {
    fn around(galaxies: &Grid2D<char>) -> Self {

        // find the empty rows/columns and the positions of all the galaxies
        let mut empty_rows: Vec<usize> = vec![];
        let mut empty_cols: Vec<usize> = vec![];
        let mut galaxy_positions: Vec<Position> = vec![];

        for row in 0..galaxies.height() {

            // if this is a row of all spaces
            if galaxies.data[row].iter().all(|s| *s == '.') {
                empty_rows.push(row);
                continue
            }

            // it's not all spaces, so locate the galaxies
            let mut positions =
                galaxies.data[row].iter()
                                  .enumerate()
                                  .filter_map(|(col, &thing)| {
                                      if thing == '#' {
                                          Some(Position { row, col })
                                      } else {
                                          None
                                      }
                                  })
                                  .collect();

            galaxy_positions.append(&mut positions);
        }

        for col in 0..galaxies.width() {
            // if this is a column of all spaces
            if galaxies.data.iter().all(|row| row[col] == '.') {
                empty_cols.push(col);
            }
        }

        // the empty rows/cols remember how much they were expanded
        let row_multiples: Vec<usize> = vec![ 1; galaxies.height() ];
        let col_multiples: Vec<usize> = vec![ 1; galaxies.width() ];

        ExpandingSpace {
            galaxy_positions,
            empty_rows,
            empty_cols,
            row_multiples,
            col_multiples,
        }
    }

    // multiply the empty space in this ExpandingSpace
    fn expand_emptiness(&mut self, factor: usize) {
        for &row in &self.empty_rows { self.row_multiples[row] *= factor }
        for &col in &self.empty_cols { self.col_multiples[col] *= factor }
    }

    // we only store the original positions of the galaxies, but obviously after expanding the
    // empty space they should be in different places. so compute a galaxy's actual position
    fn get_galaxy_position(&self, idx: usize) -> Position {
        let position = &self.galaxy_positions[idx];
        let mut row = 0;
        let mut col = 0;

        for r in 0 .. position.row { row += self.row_multiples[r] }
        for c in 0 .. position.col { col += self.col_multiples[c] }

        Position { row, col }
    }
}


fn part1(input: &Input) -> usize { process(&input.galaxies, 2) }
fn part2(input: &Input) -> usize { process(&input.galaxies, 1_000_000) }

fn process(galaxies: &Grid2D<char>, by_factor: usize) -> usize {
    
    let mut space = ExpandingSpace::around(&galaxies);
    
    // first expand the universe
    space.expand_emptiness(by_factor);
    
    // then calculate the shortest distances between all pairs of galaxies
    let mut distances: Vec<usize> = vec![] ;

    for i in 0 .. space.galaxy_positions.len() {
    for j in i+1 .. space.galaxy_positions.len()
    {
        let gal1 = space.get_galaxy_position(i);
        let gal2 = space.get_galaxy_position(j);

        distances.push(shortest_distance(&gal1, &gal2));
    }}

    distances.iter().sum()
}

// return the manhattan distance from one position to another
fn shortest_distance(pos1: &Position, pos2: &Position) -> usize {
    let dx = pos1.col.abs_diff(pos2.col);
    let dy = pos1.row.abs_diff(pos2.row);

    dx + dy
}

struct Grid2D<T> {
    data: Vec<Vec<T>>
}

impl<T> Grid2D<T> {
    // assume all rows are the same length
    fn width(&self)  -> usize { self.data[0].len() }
    fn height(&self) -> usize { self.data.len() }
}

struct Position {
    row: usize,
    col: usize,
}


/* Parsing */

// Read input from a file
fn input_from(file: &str) -> Result<Input, String> {

    let input = std::fs::read_to_string(file)
        .map_err(|err| format!("Failed to read file: {}", err))?;

    Ok(Input {
        galaxies: Grid2D {
            data: input.lines()
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
        assert_eq!(374, part1(&input))
    }

    #[test]
    fn test_sample_part2() {
        let input = input_from("sample.txt").unwrap();
        assert_eq!(82000210, part2(&input))
    }

    #[test]
    fn test_part1() {
        let input = input_from("input.txt").unwrap();
        assert_eq!(9742154, part1(&input))
    }

    #[test]
    fn test_part2() {
        let input = input_from("input.txt").unwrap();
        assert_eq!(411142919886, part2(&input))
    }
}

/*
    $ time target/release/day11.exe 
    Sample part 1: 374
    Sample part 2: 82000210
    Part 1: 9742154
    Part 2: 411142919886

    real    0m0.056s
    user    0m0.000s
    sys     0m0.000s
*/
