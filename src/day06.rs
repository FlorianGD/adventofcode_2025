use crate::helpers::transpose;
use itertools::Itertools;

type Input = (Vec<Vec<usize>>, Vec<Opt>);

#[derive(Debug, PartialEq, Eq)]
pub enum Opt {
    Add,
    Mult,
}

impl TryFrom<char> for Opt {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Add),
            '*' => Ok(Self::Mult),
            _ => Err("Unknown character"),
        }
    }
}

pub fn parse_input(input: &str) -> Input {
    // We reverse so can specialize the first input
    let mut lines = input.lines().rev();
    let opts = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|c| c.chars().next().unwrap().try_into().unwrap())
        .collect();

    let values = lines
        .map(|l| l.split_whitespace().map(|v| v.parse().unwrap()).collect())
        .collect();
    (transpose(values), opts)
}

pub fn part1((values, opts): Input) -> usize {
    values
        .into_iter()
        .zip(opts)
        .map(|(the_values, opt)| match opt {
            Opt::Add => the_values.into_iter().reduce(|acc, v| acc + v).unwrap(),
            Opt::Mult => the_values.into_iter().reduce(|acc, v| acc * v).unwrap(),
        })
        .sum()
}

/// column contains the values from the column, we align them and convert them to
/// numbers.
fn column_to_numbers(column: &[&str]) -> Vec<usize> {
    // We know that all the Strings have the same length
    let length = column[0].len();
    let mut numbers: Vec<String> = vec![String::new(); length];
    for val in column {
        for (i, c) in val.chars().enumerate() {
            if c != ' '
                && let Some(elem) = numbers.get_mut(i)
            {
                *elem = format!("{}{}", &c, &elem);
            }
        }
    }
    numbers.iter().map(|x| x.parse().unwrap()).collect()
}

pub fn parse_input_p2(input: &str) -> Input {
    // We reverse so can specialize the first input
    let mut lines = input.lines().rev();
    let opts_line = lines.next().unwrap();
    let mut opts = Vec::new();
    let mut ranges = Vec::new();
    for (i, c) in opts_line.chars().enumerate() {
        if let Ok(opt) = c.try_into() {
            opts.push(opt);
            ranges.push(i);
        }
    }
    ranges.push(opts_line.len() + 1);

    let mut columns: Vec<Vec<&str>> = Vec::new();
    for line in lines {
        for (i, (&start, &end)) in ranges.iter().tuple_windows().enumerate() {
            match columns.get_mut(i) {
                Some(elem) => elem.push(&line[start..end - 1]),
                None => columns.push(vec![&line[start..end - 1]]),
            }
        }
    }

    (
        columns.iter().map(|col| column_to_numbers(col)).collect(),
        opts,
    )
}

#[cfg(test)]
mod tests {
    use super::Opt::*;
    use super::*;
    use assert2::check;
    use indoc::indoc;

    const INPUT: &str = indoc! {
        "\
        123 328  51 64 
         45 64  387 23 
          6 98  215 314
        *   +   *   +  "
    };

    #[test]
    fn test_parse_input() {
        let (values, opts) = parse_input(INPUT);
        check!(values[0] == vec![6, 45, 123]);
        check!(values.len() == 4);
        check!(opts == vec![Mult, Add, Mult, Add]);
    }

    #[test]
    fn test_part1() {
        let input = parse_input(INPUT);
        check!(part1(input) == 4277556);
    }

    #[test]
    fn test_parse_input_p2() {
        let (values, opts) = parse_input_p2(INPUT);
        let (_, opts_p1) = parse_input(INPUT);
        check!(opts == opts_p1);
        check!(values[0] == vec![1, 24, 356]);
    }

    #[test]
    fn test_column_to_numbers() {
        // the order is reversed because we go through the lines in reverse order
        let column = vec!["  6", " 45", "123"];
        check!(column_to_numbers(&column) == vec![1, 24, 356]);
    }
    #[test]
    fn test_part2() {
        let input = parse_input_p2(INPUT);
        check!(part1(input) == 3263827);
    }
}
