mod part1;
mod part2;
use super::common::Answer;

pub fn solve() -> Answer {
    let input = include_str!("../../../input/day3.txt");

    let input: Vec<&[u8]> = input.split('\n').map(|x| x.as_bytes()).collect();
    Answer {
        day: 3,
        part1: part1::solution(&input),
        part2: part2::solution(&input),
    }
}
