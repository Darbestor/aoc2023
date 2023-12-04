pub mod day4;
pub mod day3;
pub mod day1;
pub mod day2;

pub mod common {
    use std::fmt::Display;

    #[derive(PartialEq)]
    pub enum ResultType {
        Usize(usize),
        String(String),
        None,
    }

    pub struct Answer {
        pub day: usize,
        pub part1: ResultType,
        pub part2: ResultType,
    }

    impl Display for ResultType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let answer = match self {
                ResultType::Usize(x) => x.to_string(),
                ResultType::String(x) => x.clone(),
                ResultType::None => "Not implemented".to_string(),
            };
            write!(f, "{}", answer)
        }
    }

    impl Display for Answer {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(
                f,
                r#"Day {}:
    Part 1: {}
    Part 2: {}"#,
                self.day, self.part1, self.part2
            )
        }
    }
}
