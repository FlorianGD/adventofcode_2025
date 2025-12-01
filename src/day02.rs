use std::ops::RangeInclusive;

pub fn parse_input(input: &str) -> Vec<RangeInclusive<usize>> {
    input
        .trim()
        .split(',')
        .map(|part| match part.split_once('-') {
            Some((l1, l2)) => l1.parse().expect("Not a number")..=l2.parse().expect("Not a number"),
            _ => panic!("Not a range"),
        })
        .collect()
}

fn find_closest_duplicates(n: usize) -> usize {
    let mut num_digits = n.to_string().len();
    if !num_digits.is_multiple_of(2) {
        num_digits += 1;
    }
    let mult: usize = (10u32.pow((num_digits / 2).try_into().unwrap()) + 1)
        .try_into()
        .unwrap();
    let min: usize = 10u32
        .pow((num_digits / 2 - 1).try_into().unwrap())
        .try_into()
        .unwrap();
    if n <= min * mult {
        min * mult
    } else if n.is_multiple_of(mult) {
        n
    } else {
        n + (mult - n % mult)
    }
}

pub fn part1(input: Vec<RangeInclusive<usize>>) -> usize {
    input
        .into_iter()
        .map(|range| {
            let mut total = 0;
            let mut next_mult = find_closest_duplicates(*range.start());
            while next_mult <= *range.end() {
                total += next_mult;
                next_mult = find_closest_duplicates(next_mult + 1);
            }
            total
        })
        .sum()
}

fn possible_next(n: usize, mult: usize, min: usize) -> usize {
    if n <= min * mult {
        min * mult
    } else {
        n.div_ceil(mult) * mult
    }
}

fn find_closest_duplicates_part2(n: usize) -> usize {
    // 1 digit cannot have a duplicate, skip to 2
    let num_digits = n.to_string().len().max(2);
    let mut possible: Vec<usize> = Vec::new();
    if !num_digits.is_multiple_of(2) {
        // Could be in the form 111
        // Or 1001001 with 3 digits in case of 9 digits
        // We do not have more than 10 digits, so it will be good enough
        let mult = "1".repeat(num_digits).parse().unwrap();
        let min = 1;
        possible.push(possible_next(n, mult, min));
        if num_digits == 9 {
            possible.push(possible_next(n, 1001001, 100));
        }
    } else {
        // For 8 digits, we can have duplicates in several forms:
        // multiple of 11111111 with one digit
        // multiple of 1010101 with 2 digits
        // multiple of 10001 with 4 digits
        let mult: usize = "1".repeat(num_digits).parse().unwrap();
        let min: usize = 1;
        possible.push(possible_next(n, mult, min));
        let mut i = 2;
        while num_digits / i > 1 {
            if !num_digits.is_multiple_of(i) {
                i += 1;
                continue;
            }
            let base = format!("1{}", "0".repeat(i - 1));
            let mult: usize = format!("{}1", base.repeat(num_digits / i - 1))
                .parse()
                .unwrap();
            let min: usize = base.parse().unwrap();
            possible.push(possible_next(n, mult, min));
            i += 1;
        }
    }
    possible.into_iter().min().unwrap()
}

pub fn part2(input: Vec<RangeInclusive<usize>>) -> usize {
    input
        .into_iter()
        .map(|range| {
            let mut total = 0;
            let mut next_mult = find_closest_duplicates_part2(*range.start());
            while next_mult <= *range.end() {
                total += next_mult;
                next_mult = find_closest_duplicates_part2(next_mult + 1);
            }
            total
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_parse_input() {
        let parsed = parse_input(INPUT);
        assert_eq!(parsed[0..2], vec![11usize..=22, 95..=115]);
        assert_eq!(parsed.len(), 11);
    }

    #[test]
    fn test_find_closest_duplicates() {
        assert_eq!(find_closest_duplicates(5), 11);
        assert_eq!(find_closest_duplicates(11), 11);
        assert_eq!(find_closest_duplicates(21), 22);
        assert_eq!(find_closest_duplicates(100), 1010);
        assert_eq!(find_closest_duplicates(999), 1010);
        assert_eq!(find_closest_duplicates(1000), 1010);
        assert_eq!(find_closest_duplicates(1000000000), 1000010000);
    }

    #[test]
    fn test_part1() {
        let parsed = parse_input(INPUT);
        assert_eq!(part1(parsed), 1227775554);
    }

    #[test]
    fn test_find_closest_duplicates_part2() {
        assert_eq!(find_closest_duplicates_part2(5), 11);
        assert_eq!(find_closest_duplicates_part2(11), 11);
        assert_eq!(find_closest_duplicates_part2(21), 22);
        assert_eq!(find_closest_duplicates_part2(100), 111);
        assert_eq!(find_closest_duplicates_part2(999), 999);
        assert_eq!(find_closest_duplicates_part2(1000), 1010);
        assert_eq!(find_closest_duplicates_part2(824824821), 824824824);
        assert_eq!(find_closest_duplicates_part2(2121212118), 2121212121);
        assert_eq!(find_closest_duplicates_part2(1000000000), 1000010000);
    }

    #[test]
    fn test_part2() {
        let parsed = parse_input(INPUT);
        assert_eq!(part2(parsed), 4174379265);
    }
}
