use days::common::Answer;

const TOTAL_DAYS: usize = 1;
pub mod days;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => {
            let results = run_all_days();
        }
        2 => help(),
        3 => {
            let day = args[2].parse::<usize>();
            if let Ok(n) = day {
                if n <= TOTAL_DAYS {
                    print!("{}", run_day(n));
                } else {
                    panic!("Day must be less than {TOTAL_DAYS}");
                }
            } else {
                panic!("Day must be in range between 1 and {TOTAL_DAYS}");
            }
        }
        _ => help(),
    }
}

fn run_all_days() {
    todo!()
}

fn run_day(day: usize) -> Answer {
    match day {
        1 => days::day1::solve(),
        _ => unimplemented!(),
    }
}

fn help() {
    println!(
        r#"Usage: aoc2023 [command] [option]
command: day [option]
option: 1..1
"#
    );
}
