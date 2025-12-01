use adventofcode2025::{day01, day02};

use anyhow::Result;
use aoc_next::{Aoc, aoc_main, parser, solution, solver};

const AOC: Aoc = Aoc {
    allow_download: true,
    year: 2025,
    solutions: &[
        solution! {1, parser!{ day01::parse_input }, solver!{ day01::part1 }},
        solution! {1, parser!{ day01::parse_input }, solver!{ day01::part2 }},
        solution! {2, parser!{ day02::parse_input }, solver!{ day02::part1 }},
        solution! {2, parser!{ day02::parse_input }, solver!{ day02::part2 }},
    ],
};

pub fn main() -> Result<()> {
    aoc_main(AOC)
}
