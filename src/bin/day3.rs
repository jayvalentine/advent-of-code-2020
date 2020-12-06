// Advent of Code 2020
// Day 3

use std::collections::HashMap;
use std::fs;

#[cfg(test)]
mod test_examples {
    use super::*;

    // The example from part 1 of the puzzle.
    #[test]
    fn test_example_part1() {
        let grid = "
        ..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#";

        assert_eq!(7, trees_in_slope(3, 1, grid).expect("Error calculating number of trees."));
    }

    // The example from part 2 of the puzzle.
    #[test]
    fn test_example_part2() {
        let grid = "
        ..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#";

        assert_eq!(2, trees_in_slope(1, 1, grid).expect("Error calculating number of trees."));
        assert_eq!(7, trees_in_slope(3, 1, grid).expect("Error calculating number of trees."));
        assert_eq!(3, trees_in_slope(5, 1, grid).expect("Error calculating number of trees."));
        assert_eq!(4, trees_in_slope(7, 1, grid).expect("Error calculating number of trees."));
        assert_eq!(2, trees_in_slope(1, 2, grid).expect("Error calculating number of trees."));
    }
}

#[cfg(test)]
mod test_grid_parsing {
    use super::*;

    // Ensure we raise an error when trying to parse an empty grid.
    #[test]
    fn test_empty_grid() {
        let grid = "";

        let err = parse_grid(grid).err().expect("Error not returned");

        assert_eq!("Invalid grid: empty", err);
    }

    // Ensure we raise an error when grid lines are of different lengths.
    #[test]
    fn test_grid_with_mismatched_lines() {
        let grid = "..#..#\n...##";

        let err = parse_grid(grid).err().expect("Error not returned");

        assert_eq!("Invalid grid: lines of different lengths", err);
    }

    // Ensure we raise an error when unsupported characters are used in a grid.
    #[test]
    fn test_grid_with_unsupported_chars() {
        let grid = "..#.\n..#Y";

        let err = parse_grid(grid).err().expect("Error not returned");

        assert_eq!("Invalid grid: unsupported character", err);
    }

    // Ensure we can parse a valid grid correctly.
    #[test]
    fn test_valid_grid() {
        let grid_string = "..#.\n.#..\n..##";

        let grid = parse_grid(grid_string).expect("Grid parsing failed");

        assert_eq!(false, grid.tree_at(0, 0));
        assert_eq!(true, grid.tree_at(2, 0));
        assert_eq!(false, grid.tree_at(0, 1));
        assert_eq!(true, grid.tree_at(3, 2));
    }

    // Ensure we can index a grid using modulo arithmetic.
    #[test]
    fn test_valid_grid_modulo() {
        let grid_string = "..#.\n.#..\n..##";

        let grid = parse_grid(grid_string).expect("Grid parsing failed");

        assert_eq!(true, grid.tree_at(6, 0));
        assert_eq!(false, grid.tree_at(4, 1));
        assert_eq!(true, grid.tree_at(7, 2));
    }
}

struct Grid {
    x_size: usize,
    y_size: usize,
    grid: HashMap<(usize, usize), bool>
}

impl Grid {
    fn tree_at(&self, x: usize, y: usize) -> bool {
        return *self.grid.get(&(x % self.x_size, y)).expect("Invalid grid access.");
    }
}

fn parse_grid(g: &str) -> Result<Grid, &str> {
    if g.trim().is_empty() {
        return Err("Invalid grid: empty");
    }

    let mut line_length = 0;

    let mut x = 0;
    let mut y = 0;

    let mut grid: HashMap<(usize, usize), bool> = HashMap::new();

    for line in g.trim().lines() {
        let line = line.trim();

        if line_length != 0 && line.len() != line_length {
            return Err("Invalid grid: lines of different lengths");
        }

        line_length = line.len();

        for c in line.chars() {
            let is_tree = match c {
                '.' => false,
                '#' => true,
                _ => return Err("Invalid grid: unsupported character")
            };

            grid.insert((x, y), is_tree);

            x += 1;
        }

        y += 1;
        x = 0;
    }

    return Ok(Grid { x_size: line_length, y_size: y, grid });
}

// Given a grid and a slope (expressed as an X-speed - Y-speed is assumed to be 1)
// returns the number of trees encountered on that slope.
fn trees_in_slope(right: usize, down: usize, grid: &str) -> Result<usize, &str> {
    let grid = parse_grid(grid)?;

    let mut x = 0;
    let mut y = 0;

    let mut num_trees = 0;

    while y < grid.y_size {
        if grid.tree_at(x, y) {
            num_trees += 1;
        }

        x += right;
        y += down;
    }

    Ok(num_trees)
}

fn main() {
    let grid = fs::read_to_string("data/day3.txt").expect("Error reading file data/day3.txt");

    let num_trees = match trees_in_slope(3, 1, &grid) {
        Ok(n) => n,
        Err(s) => { println!("{}", s); return; }
    };

    println!("Part 1: Number of trees encountered is {}", num_trees);

    let num_trees_a = trees_in_slope(1, 1, &grid).unwrap();
    let num_trees_b = num_trees; // Already checked in previous solution.
    let num_trees_c = trees_in_slope(5, 1, &grid).unwrap();
    let num_trees_d = trees_in_slope(7, 1, &grid).unwrap();
    let num_trees_e = trees_in_slope(1, 2, &grid).unwrap();

    println!("Part 2: Total number of trees encountered (product) is: {}",
             num_trees_a * num_trees_b * num_trees_c * num_trees_d * num_trees_e);
}
