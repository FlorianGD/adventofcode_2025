use std::ops::RangeInclusive;

type Ranges = Vec<RangeInclusive<usize>>;
type Input = (Ranges, Vec<usize>);

pub fn parse_input(input: &str) -> Input {
    let (p1, p2) = input.split_once("\n\n").expect("not 2 parts");
    let mut ranges: Ranges = p1
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            start.parse::<usize>().unwrap()..=end.parse().unwrap()
        })
        .collect();
    let numbers = p2.lines().map(|v| v.parse().unwrap()).collect();
    // we sort and reverse the order so when we pop, we get the minimum range start
    ranges.sort_by(|r1, r2| r2.start().cmp(r1.start()));
    (ranges, numbers)
}

pub fn part1((ranges, numbers): Input) -> usize {
    numbers
        .into_iter()
        .filter(|n| ranges.iter().any(|r| r.contains(n)))
        .count()
}

pub fn part2((mut ranges, _): Input) -> usize {
    let mut merged_ranges_size = 0;
    let mut previous = ranges.pop().unwrap();
    while let Some(range) = ranges.pop() {
        if previous.end() < range.start() {
            merged_ranges_size += previous.count();
            previous = range;
        } else {
            let r = *previous.start()..=*(previous.end().max(range.end()));
            previous = r;
        }
    }
    merged_ranges_size + previous.count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;
    use indoc::indoc;

    const INPUT: &str = indoc! {
        "3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32"
    };

    #[test]
    fn test_parse_input() {
        let (ranges, numbers) = parse_input(INPUT);
        check!(ranges[ranges.len() - 1].start() == &3);
        check!(ranges[ranges.len() - 1].end() == &5);
        check!(numbers == vec![1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn test_part1() {
        let (ranges, numbers) = parse_input(INPUT);
        check!(part1((ranges, numbers)) == 3);
    }

    #[test]
    fn test_part2() {
        let (ranges, _numbers) = parse_input(INPUT);
        check!(part2((ranges, _numbers)) == 14);
    }
}
