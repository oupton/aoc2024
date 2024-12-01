use std::collections::HashMap;

fn parse_lists(input: &str) -> Option<(Vec<isize>, Vec<isize>)> {
    let mut lhs = Vec::new();
    let mut rhs = Vec::new();

    for l in input.lines() {
        let nums = l
            .split_whitespace()
            .map(|n| n.parse::<isize>())
            .collect::<Result<Vec<_>, _>>()
            .ok()?;

        if nums.len() != 2 {
            return None;
        }

        lhs.push(nums[0]);
        rhs.push(nums[1]);
    }

    Some((lhs, rhs))
}

pub fn part1(input: &str) -> Option<isize> {
    let (mut lhs, mut rhs) = parse_lists(input)?;

    lhs.sort();
    rhs.sort();

    let distance = lhs.iter().zip(rhs.iter()).map(|(l, r)| (l - r).abs()).sum();
    Some(distance)
}

pub fn part2(input: &str) -> Option<isize> {
    let (lhs, rhs) = parse_lists(input)?;
    let mut appearances = HashMap::<isize, isize>::new();

    for n in rhs {
        *appearances.entry(n).or_insert(0) += 1;
    }

    let similarity = lhs
        .iter()
        .map(|n| n * appearances.get(n).unwrap_or(&0))
        .sum();
    Some(similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const TEST: &str = include_str!("test.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), Some(11))
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST), Some(2000468))
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), Some(31))
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST), Some(18567089))
    }
}
