/*
 * Part1: Word Search. Find the number of times 'XMAS' appears. It can be forwards, backwards, or
 * diagonal.
 *
 * Part2: Actually, you are supposed to find 'MAS' in the shape of an X. Each X is one. Find total
 * number of Xs
 */
fn main() {
    let data = std::fs::read_to_string("./data/4.input").expect("could not read file");
    let part1 = part1(&data);
    println!("{part1}");
    let part2 = part2(&data);
    println!("{part2}");
}

fn part1(data: &str) -> u32 {
    let grid = parse_input(data);
    let mut count = 0;

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] != 'X' {
                continue;
            }

            // We are at 'X' now, we need to check for next letter in all directions
            for dx in -1..=1 {
                'vertical: for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let mut new_x = x as isize;
                    let mut new_y = y as isize;

                    let mut matched_letter = 0;
                    for c in ['M', 'A', 'S'].iter() {
                        new_y += dx; // Horizontal check
                        new_x += dy; // Vertical check
                        let ux = new_x as usize;
                        let uy = new_y as usize;

                        // Bounds check
                        if uy < grid[0].len() && ux < grid.len() {
                            let checking = grid[ux][uy];
                            if checking != *c {
                                continue 'vertical;
                            } else {
                                matched_letter += 1;
                            }
                        } else {
                            continue 'vertical;
                        }
                    }
                    if matched_letter == 3 {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn part2(data: &str) -> u32 {
    let grid = parse_input(data);
    let mut count = 0;

    dbg!(grid.clone());

    for x in 1..grid.len() {
        'vertical: for y in 1..grid[0].len() {
            if grid[x][y] != 'A' {
                continue;
            }

            // We are at 'A' and need to check each corner
            for dx in [-1, 1] {
                let new_x = x as isize;
                let new_y = y as isize;

                // Need to check [-1,-1] & [-1,1] & [1,-1] & [1,1]
                let x1 = (new_x - 1) as usize;
                let x2 = (new_x + 1) as usize;
                let y1 = (new_y + dx) as usize;
                let y2 = (new_y - dx) as usize;

                // Bounds check
                if (x1 < grid.len() && x2 < grid.len())
                    && (y1 < grid[0].len() && y2 < grid[0].len())
                {
                    if (grid[x1][y1] == 'M'
                        && grid[x1][y2] == 'M'
                        && grid[x2][y1] == 'S'
                        && grid[x2][y2] == 'S')
                        || (grid[x1][y1] == 'S'
                            && grid[x1][y2] == 'S'
                            && grid[x2][y1] == 'M'
                            && grid[x2][y2] == 'M')
                    {
                        count += 1;
                        continue 'vertical; // Found a valid X, can move on to next 'A'
                    }
                    if (grid[x1][y1] == 'M'
                        && grid[x1][y2] == 'S'
                        && grid[x2][y1] == 'M'
                        && grid[x2][y2] == 'S')
                        || (grid[x1][y1] == 'S'
                            && grid[x1][y2] == 'M'
                            && grid[x2][y1] == 'S'
                            && grid[x2][y2] == 'M')
                    {
                        count += 1;
                        continue 'vertical; // Found a valid X, can move on to next 'A'
                    }
                }
            }
        }
    }
    count
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    // [ [A, B, C, D], 0,0 = A; 0,1 = B ...
    //   [E, F, G, H], 1,0 = E; 1,1 = F ...
    // ] Visually, X is the Vertical axis, Y is the Horizontal axis

    let mut grid = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_horizontal_forwards() {
        let data = r"XMAS
....
....
....";
        let result = part1(data);
        assert_eq!(result, 1);
    }

    #[test]
    fn case_horizontal_backwards() {
        let data = r"SAMX
....
....
....";
        let result = part1(data);
        assert_eq!(result, 1);
    }

    #[test]
    fn case_vertical_forwards() {
        let data = r"X...
M...
A...
S...";
        let result = part1(data);
        assert_eq!(result, 1);
    }

    #[test]
    fn case_topleft_diagonal_forwards() {
        let data = r"X...
.M..
..A.
...S";
        let result = part1(data);
        assert_eq!(result, 1);
    }

    #[test]
    fn case_topleft_diagonal_backwards() {
        let data = r"S...
.A..
..M.
...X";
        let result = part1(data);
        assert_eq!(result, 1)
    }

    #[test]
    fn case_bottomleft_diagonal_forwards() {
        let data = r"...S
..A.
.M..
X...";
        let result = part1(data);
        assert_eq!(result, 1)
    }

    #[test]
    fn case_bottomleft_diagonal_backwards() {
        let data = r"...X
..M.
.A..
S...";
        let result = part1(data);
        assert_eq!(result, 1)
    }

    #[test]
    fn case_x() {
        let data = r"X..X
.MM.
.AA.
S..S";
        let result = part1(data);
        assert_eq!(result, 2)
    }

    #[test]
    fn case_cross() {
        let data = r".X..
XMAS
.A..
.S..";
        let result = part1(data);
        assert_eq!(result, 2)
    }

    #[test]
    fn part1_example() {
        let data = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let result = part1(data);
        assert_eq!(result, 18);
    }

    #[test]
    fn case_xmas() {
        let data = r"M.M.
.A..
S.S.";
        let result = part2(data);
        assert_eq!(result, 1);
    }

    #[test]
    fn part2_example() {
        let data = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let result = part2(data);
        assert_eq!(result, 9);
    }
}
