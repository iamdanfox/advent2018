#[cfg(test)]
mod test {
    use itertools::Itertools;
    use regex::Regex;
    use std::collections::BTreeMap;
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::fs;
    use std::io::Error;
    use std::str::FromStr;

    #[test]
    fn example() {
        let input = sample_input();
        assert_eq!(solve_part1(&input), "CABDFE".to_string());

        solve_part2(&input);
    }

    #[test]
    fn real_data() {
        let input = real_input();
        assert_eq!(solve_part1(&input), "BFKEGNOVATIHXYZRMCJDLSUPWQ");
    }

    fn solve_part1(dependencies: &[Dependency]) -> String {
        let ids = step_ids(&dependencies);

        // Step A (key) requires C (values) to be complete
        let mut back_edges = new_edge_map(&ids);
        for dep in dependencies {
            back_edges.entry(dep.step).or_default().push(dep.prereq);
        }

        let mut order = Vec::new();
        while !back_edges.is_empty() {
            let step = *back_edges
                .iter()
                .filter(|(_node, prereqs)| prereqs.is_empty())
                .next()
                .expect("No step found with zero prereqs")
                .0;

            order.push(step);
            back_edges.remove(&step);

            for (_, prereqs) in &mut back_edges {
                prereqs.retain(|&prereq| prereq != step);
            }
        }

        order.iter().collect()
    }

    fn solve_part2(dependencies: &[Dependency]) {
        let ids = step_ids(&dependencies);

        // Step A (key) requires C (values) to be complete
        let mut back_edges = new_edge_map(&ids);
        for dep in dependencies {
            back_edges.entry(dep.step).or_default().push(dep.prereq);
        }
        dbg!(&back_edges);

        let mut workers = WorkerPool::new(1);

        while !back_edges.is_empty() {
            while workers.available() {
                let step = *back_edges
                    .iter()
                    .filter(|(_node, prereqs)| prereqs.is_empty())
                    .next()
                    .expect("No step found with zero prereqs")
                    .0;

                let duration = step_duration(step);
                workers.begin_work(step, duration);

                // remove from the graph so nobody else tries to start this work
                back_edges.remove(&step);
            }

            dbg!(&workers);

            if let Some(completed) = workers.tick() {
                // unblock new tasks
                for (_, prereqs) in &mut back_edges {
                    prereqs.retain(|prereq| completed.contains(prereq));
                }
            }
        }
    }

    fn step_duration(step: StepId) -> Time {
//        (step as usize - 'A' as usize) + 61
        (step as usize - 'A' as usize) + 1
    }

    #[ignore]
    #[test]
    fn test_step_duration() {
        assert_eq!(step_duration('A'), 61);
        assert_eq!(step_duration('B'), 62);
        assert_eq!(step_duration('C'), 63);
        assert_eq!(step_duration('Z'), 86);
    }

    type Time = usize;

    #[derive(Debug)]
    struct WorkerPool {
        time: Time,
        available_workers: u32,
        // stores tasks and the clock time when they will be finished
        in_progress: HashMap<StepId, Time>,
    }

    impl WorkerPool {
        fn new(num_workers: u32) -> WorkerPool {
            WorkerPool {
                time: 0,
                available_workers: num_workers,
                in_progress: HashMap::new()
            }
        }

        fn available(&self) -> bool {
            self.available_workers > 0
        }

        fn begin_work(&mut self, step_id: StepId, duration: Time) {
            assert!(self.available(), "tried to begin work but no worker available");
            assert!(!self.in_progress.contains_key(&step_id), "Can't schedule work already running");
            self.available_workers -= 1;
            self.in_progress.insert(step_id, self.time + duration);
        }

        // advances the clock and and work that was completed (if any)
        fn tick(&mut self) -> Option<HashSet<StepId>> {
            self.time += 1;

            // figure out which tasks have finished
            let mut completed = HashSet::new();
            for (&step, step_finished_time) in &self.in_progress {
                if step_finished_time == &self.time {
                    completed.insert(step);
                }
            }
            if completed.is_empty() {
                return None;
            }

            for step in &completed {
                self.in_progress.remove(&step);
            }

            self.available_workers += completed.len() as u32;
            Some(completed)
        }
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
