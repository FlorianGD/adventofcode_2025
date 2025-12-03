type Input = Vec<Vec<u32>>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

pub fn part1(input: Input) -> u32 {
    input
        .into_iter()
        .map(|v| {
            let mut current_max = 0;
            for (i, ten) in v.iter().rev().skip(1).rev().enumerate() {
                if 10 * ten < current_max {
                    continue;
                }
                for unit in v.iter().skip(i + 1) {
                    current_max = current_max.max(10 * ten + unit);
                }
            }
            current_max
        })
        .sum()
}

type Input2 = Vec<String>;

pub fn parse_input_p2(input: &str) -> Input2 {
    input.lines().map(|line| line.to_string()).collect()
}

/// First biggest digit in values, excluding the last `remaining` characters
fn find_next_digit(values: &str, remaining: usize) -> (char, &str) {
    let slice = &values[0..values.len() - remaining];
    let max = slice.chars().max().unwrap();
    let first_idx = slice.chars().position(|c| c == max).unwrap();
    (max, &values[(first_idx + 1)..values.len()])
}

pub fn part2(input: Input2) -> usize {
    input
        .into_iter()
        .map(|v| {
            let mut res = String::new();
            let mut view = v.as_str();
            for i in (0..12).rev() {
                let (c, new_view) = find_next_digit(view, i);
                view = new_view;
                res.push(c);
            }
            res.parse::<usize>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const INPUT: &str = indoc! {
        "987654321111111
        811111111111119
        234234234234278
        818181911112111"
    };

    #[test]
    fn test_parse_input() {
        let parsed = parse_input(INPUT);
        assert_eq!(parsed.len(), 4);
        assert_eq!(parsed[0], vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]);
    }
    #[test]
    fn test_part1() {
        let parsed = parse_input(INPUT);
        assert_eq!(part1(parsed), 357);
    }
    #[test]
    fn test_part2() {
        let parsed = parse_input_p2(INPUT);
        assert_eq!(part2(parsed), 3121910778619);
    }
}
