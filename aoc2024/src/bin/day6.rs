/*
 * Part1: Determine the number of unique positions the guard passes through before leaving the map.
 *  The guard follows one direction until it reaches an obstacle then turns right and continues.
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

#[derive(Debug, Clone, Copy)]
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
}

fn part1(data: &str) -> usize {
    let grid = Grid::new(data);

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
        next_position = current_position + direction;
    }

    visited.len()
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
}
