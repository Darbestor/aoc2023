const TOTAL_DAYS: usize = 1;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => run_all_days(),
        2 => help(),
        3 => {
            let day = args[2].parse::<usize>();
            if let Ok(n) = day {
                if n <= TOTAL_DAYS {
                    run_day(n);
                }
                panic!("Day must be less than {TOTAL_DAYS}");
            }
            panic!("Day must be in range between 1 and {TOTAL_DAYS}");
        }
        _ => help(),
    }
    println!("Hello, world!");
}

fn run_all_days() {
    todo!()
}

fn run_day(day: usize) {
    todo!()
}

fn help() {
    println!(
        r#"Usage: aoc2023 [command] [option]
command: day [option]
option: 1..1
"#
    );
}
