use std::{fmt::{Display, Write}, str::FromStr};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ChallengePart {
    Gold,
    Silver,
}

impl FromStr for ChallengePart {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let matched = match s {
            "g" | "G" | "gold" | "Gold" => Some(ChallengePart::Gold),
            "s" | "S" | "silver" | "Silver" => Some(ChallengePart::Silver),
            _ => None,
        };

        matched.ok_or(s.into())
    }
}

impl Display for ChallengePart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            ChallengePart::Gold => 'g',
            ChallengePart::Silver => 's',
        };

        f.write_char(content)
    }
}

pub struct CliArgs {
    day: Option<i32>,
    part: Option<ChallengePart>,
    all: bool,
}

impl CliArgs {
    pub fn load() -> CliArgs {
        std::env::args()
            .collect::<Vec<_>>()
            .iter()
            .map(|s| s.as_str())
            .collect()
    }
}

impl<'a> FromIterator<&'a str> for CliArgs {    
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut result = CliArgs { day: None, part: None, all: false, };
        
        for arg in iter {
            if let Some(day) = arg.parse().ok() {
                result.day = Some(day);
            } else if let Some(part) = arg.parse().ok() {
                result.part = Some(part);
            } else if arg == "--all" {
                result.all = true;
            }
        }

        result
    }
}

pub struct CliExecution {
    args: CliArgs,
    executed: u32,
}

impl CliExecution {
    fn should_solve<T: PartialEq>(&self, select: impl FnOnce(&CliArgs) -> Option<T>, value: &T) -> Option<bool> {
        let from_args = select(&self.args)?;
        Some(value == &from_args)
    }

    pub fn solved(&mut self) {
        self.executed += 1;
    }
}

impl From<CliArgs> for CliExecution {
    fn from(value: CliArgs) -> Self {
        CliExecution { args: value, executed: 0, }
    }
}

impl PartialEq<i32> for CliExecution {
    fn eq(&self, other: &i32) -> bool {
        let solve_unmatched = self.args.all || self.executed == 0;
        self.should_solve(|a| a.day, other)
            .unwrap_or(solve_unmatched)        
    }
}

impl PartialEq<ChallengePart> for CliExecution {
    fn eq(&self, other: &ChallengePart) -> bool {
        self.should_solve(|a| a.part, other).unwrap_or(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve() {
        fn parse<'a>(input: impl IntoIterator<Item = &'a str>) -> CliExecution {
            let args: CliArgs = input.into_iter().collect();
            args.into()
        }

        assert_eq!(parse(["2"]) == 2, true);
        assert_eq!(parse(["2"]) == 1, false);
        assert_eq!(parse([]) == 1, true);
        assert_eq!(parse(["s"]) == ChallengePart::Silver, true);
        assert_eq!(parse(["s"]) == ChallengePart::Gold, false);
        assert_eq!(parse([]) == ChallengePart::Silver, true);

        let mut solved = parse([]);
        solved.solved();
        assert_eq!(solved == 1, false);

        let mut all_solved = parse(["--all"]);
        all_solved.solved();
        assert_eq!(all_solved == 1, true);
    }

    #[test]
    fn load_arg_all() {
        let args: CliArgs = ["--all"].into_iter().collect();
        assert_eq!(args.all, true);
    }
}
