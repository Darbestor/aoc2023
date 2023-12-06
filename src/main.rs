const TOTAL_DAYS: usize = 6;
pub mod days;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => {
            run_all_days();
        }
        2 => help(),
        3 => {
            let day = args[2].parse::<usize>();
            if let Ok(n) = day {
                if n <= TOTAL_DAYS {
                    run_day(n);
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
    for day in 1..=TOTAL_DAYS {
        run_day(day);
    }
}

fn run_day(day: usize) {
    let answer = match day {
        1 => days::day1::solve(),
        2 => days::day2::solve(),
        3 => days::day3::solve(),
        4 => days::day4::solve(),
        5 => days::day5::solve(),
        6 => days::day6::solve(),
        _ => unimplemented!(),
    };
    println!("{}", answer);
}

fn help() {
    println!(
        r#"Usage: aoc2023 [command] [option]
command: day [option]
option: 1..1
"#
    );
}
