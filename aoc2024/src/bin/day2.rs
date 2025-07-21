/*
 * Part1: Determine which levels are safe.
 * Safe is defined as: all increasing OR all decreasing AND adjacent levels are [1,2,3] apart.
 * Part2: Determine if any levels are safe IF one of the values is removed from the level.
 */
fn main() {
    let data = std::fs::read_to_string("./data/2.input").expect("couldn't open file");

    let part1 = part1(&data);
    println!("Part1: {part1}");
    let part2 = part2(&data);
    println!("Part2: {part2}");
}

fn part1(data: &str) -> i64 {
    let mut count = 0;
    for line in data.lines() {
        let l = line
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        if validate_level(&l) {
            count += 1;
        }
    }
    count
}

fn part2(data: &str) -> i64 {
    let mut count = 0;
    for line in data.lines() {
        let l = line
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        if validate_level(&l)
            || l.iter().enumerate().any(|(i, _)| {
                let mut new_level = l.to_vec();
                new_level.remove(i);
                validate_level(&new_level)
            })
        {
            count += 1;
        }
    }
    count
}

fn validate_level(level: &[i64]) -> bool {
    // They must be sorted (forwards or backwards)
    // difference between adjacent values cannot be >3
    (level.is_sorted() || level.iter().rev().is_sorted())
        && level
            .windows(2)
            .all(|w| w[0] != w[1] && (w[0] - w[1]).abs() <= 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        let valid = vec![7, 6, 4, 2, 1];
        assert!(validate_level(&valid));
    }
    #[test]
    fn invalid() {
        let invalid = vec![1, 2, 7, 8, 9];
        assert!(!validate_level(&invalid));
    }

    #[test]
    fn part1_example() {
        let data = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let result = part1(data);
        assert_eq!(result, 2);
    }

    #[test]
    fn part2_example() {
        let data = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let result = part2(data);
        assert_eq!(result, 4);
    }
}
