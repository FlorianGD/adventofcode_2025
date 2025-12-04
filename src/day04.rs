use num::Complex;
use rustc_hash::FxHashSet as HashSet;

type Pos = Complex<isize>;
type Input = HashSet<Pos>;

const DIRECTIONS: [Complex<isize>; 8] = [
    Complex::new(0, 1),
    Complex::new(0, -1),
    Complex::new(1, 0),
    Complex::new(1, 1),
    Complex::new(1, -1),
    Complex::new(-1, 0),
    Complex::new(-1, 1),
    Complex::new(-1, -1),
];

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .enumerate()
        .flat_map(|(j, line)| {
            line.chars().enumerate().filter_map(move |(i, c)| match c {
                '@' => Some(Complex::new(i.try_into().unwrap(), j.try_into().unwrap())),
                _ => None,
            })
        })
        .collect()
}

fn num_neighbors(roll: &Pos, input: &Input) -> usize {
    DIRECTIONS
        .iter()
        .filter(|&d| input.contains(&(roll + d)))
        .count()
}

pub fn part1(input: Input) -> usize {
    input
        .iter()
        .map(|roll| num_neighbors(roll, &input))
        .filter(|v| v < &4)
        .count()
}

pub fn part2(mut input: Input) -> usize {
    let initial_len = input.len();
    let mut prev_len = 0;
    while input.len() != prev_len {
        prev_len = input.len();
        input = input
            .clone()
            .into_iter()
            .filter(|roll| num_neighbors(roll, &input) >= 4)
            .collect();
    }
    initial_len - input.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const INPUT: &str = indoc! {
        "..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
        "
    };

    #[test]
    fn test_parse_input() {
        let parsed = parse_input(INPUT);
        assert!(!parsed.contains(&Complex::new(0, 0)));
        assert!(parsed.contains(&Complex::new(2, 0)));
    }

    #[test]
    fn test_part1() {
        let parsed = parse_input(INPUT);
        assert_eq!(part1(parsed), 13);
    }

    #[test]
    fn test_part2() {
        let parsed = parse_input(INPUT);
        assert_eq!(part2(parsed), 43);
    }
}
