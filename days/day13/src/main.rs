// https://adventofcode.com/2023/day/13

fn main() -> Result<(), String> {

    let sample = input_from("sample.txt")?;
    println!("Sample part 1: {}", part1(&sample));
    println!("Sample part 2: {}", part2(&sample));

    let input = input_from("input.txt")?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

struct Input {
    grids: Vec<Grid<char>>
}

// a 2D-grid
struct Grid<T> {
    elements: Vec<Vec<T>>
}

fn part1(input: &Input) -> usize {
    input.grids
         .iter()
         .map(|grid| score_grid(grid, Part1))
         .sum()
}

fn part2(input: &Input) -> usize {
    input.grids
         .iter()
         .map(|grid| score_grid(grid, Part2))
         .sum()
}

fn score_grid(grid: &Grid<char>, part: Part) -> usize {
    let mut score = 0;

    if let Some(row) = find_mirror_point(grid, &part) {
        score += row * 100
    }
    if let Some(column) = find_mirror_point(&grid.transpose(), &part) {
        score += column
    }
    score
}

#[allow(unused_parens)]
fn find_mirror_point(grid: &Grid<char>, part: &Part) -> Option<usize> {
    assert!(grid.elements.len() >= 2);

    'next_k: for k in 0..grid.elements.len()-1 {
        let mut count_diffs = 0;

        // fan out from the current center-line between k and k+1
        let backwards = (  0 ..= k).rev();
        let forwards  = (k+1 .. grid.elements.len());

        let iter_backwards = griderator(grid, Box::new(backwards));
        let iter_forward   = griderator(grid, Box::new(forwards));

        let zipped = iter_backwards.zip(iter_forward);

        for (b, f) in zipped {
            if b == f { continue }
            
            count_diffs += 1;

            match part {
                Part1 => { continue 'next_k },
                Part2 => {
                    // short-circuit this k attempt when there's more than one difference,
                    // we're looking for exactly one
                    if count_diffs > 1 {
                        continue 'next_k
                    }
                }
            }
        }

        match part {
            Part1                     => return Some(k + 1),
            Part2 if count_diffs == 1 => return Some(k + 1),
            Part2                     => continue
        }
    }

    None
}

// we need to iterate over the grid. so this is the, um, Griderator
struct Griderator<'a> {
    iter: Box<dyn Iterator<Item=char> + 'a>
}

fn griderator(grid: &Grid<char>,
              indices: Box<dyn Iterator<Item=usize>>) -> Griderator
{
    let flattened_iter =
        indices.map(move |index| &grid.elements[index]) // Select rows by indices
               .flat_map(|row| row.iter().copied());    // Flatten rows into chars

    Griderator {
        iter: Box::new(flattened_iter),
    }
}

impl Iterator for Griderator<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

// transposition of Vec<Vec<_>> by chatgpt 4.0
impl<T: Clone> Grid<T> {
    fn transpose(&self) -> Grid<T> {
        if self.elements.is_empty() || self.elements[0].is_empty() {
            return Grid { elements: vec![] };
        }

        let rows = self.elements.len();
        let cols = self.elements[0].len();

        let mut transposed_elements = Vec::with_capacity(cols);
        
        for j in 0..cols {
            let mut transposed_row = Vec::with_capacity(rows);
            for i in 0..rows {
                transposed_row.push(self.elements[i][j].clone());
            }
            transposed_elements.push(transposed_row);
        }

        Grid {
            elements: transposed_elements,
        }
    }
}

enum Part {
    Part1,
    Part2
}

use Part::*;


/* Parsing */

// Read input from a file
fn input_from(file: &str) -> Result<Input, String> {

    let input = std::fs::read_to_string(file)
        .map_err(|err| format!("Failed to read file: {}", err))?;

    let grids: Vec<Grid<char>> =
        input.split("\r\n\r\n")
             .map(|block| Grid {
                elements: block.lines()
                               .map(|line| line.chars().collect())
                               .collect()
             })
             .collect();
                      
    Ok(Input {
        grids
    })
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let input = input_from("sample.txt").unwrap();
        assert_eq!(405, part1(&input))
    }

    #[test]
    fn test_sample_part2() {
        let input = input_from("sample.txt").unwrap();
        assert_eq!(400, part2(&input))
    }

    #[test]
    fn test_part1() {
        let input = input_from("input.txt").unwrap();
        assert_eq!(29165, part1(&input))
    }

    #[test]
    fn test_part2() {
        let input = input_from("input.txt").unwrap();
        assert_eq!(32192, part2(&input))
    }

    #[test]
    fn test_find_mirror_point_part1() {
        let input = input_from("sample.txt").unwrap();
        let part = Part1;

        assert_eq!(None   , find_mirror_point(&input.grids[0], &part));
        assert_eq!(Some(5), find_mirror_point(&input.grids[0].transpose(), &part));
        assert_eq!(Some(4), find_mirror_point(&input.grids[1], &part));
    }
}

/*
    $ time target/release/day13.exe 
    Sample part 1: 405
    Sample part 2: 400
    Part 1: 29165
    Part 2: 32192

    real    0m0.040s
    user    0m0.000s
    sys     0m0.015s
*/
