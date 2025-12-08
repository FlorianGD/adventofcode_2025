use num::{Complex, Zero};
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

type Pos = Complex<isize>;
type Grid = HashSet<Pos>;
type Input = (Pos, Grid);

pub fn parse_input(input: &str) -> Input {
    let mut start: Pos = Complex::zero();
    let mut grid = Grid::default();
    for (j, line) in input.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            match c {
                'S' => start = Complex::new(i.try_into().unwrap(), j.try_into().unwrap()),
                '^' => {
                    grid.insert(Complex::new(i.try_into().unwrap(), j.try_into().unwrap()));
                }
                _ => continue,
            };
        }
    }
    (start, grid)
}

pub fn part1((start, grid): Input) -> usize {
    let im_max = grid.iter().map(|c| c.im).max().unwrap();
    let mut total_split = 0;
    let mut current_pos = HashSet::from_iter([start]);
    for y in 1..=im_max {
        let mut new_positions = HashSet::default();
        for pos in current_pos {
            let new_pos = Complex::new(pos.re, y);
            if grid.contains(&new_pos) {
                new_positions.insert(new_pos + Complex::new(1, 0)); // to the right
                new_positions.insert(new_pos + Complex::new(-1, 0)); // to the left
                total_split += 1;
            } else {
                new_positions.insert(new_pos);
            }
        }
        current_pos = new_positions;
    }
    total_split
}

pub fn part2((start, grid): Input) -> usize {
    let im_max = grid.iter().map(|c| c.im).max().unwrap();

    let mut current_pos: HashMap<Pos, usize> = HashMap::from_iter([(start, 1)]);
    for y in 1..=im_max {
        let mut new_positions = HashMap::default();
        for (pos, value) in current_pos {
            let new_pos = Complex::new(pos.re, y);
            if grid.contains(&new_pos) {
                let right = new_pos + Complex::new(1, 0);
                let left = new_pos + Complex::new(-1, 0);

                new_positions
                    .entry(right)
                    .and_modify(|e| *e += value)
                    .or_insert(value);
                new_positions
                    .entry(left)
                    .and_modify(|e| *e += value)
                    .or_insert(value);
            } else {
                new_positions
                    .entry(new_pos)
                    .and_modify(|e| *e += value)
                    .or_insert(value);
            }
        }
        current_pos = new_positions;
    }
    current_pos.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;
    use indoc::indoc;

    const INPUT: &str = indoc! {
        ".......S.......
        ...............
        .......^.......
        ...............
        ......^.^......
        ...............
        .....^.^.^.....
        ...............
        ....^.^...^....
        ...............
        ...^.^...^.^...
        ...............
        ..^...^.....^..
        ...............
        .^.^.^.^.^...^.
        ..............."
    };

    #[test]
    fn test_parse_input() {
        let (start, grid) = parse_input(INPUT);
        check!(start == Complex::new(7, 0));
        check!(grid.contains(&Complex::new(7, 2)));
        check!(!grid.contains(&Complex::new(7, 1)));
    }

    #[test]
    fn test_part2() {
        let input = parse_input(INPUT);
        check!(part2(input) == 40);
    }
}
