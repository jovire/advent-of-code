/*
 * Part 1: Find total distance between left and right list. Pair the smallest number together in
 * both lists and find the difference between them. Do the same for rest of the numbers. Add those
 * distances.
 *
 * Part 2: Determine the similarity score but multiplying the left list by the number of times it
 * appears in the right list. Add those scores together.
 */
fn main() {
    let data = std::fs::read_to_string("./data/1.input").expect("couldn't read input file");
    let part1 = part1(&data);
    println!("Part1: {part1}");
    let part2 = part2(&data);
    println!("Part2: {part2}");
}

fn part1(data: &str) -> u64 {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in data.lines() {
        if let Some((a, b)) = line.split_once("   ") {
            left.push(a.parse::<u64>().unwrap());
            right.push(b.parse::<u64>().unwrap());
        }
    }

    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .map(|(v1, v2)| v1.abs_diff(v2))
        .sum::<u64>()
}

fn part2(data: &str) -> u64 {
    let mut map = std::collections::HashMap::new();

    let mut left = Vec::new();
    for line in data.lines() {
        if let Some((a, b)) = line.split_once("   ") {
            left.push(a.parse::<u64>().unwrap());
            *map.entry(b.parse::<u64>().unwrap()).or_insert(0) += 1;
        }
    }

    let mut result = 0;
    for val in left {
        if map.contains_key(&val) {
            result += val * map[&val];
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let data = r"
3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(part1(data), 11);
    }
    #[test]
    fn part2_example() {
        let data = r"
3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(part2(data), 31);
    }
}
