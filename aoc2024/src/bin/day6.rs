/*
 * Part1: Determine the number of unique positions the guard passes through before leaving the map.
 *  The guard follows one direction until it reaches an obstacle then turns right and continues.
 *
 * Part2: How many distinct positions can an obstacle be placed in the map that would cause the
 * guard to be stuck in a loop?
 */

use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Grid {
    height: usize, // X
    width: usize,  // Y
    data: Vec<Vec<char>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut height = 0;
        let mut data = Vec::new();
        for line in input.lines() {
            height += 1;
            let row: Vec<char> = line.chars().collect();
            data.push(row);
        }
        Grid {
            width: data[0].len(),
            height,
            data,
        }
    }

    fn get(&self, index: &Point) -> Option<char> {
        let x = index.x as usize;
        let y = index.y as usize;
        if x < self.height && y < self.width {
            // This looks weird
            Some(self[index])
        } else {
            None
        }
    }

    fn set(&mut self, index: &Point, c: char) {
        self.data[index.x as usize][index.y as usize] = c
    }
}

impl std::ops::Index<Point> for Grid {
    type Output = char;

    fn index(&self, index: Point) -> &Self::Output {
        &self.data[index.x as usize][index.y as usize]
    }
}

impl std::ops::Index<&Point> for Grid {
    type Output = char;

    fn index(&self, index: &Point) -> &Self::Output {
        &self.data[index.x as usize][index.y as usize]
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point {
            x: x as i32,
            y: y as i32,
        }
    }
}

impl std::ops::Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::North => Point {
                x: self.x - 1,
                y: self.y,
            },
            Direction::South => Point {
                x: self.x + 1,
                y: self.y,
            },
            Direction::East => Point {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Point {
                x: self.x,
                y: self.y - 1,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

fn main() {
    let data = std::fs::read_to_string("./data/6.example").expect("couldn't open the file");
    let part1 = part1(&data);
    println!("Part1: {part1}");
    let part2 = part2(&data);
    println!("Part2: {part2}");
}

fn part1(data: &str) -> usize {
    let grid = Grid::new(data);

    get_visited(&grid, false).unwrap().len()
}

// Damn this is really slow. 20+ seconds!
fn part2(data: &str) -> usize {
    let grid = Grid::new(data);

    // Grab visited points
    let visited = get_visited(&grid, false).unwrap();

    // Get all possible positions
    let mut product = vec![];
    for i in 0..grid.width {
        for j in 0..grid.height {
            product.push((i, j));
        }
    }

    // For each possible position, if it is a wall, check if there is a loop
    // If there is a loop, filter that as possible position, then count it
    product
        .iter()
        .filter(|&(x, y)| {
            let p = Point::new(*x, *y);
            if visited.get(&p) == Some(&p) {
                let mut new_grid = grid.clone();
                new_grid.set(&p, '#');
                get_visited(&new_grid, true).is_none()
            } else {
                false
            }
        })
        .count()
}

fn get_visited(grid: &Grid, check_loop: bool) -> Option<HashSet<Point>> {
    let mut start = Point { x: 0, y: 0 };

    for x in 0..grid.width {
        for y in 0..grid.height {
            let p = Point::new(x, y);
            if grid[&p] == '^' {
                start = p;
            }
        }
    }

    let mut direction = Direction::North;
    let mut current_position = start;

    let mut next_position = start + direction;
    let mut visited: HashSet<Point> = std::collections::HashSet::new();
    let mut seen: HashSet<(Point, Direction)> = std::collections::HashSet::new();

    while grid.get(&current_position).is_some() {
        visited.insert(current_position);
        if grid.get(&next_position).is_none() {
            break;
        }
        if grid[next_position] == '#' {
            direction = direction.turn();
        } else {
            current_position = next_position;
        }
        if check_loop {
            // If we have seen the same position and going the same direction, we are in a loop
            if seen.contains(&(current_position, direction)) {
                return None;
            }
            seen.insert((current_position, direction));
        }
        next_position = current_position + direction;
    }

    Some(visited)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let data = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        let result = part1(data);
        assert_eq!(result, 41);
    }

    #[test]
    fn part2_example() {
        let data = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        let result = part2(data);
        assert_eq!(result, 6);
    }
}
