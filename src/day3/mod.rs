use regex::Regex;

pub fn part1(input: &str) -> Option<isize> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").ok()?;
    let mut sum = 0;

    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        let x = a.parse::<isize>().ok()?;
        let y = b.parse::<isize>().ok()?;

        sum += x * y;
    }

    Some(sum)
}

pub fn part2(input: &str) -> Option<isize> {
    let parts = input.split("don't()").collect::<Vec<_>>();

    // instructions are enabled at the start of the program.
    let mut sum = part1(parts[0])?;

    for s in &parts[1..] {
        let enabled = s.split("do()").collect::<Vec<_>>();

        // instructions are disabled in the first subsection,
        // and enabled in subsequent sections should 'do()' be
        // repeated.
        let vals = &enabled[1..]
            .iter()
            .map(|c| part1(c))
            .collect::<Option<Vec<_>>>()?;

        sum += vals.iter().sum::<isize>();
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const EXAMPLE2: &str = include_str!("example2.txt");
    const TEST: &str = include_str!("test.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), Some(161));
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST), Some(184576302));
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE2), Some(48));
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(TEST), Some(118173507));
    }
}
