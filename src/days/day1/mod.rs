use super::common::{Answer, ResultType};

const ASCII_ZERO: u8 = 48;

pub fn solve() -> Answer {
    let input = include_str!("../../../input/day1.txt");
    Answer {
        day: 1,
        part1: part1(input),
        part2: ResultType::None,
    }
}

fn part1(input: &str) -> ResultType {
    ResultType::Usize(
        input
            .split('\n')
            .map(|line| {
                let numbers_raw: Vec<u8> = line.bytes().filter(|c| c.is_ascii_digit()).collect();
                (numbers_raw.last().unwrap() - ASCII_ZERO
                    + ((numbers_raw.first().unwrap() - ASCII_ZERO) * 10)) as usize
            })
            .sum(),
    )
}
