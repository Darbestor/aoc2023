use std::usize;

use super::common::{Answer, ResultType};

pub fn solve() -> Answer {
    let input = include_str!("../../../input/day5.txt");

    Answer {
        day: 5,
        part1: ResultType::Usize(part1(input)),
        part2: ResultType::Usize(part2(input)),
    }
}

fn part1(input: &str) -> usize {
    let blocks: Vec<&str> = input.split("\n\n").collect();
    let seeds: Vec<usize> = blocks[0]
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|x| {
            x.parse::<usize>()
                .unwrap_or_else(|_| panic!("Cannot parse sequence to usize '{}'", x))
        })
        .collect();
    let results = blocks[1..].iter().fold(seeds, |acc, block| {
        let mut mapped_flags = vec![false; acc.len()];
        block.split('\n').skip(1).fold(acc, |mut acc, line| {
            let mut line = line.split(' ');
            let dest_start = parse_usize(line.next().unwrap());
            let target_start = parse_usize(line.next().unwrap());
            let range = parse_usize(line.next().unwrap());
            let target_end = target_start + range;
            for (i, seed) in acc.iter_mut().enumerate() {
                if !mapped_flags[i] && *seed >= target_start && *seed < target_end {
                    mapped_flags[i] = true;
                    *seed = dest_start + (*seed - target_start);
                }
            }
            acc
        })
    });

    *results.iter().min().unwrap()
}

#[derive(Debug, Clone, Copy)]
struct SeedRange {
    start: usize,
    end: usize,
}

struct Step {
    dest_start: usize,
    target_start: usize,
    range: usize,
}

fn part2(input: &str) -> usize {
    let blocks: Vec<&str> = input.split("\n\n").collect();
    let seeds: Vec<SeedRange> = blocks[0]
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|x| {
            x.parse::<usize>()
                .unwrap_or_else(|_| panic!("Cannot parse sequence to usize '{}'", x))
        })
        .collect::<Vec<usize>>()
        .chunks(2)
        .map(|x| SeedRange {
            start: x[0],
            end: x[0] + x[1] - 1,
        })
        .collect();
    let results = blocks[1..].iter().fold(seeds, |acc, block| {
        let mappings: Vec<Step> = block
            .split('\n')
            .skip(1)
            .map(|line| {
                let mut line = line.split(' ');
                let dest_start = parse_usize(line.next().unwrap());
                let target_start = parse_usize(line.next().unwrap());
                let range = parse_usize(line.next().unwrap());
                Step {
                    dest_start,
                    target_start,
                    range,
                }
            })
            .collect();

        acc.iter()
            .fold(Vec::new(), |mut acc: Vec<SeedRange>, range| {
                let mut mapped = false;
                for map in &mappings {
                    let target_end = map.target_start + map.range;
                    if range.start < target_end && range.end > map.target_start {
                        let start_offset = if range.start > map.target_start {
                            range.start - map.target_start
                        } else {
                            0
                        };
                        let end_offset = if range.end < target_end {
                            target_end - range.end
                        } else {
                            0
                        };
                        acc.push(SeedRange {
                            start: map.dest_start + start_offset,
                            end: map.dest_start + map.range - end_offset,
                        });
                        mapped = true;
                    }
                }
                if !mapped {
                    acc.push(*range);
                }
                acc
            })
    });

    results
        .iter()
        .fold(usize::MAX, |acc, x| usize::min(x.start, acc))
}

fn parse_usize(str: &str) -> usize {
    str.parse::<usize>()
        .unwrap_or_else(|_| panic!("Cannot parse sequence to usize '{}'", str))
}

#[cfg(test)]
mod day5_tests {
    use super::*;

    static INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn part1_test() {
        let result = part1(INPUT);

        assert_eq!(result, 35);
    }

    #[test]
    fn part2_test() {
        let result = part2(INPUT);

        assert_eq!(result, 46);
    }
}
