use crate::days::common::ResultType;

use super::Game;

pub fn solution(input: &[Game]) -> ResultType {
    ResultType::Usize(input.iter().fold(0, |acc, game| {
        let count = game.winning_numbers.intersection(&game.numbers).count();
        if count > 0 {
            acc + usize::pow(2, (count - 1) as u32)
        } else {
            acc
        }
    }))
}
