const DIRECTIONS: [(isize, isize); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

fn check_direction(
    grid: &[Vec<char>],
    key: &str,
    x: usize,
    dx: isize,
    y: usize,
    dy: isize,
) -> Option<isize> {
    let mut string = String::new();

    for step in 0..(key.len() as isize) {
        let row = y.checked_add_signed(step * dy)?;
        let col = x.checked_add_signed(step * dx)?;

        string.push(*grid.get(row)?.get(col)?);
    }

    if string != key {
        None
    } else {
        Some(1)
    }
}

fn check_coord(grid: &[Vec<char>], x: usize, y: usize) -> isize {
    let mut count = 0;

    for (dx, dy) in DIRECTIONS {
        count += check_direction(grid, "XMAS", x, dx, y, dy).unwrap_or(0);
    }

    count
}

pub fn part1(input: &str) -> isize {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();
    let mut count = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            count += check_coord(&grid, x, y);
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const TEST: &str = include_str!("test.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 18)
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(TEST), 2521)
    }
}
