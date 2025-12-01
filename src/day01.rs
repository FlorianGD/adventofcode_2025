pub fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();

            let prefix = match chars.next() {
                Some('R') => 1,
                Some('L') => -1,
                _ => panic!("Unexpected first value"),
            };
            let rest: i32 = chars.as_str().parse().unwrap();
            rest * prefix
        })
        .collect()
}

pub fn part1(input: Vec<i32>) -> i32 {
    let mut pos = 50;
    let mut result = 0;
    for value in input {
        pos += value;
        pos = pos.rem_euclid(100);
        if pos == 0 {
            result += 1;
        }
    }
    result
}

pub fn part2(input: Vec<i32>) -> i32 {
    let mut pos = 50;
    let mut result = 0;
    for value in input {
        let prev_pos = pos;
        let quotient = value / 100;
        let remainder = value % 100;
        pos += remainder;
        result += quotient.abs();
        if prev_pos != 0 && (pos >= 100 || pos <= 0) {
            result += 1;
        }
        pos = pos.rem_euclid(100);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    const INPUT: &str = indoc! {
    "L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82
    R1000
    L1500
    "};

    #[test]
    fn test_part2() {
        let input = parse_input(INPUT);
        let result = part2(input);
        assert_eq!(result, 31);
    }
}
