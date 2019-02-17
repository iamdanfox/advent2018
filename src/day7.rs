#[cfg(test)]
mod test {
    use itertools::Itertools;
    use regex::Regex;
    use std::collections::BTreeMap;
    use std::fs;
    use std::io::Error;
    use std::str::FromStr;

    #[test]
    fn example() {
        let dependencies = sample_input();
        let ids = step_ids(&dependencies);

        // Step A (key) requires C (values) to be complete
        let mut back_edges = new_edge_map(&ids);
        for dep in &dependencies {
            back_edges.entry(dep.step).or_default().push(dep.prereq);
        }
        dbg!(back_edges);

        // Step A (key) is prereq of B, D (values)
        let mut forward_edges = new_edge_map(&ids);
        for dep in &dependencies {
            forward_edges.entry(dep.prereq).or_default().push(dep.step);
        }
        dbg!(&forward_edges);

        let starting_node = *forward_edges
            .iter()
            .filter(|(_node, prereqs)| prereqs.is_empty())
            .next()
            .expect("No step found with zero prereqs")
            .0;

        assert_eq!(starting_node, 'E');
    }

    // ensures all node ids appear
    fn new_edge_map(ids: &[StepId]) -> BTreeMap<StepId, Vec<StepId>> {
        let mut map: BTreeMap<StepId, Vec<StepId>> = BTreeMap::new();
        for &id in ids {
            map.insert(id, Vec::new());
        }
        map
    }

    type StepId = char;

    #[derive(Debug, PartialEq)]
    struct Dependency {
        pub step: StepId,
        pub prereq: StepId,
    }

    impl FromStr for Dependency {
        type Err = Box<Error>;
        fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
            lazy_static! {
                static ref RE: Regex =
                    Regex::new(r"^Step (.) must be finished before step (.) can begin.$").unwrap();
            }

            let captures = RE
                .captures(s)
                .expect(&format!("Regex didn't match '{}'", &s));
            Ok(Dependency {
                prereq: captures[1].chars().next().expect("Chars for prereq"),
                step: captures[2].chars().next().expect("Chars for step"),
            })
        }
    }

    fn step_ids(input: &[Dependency]) -> Vec<StepId> {
        let iter1 = input.iter().map(|d| d.step);
        let iter2 = input.iter().map(|d| d.prereq);
        iter1.chain(iter2).sorted().unique().collect()
    }

    #[test]
    fn dependency_from_str() {
        assert_eq!(
            Dependency::from_str("Step C must be finished before step A can begin.").unwrap(),
            Dependency {
                step: 'A',
                prereq: 'C'
            }
        );

        assert_eq!(real_input().len(), 101);
    }

    fn real_input() -> Vec<Dependency> {
        fs::read_to_string("day7.txt")
            .unwrap()
            .lines()
            .map(|s| s.parse().unwrap())
            .collect()
    }

    fn sample_input() -> Vec<Dependency> {
        let sample = r#"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
"#;
        sample
            .lines()
            .map(|l| Dependency::from_str(l).unwrap())
            .collect_vec()
    }
}
