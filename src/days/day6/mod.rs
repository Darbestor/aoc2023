use super::common::{Answer, ResultType};

struct Record {
    time: usize,
    distance: usize,
}

pub fn solve() -> Answer {
    let input = include_str!("../../../input/day6.txt");
    let puzzle = parse_input(input);
    Answer {
        day: 6,
        part1: ResultType::Usize(part1(&puzzle)),
        part2: ResultType::Usize(part2(&puzzle)),
    }
}

fn parse_input(input: &str) -> Vec<Record> {
    let mut input = input.split('\n');
    let times: Vec<usize> = input
        .next()
        .map(|line| {
            let line = line.split_once(':').unwrap().1;
            line.split_ascii_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect()
        })
        .unwrap();
    let distances: Vec<usize> = input
        .next()
        .map(|line| {
            let line = line.split_once(':').unwrap().1;
            line.split_ascii_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect()
        })
        .unwrap();
    times
        .iter()
        .zip(distances)
        .map(|(&time, distance)| Record { distance, time })
        .collect()
}

fn part1(input: &[Record]) -> usize {
    input.iter().fold(0, |acc, record| {
        let middle_point = record.time / 2 + record.time % 2;
        let mut ways = 0;
        if record.time % 2 == 0 {
            let traveled = middle_point * middle_point;
            if traveled > middle_point {
                ways += 1;
            }
        }
        for charge_time in 1..middle_point {
            let traveled = charge_time * (record.time - charge_time);
            if traveled > record.distance {
                ways += 2;
            }
        }
        if acc == 0 {
            ways
        } else {
            acc * ways
        }
    })
}

fn part2(input: &[Record]) -> usize {
    let (time, distance) = input
        .iter()
        .fold((String::new(), String::new()), |mut acc, record| {
            acc.0.push_str(&record.time.to_string());
            acc.1.push_str(&record.distance.to_string());
            acc
        });
    let time: usize = time.parse().unwrap();
    let distance: usize = distance.parse().unwrap();

    let middle_point = time / 2 + time % 2;
    let mut ways = 0;
    if time % 2 == 0 {
        let traveled = middle_point * middle_point;
        if traveled > middle_point {
            ways += 1;
        }
    }
    for charge_time in 1..middle_point {
        let traveled = charge_time * (time - charge_time);
        if traveled > distance {
            ways += 2;
        }
    }
    ways
}

#[cfg(test)]
mod day6_tests {
    use super::*;

    static INPUT: &str = r#"Time:      7  15   30
    Distance:  9  40  200"#;

    #[test]
    fn part1_test() {
        assert_eq!(part1(&parse_input(INPUT)), 288);
    }
}
