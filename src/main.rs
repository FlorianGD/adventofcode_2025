use adventofcode2025::day01;

use anyhow::Result;
use aoc_next::{Aoc, aoc_main, parser, solution, solver};

const AOC: Aoc = Aoc {
    allow_download: true,
    year: 2025,
    solutions: &[
        solution! {1, parser!{ day01::parse_input }, solver!{ day01::part1 }},
        solution! {1, parser!{ day01::parse_input }, solver!{ day01::part2 }},
    ],
};

pub fn main() -> Result<()> {
    aoc_main(AOC)
}
