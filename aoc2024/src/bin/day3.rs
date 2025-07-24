use regex::Regex;

/*
 * Part1: Follow the correct multiplier instruction, mul(x,y), and ignore the rest. Multiply the
 * values together and then add them to get the final result
 *
 * Part2: Same as part1, except if the instruction says `don't` then don't do anything
 */
fn main() {
    let data = std::fs::read_to_string("./data/3.input").expect("couldn't read file");
    let part1 = part1(&data);
    println!("Part1 : {part1}");
    let part2 = part2(&data);
    println!("Part2 : {part2}");
}

fn part1(data: &str) -> u64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(&data)
        .map(|c| c[1].parse::<u64>().unwrap() * c[2].parse::<u64>().unwrap())
        .sum::<u64>()
}

fn part2(data: &str) -> u64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    let mut sum = true;
    let mut result = vec![];
    for cap in re.captures_iter(&data) {
        match &cap[0] {
            "do()" => sum = true,
            "don't()" => sum = false,
            _ => {
                if sum {
                    let val = cap[1].parse::<u64>().unwrap() * cap[2].parse::<u64>().unwrap();
                    result.push(val);
                }
            }
        }
    }
    result.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let data = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let result = part1(data);
        assert_eq!(result, 161);
    }

    #[test]
    fn part2_example() {
        let data = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let result = part2(data);
        assert_eq!(result, 48);
    }
}
