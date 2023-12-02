use super::common::{Answer, ResultType};

struct GameSet {
    green: usize,
    red: usize,
    blue: usize,
}

struct Game {
    id: usize,
    sets: Vec<GameSet>,
}

impl Game {
    fn new(id: usize) -> Game {
        Game { id, sets: vec![] }
    }
}

impl GameSet {
    fn new() -> GameSet {
        GameSet {
            green: 0,
            red: 0,
            blue: 0,
        }
    }
}

pub fn solve() -> Answer {
    let input = include_str!("../../../input/day2.txt");
    let games: Vec<Game> = parse_input(input);

    Answer {
        day: 2,
        part1: part1(&games),
        part2: part2(&games),
    }
}

fn parse_input(input: &str) -> Vec<Game> {
    input
        .split('\n')
        .enumerate()
        .map(|(i, line)| {
            let (_, game_sets) = line.split_once(':').unwrap();
            let mut game = Game::new(i + 1);
            game_sets.split(';').for_each(|set| {
                let mut game_set = GameSet::new();
                set.trim_start().split(", ").for_each(|x| {
                    let (balls, color) = x.split_once(' ').unwrap();
                    let balls = balls.parse::<usize>().unwrap();
                    match color {
                        "green" => game_set.green = balls,
                        "blue" => game_set.blue = balls,
                        "red" => game_set.red = balls,
                        _ => unimplemented!(),
                    }
                });
                game.sets.push(game_set);
            });
            game
        })
        .collect()
}

fn part1(input: &[Game]) -> ResultType {
    const RED_LIMIT: usize = 12;
    const GREEN_LIMIT: usize = 13;
    const BLUE_LIMIT: usize = 14;
    ResultType::Usize(input.into_iter().fold(0, |acc, game| {
        if game
            .sets
            .iter()
            .all(|x| x.blue <= BLUE_LIMIT && x.red <= RED_LIMIT && x.green <= GREEN_LIMIT)
        {
            acc + game.id
        } else {
            acc
        }
    }))
}

fn part2(input: &[Game]) -> ResultType {
    ResultType::Usize(input.into_iter().fold(0, |acc, game| {
        let set = game.sets.iter().fold(GameSet::new(), |mut acc, s| {
            acc.blue = usize::max(acc.blue, s.blue);
            acc.red = usize::max(acc.red, s.red);
            acc.green = usize::max(acc.green, s.green);
            acc
        });
        acc + set.blue * set.green * set.red
    }))
}
