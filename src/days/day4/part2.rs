use crate::days::common::ResultType;

use super::Game;

pub fn solution(input: &[Game]) -> ResultType {
    let mut scratches = vec![1; input.len()];
    input.iter().enumerate().for_each(|(i, game)| {
        if i + 1 >= scratches.len() {
            return;
        }
        let next_card = i + 1;
        let cards = scratches[i];
        let count = usize::min(
            game.winning_numbers.intersection(&game.numbers).count(),
            scratches.len(),
        );
        for num in scratches[next_card..next_card + count].iter_mut() {
            *num += cards;
        }
    });
    ResultType::Usize(scratches.iter().sum())
}
