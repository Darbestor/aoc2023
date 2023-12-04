mod part1;
mod part2;
use std::collections::HashSet;

use super::common::Answer;

struct Game {
    winning_numbers: HashSet<usize>,
    numbers: HashSet<usize>,
}

pub fn solve() -> Answer {
    let input = include_str!("../../../input/day4.txt");

    let input = parse_input(input);
    Answer {
        day: 4,
        part1: part1::solution(&input),
        part2: part2::solution(&input),
    }
}

fn parse_input(input: &str) -> Vec<Game> {
    const SEPARATOR: u8 = '|' as u8;
    input
        .split('\n')
        .map(|line| {
            let card_numbers = line.split_once(':').unwrap().1.as_bytes();
            let mut is_winning_numbers = true;
            let mut winning_numbers: HashSet<usize> = HashSet::new();
            let mut numbers: HashSet<usize> = HashSet::new();
            let mut i = 0;
            while i < card_numbers.len() {
                match card_numbers[i] {
                    SEPARATOR => {
                        is_winning_numbers = false;
                        i += 1;
                    }
                    x if x.is_ascii_digit() => {
                        let mut j = i;
                        while j < card_numbers.len() && card_numbers[j].is_ascii_digit() {
                            j += 1;
                        }
                        let number = std::str::from_utf8(&card_numbers[i..j])
                            .expect("Cannot make str from sequence")
                            .parse()
                            .expect("Cannot parse sequence to usize");
                        if is_winning_numbers {
                            winning_numbers.insert(number);
                        } else {
                            numbers.insert(number);
                        }
                        i += j - i;
                    }
                    _ => i += 1,
                }
            }
            Game {
                numbers,
                winning_numbers,
            }
        })
        .collect()
}

#[cfg(test)]
mod day4_tests {
    use super::parse_input;

    #[test]
    fn test_parse() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        let sut = parse_input(input);

        assert!(sut.len() == 6);
        assert_eq!(
            Vec::from_iter(sut[0].numbers.clone()),
            [83, 86, 6, 31, 17, 9, 48, 53]
        );
        assert_eq!(
            Vec::from_iter(sut[0].winning_numbers.clone()),
            [41, 48, 83, 86, 17]
        );
    }
}
