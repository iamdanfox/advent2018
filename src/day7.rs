#[cfg(test)]
mod test {
    use regex::Regex;
    use std::fs;
    use std::str::FromStr;
    use std::io::Error;
    use itertools::Itertools;

    #[test]
    fn example() {
        let sample = r#"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
"#;

        let input = sample.lines().map(|l| Dependency::from_str(l).unwrap()).collect_vec();
        dbg!(input);
    }

    type StepId = char;

    #[derive(Debug, PartialEq)]
    struct Dependency {
        step: StepId,
        depends_on: StepId,
    }

    impl FromStr for Dependency {
        type Err = Box<Error>;
        fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^Step (.) must be finished before step (.) can begin.$").unwrap();
            }

            let captures = RE.captures(s).expect(&format!("Regex didn't match '{}'", &s));
            Ok(Dependency {
                depends_on: captures[1].chars().next().expect("Chars for prereq"),
                step: captures[2].chars().next().expect("Chars for step"),
            })
        }
    }

    #[test]
    fn dependency_from_str() {
        assert_eq!(
            Dependency::from_str("Step C must be finished before step A can begin.").unwrap(),
            Dependency {
                step: 'A',
                depends_on: 'C'
            });

        assert_eq!(parse_input().len(), 101);
    }

    fn parse_input() -> Vec<Dependency> {
        fs::read_to_string("day7.txt")
            .unwrap()
            .lines()
            .map(|s| s.parse().unwrap())
            .collect()
    }
}
