use crate::days::common::ResultType;

pub fn solution(input: &[&[u8]]) -> ResultType {
    const ENGINE_PART_ASCII: u8 = '*' as u8;
    let mut result = 0;

    for (i, &line) in input.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if *ch == ENGINE_PART_ASCII {
                let numbers = get_surround_numbers(input, i, j);
                if numbers.len() == 2 {
                    result += numbers[0] * numbers[1];
                }
            }
        }
    }

    ResultType::Usize(result)
}

fn get_surround_numbers(map: &[&[u8]], row: usize, col: usize) -> Vec<usize> {
    let mut numbers = get_line_numbers(map[row], col);
    if row > 0 {
        numbers.append(&mut get_line_numbers(map[row - 1], col));
    }
    if row < map.len() {
        numbers.append(&mut get_line_numbers(map[row + 1], col));
    }
    numbers
}

fn get_line_numbers(line: &[u8], col: usize) -> Vec<usize> {
    let mut numbers = Vec::new();
    let mut middle_is_number = false;
    if line[col].is_ascii_digit() {
        middle_is_number = true;
        numbers.push(parse_number(line, col));
    }
    if col > 0 && !middle_is_number && line[col - 1].is_ascii_digit() {
        numbers.push(parse_number(line, col - 1));
    }
    if col < line.len() && !middle_is_number && line[col + 1].is_ascii_digit() {
        numbers.push(parse_number(line, col + 1));
    }
    numbers
}

fn parse_number(line: &[u8], index: usize) -> usize {
    let mut start = index;
    let mut end = index;
    while start > 0 {
        if !line[start - 1].is_ascii_digit() {
            break;
        }
        start -= 1;
    }
    while end < line.len() - 1 {
        if !line[end + 1].is_ascii_digit() {
            break;
        }
        end += 1;
    }

    std::str::from_utf8(&line[start..=end])
        .expect("invalid utf-8 sequence")
        .parse::<usize>()
        .expect("Cannot parse sequence to usize")
}
