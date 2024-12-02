use std::num::ParseIntError;

fn parse_line(l: &str) -> impl Iterator<Item = Result<isize, ParseIntError>> + '_ {
    l.split_whitespace().map(|n| n.parse::<isize>())
}

fn safe_reading(vec: &[isize]) -> bool {
    if vec.len() < 2 {
        return true;
    }

    let increasing = vec[1] > vec[0];

    for (prev, cur) in vec.iter().zip(vec.iter().skip(1)) {
        if increasing != (cur > prev) {
            return false;
        }

        let delta = (cur - prev).abs();
        if !(1..=3).contains(&delta) {
            return false;
        }
    }

    true
}

pub fn part1(input: &str) -> Result<isize, ParseIntError> {
    let safe = input
        .lines()
        .map(|l| parse_line(l).collect::<Result<Vec<_>, _>>())
        .map(|r| r.map(|v| safe_reading(&v)))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(safe.iter().filter(|b| **b).count() as isize)
}

fn safe_reading_if_dampened(vec: &[isize]) -> bool {
    if safe_reading(vec) {
        return true;
    }

    for i in 0..vec.len() {
        let dampened = [&vec[..i], &vec[i + 1..]].concat();

        if safe_reading(&dampened) {
            return true;
        }
    }

    false
}

pub fn part2(input: &str) -> Result<isize, ParseIntError> {
    let safe = input
        .lines()
        .map(|l| parse_line(l).collect::<Result<Vec<_>, _>>())
        .map(|r| r.map(|v| safe_reading_if_dampened(&v)))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(safe.iter().filter(|b| **b).count() as isize)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const TEST: &str = include_str!("test.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), Ok(2));
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST), Ok(670));
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), Ok(4));
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST), Ok(700));
    }
}
