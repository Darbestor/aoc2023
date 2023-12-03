use super::common::{Answer, ResultType};

pub fn solve() -> Answer {
    let input = include_str!("../../../input/day3.txt");

    let input: Vec<&[u8]> = input.split('\n').map(|x| x.as_bytes()).collect();
    Answer {
        day: 3,
        part1: part1(&input),
        part2: ResultType::None,
    }
}

fn part1(input: &[&[u8]]) -> ResultType {
    const DOT_ASCII: u8 = '.' as u8;
    let mut bool_map: Vec<Vec<bool>> = input.iter().map(|x| vec![false; x.len()]).collect();
    let mut result = 0;

    for (i, &line) in input.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if !ch.is_ascii_digit() && *ch != DOT_ASCII {
                fill_true(&mut bool_map, i, j);
            }
        }
    }
    for (i, &line) in input.iter().enumerate() {
        let mut numbers = Vec::new();
        let mut adjecent = false;
        for (j, &ch) in line.iter().enumerate() {
            if ch.is_ascii_digit() {
                numbers.push(ch);
                adjecent = match adjecent {
                    true => true,
                    false => bool_map[i][j],
                };
            } else if adjecent {
                adjecent = false;
                result += String::from_utf8(numbers)
                    .expect("invalid utf-8 sequence")
                    .parse::<usize>()
                    .expect("Cannot parse sequence to usize");
                numbers = Vec::new();
            } else {
                if numbers.len() > 0 {
                    numbers.clear();
                }
            }
        }
        if adjecent {
            result += String::from_utf8(numbers)
                .expect("invalid utf-8 sequence")
                .parse::<usize>()
                .expect("Cannot parse sequence to usize");
        }
    }

    ResultType::Usize(result)
}

fn fill_true(bool_map: &mut [Vec<bool>], row: usize, col: usize) {
    let next_col = usize::min(col + 1, bool_map[row].len() - 1);
    let prev_col = match col {
        0 => 0,
        c => c - 1,
    };
    let next_row = usize::min(row + 1, bool_map.len() - 1);
    let prev_row = match row {
        0 => 0,
        r => r - 1,
    };
    bool_map[row][col] = true;
    bool_map[row][prev_col] = true;
    bool_map[row][next_col] = true;
    bool_map[prev_row][col] = true;
    bool_map[prev_row][next_col] = true;
    bool_map[prev_row][prev_col] = true;
    bool_map[next_row][col] = true;
    bool_map[next_row][next_col] = true;
    bool_map[next_row][prev_col] = true;
}
