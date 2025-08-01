use std::collections::BTreeMap;

/*
 * Part1: Given a set of rules, is the update valid? For the valid updates, sum the mid points
 * Part2: Correct the incorrectly-ordered updates, and perform the same calculation as Part1
 */
fn main() {
    let data = std::fs::read_to_string("./data/5.example").expect("could not open file");
    let part1 = part1(&data);
    println!("Part1: {part1}");
    let part2 = part2(&data);
    println!("Part2: {part2}");
}

fn part1(data: &str) -> u32 {
    let (rules, updates) = parse_input(data);

    updates
        .into_iter()
        .filter(|u| validate_rule(&rules, u))
        .map(|x| x[x.len() / 2])
        .sum::<u32>()
}

fn part2(data: &str) -> u32 {
    let (rules, updates) = parse_input(data);

    // Filter for incorrect updates. Correct them and sum the midpoints
    updates
        .into_iter()
        .filter(|u| !validate_rule(&rules, u))
        .map(|mut v| {
            v.sort_by(|&a, &b| {
                if validate_rule(&rules, &[a, b]) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });
            v
        })
        .map(|x| x[x.len() / 2])
        .sum::<u32>()
}

fn validate_rule(rules: &BTreeMap<u32, Vec<u32>>, update: &[u32]) -> bool {
    // For each item in the update, check if the following elements are also valid

    let update_len = update.len();
    for i in 0..update_len - 1 {
        let curr_val = update[i];
        if !rules.contains_key(&curr_val) {
            return false;
        }
        for j in i + 1..update_len {
            // Is current value valid?
            let val = update[j];
            if !rules[&curr_val].contains(&val) {
                return false;
            }
        }
    }
    true
}

fn parse_input(input: &str) -> (BTreeMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let mut rules_map: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if let Some((k, v)) = line.split_once('|') {
            let k = k.parse::<u32>().unwrap();
            let v = v.parse::<u32>().unwrap();
            let entry = rules_map.entry(k).or_default();
            entry.push(v);
        } else {
            let update = line.split(',').map(|n| n.parse::<u32>().unwrap()).collect();
            updates.push(update);
        }
    }
    (rules_map, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passing_rule() {
        let data = r"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let (rules, _) = parse_input(data);

        let valid_update = vec![75, 47, 61, 53, 29];
        let result = validate_rule(&rules, &valid_update);
        assert!(result);

        let valid_update2 = vec![97, 61, 53, 29, 13];
        let result = validate_rule(&rules, &valid_update2);
        assert!(result);

        let valid_update3 = vec![75, 29, 13];
        let result = validate_rule(&rules, &valid_update3);
        assert!(result);

        let invalid_update = vec![75, 97, 47, 61, 53];
        let result = validate_rule(&rules, &invalid_update);
        assert!(!result);
    }
}
