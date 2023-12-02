use super::common::{Answer, ResultType};

const ASCII_ZERO: u8 = 48;

pub fn solve() -> Answer {
    let input = include_str!("../../../input/day1.txt");
    Answer {
        day: 1,
        part1: part1(input),
        part2: part2(input),
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

static NUMBERS: [&[u8]; 9] = [
    "one".as_bytes(),
    "two".as_bytes(),
    "three".as_bytes(),
    "four".as_bytes(),
    "five".as_bytes(),
    "six".as_bytes(),
    "seven".as_bytes(),
    "eight".as_bytes(),
    "nine".as_bytes(),
];

fn part2(input: &str) -> ResultType {
    ResultType::Usize(
        input
            .split('\n')
            .map(|line| {
                let numbers = find_numbers(line);
                numbers.last().unwrap() + (numbers.first().unwrap() * 10) as usize
            })
            .sum(),
    )
}

fn find_numbers(str: &str) -> Vec<usize> {
    let mut numbers = Vec::new();
    let str = str.as_bytes();
    for i in 0..str.len() {
        if str[i].is_ascii_digit() {
            numbers.push((str[i] - ASCII_ZERO) as usize);
        }
        for (number, &num_str) in NUMBERS.iter().enumerate() {
            let mut j = 0;
            for &ch in num_str {
                let next = i + j;
                if next >= str.len() || ch != str[next] {
                    break;
                }
                j += 1;
            }
            if j == num_str.len() {
                numbers.push(number + 1);
            }
        }
    }
    numbers
}
