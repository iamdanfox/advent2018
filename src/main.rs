fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod day1 {
    use std::collections::HashSet;

    fn input() -> Vec<i32> {
        let result = std::fs::read_to_string("day1.txt").unwrap();
        let lines = result.lines();

        lines.map(|line| line.parse().expect(&format!("trying to parse '{}'", line))).collect()
    }

    #[test]
    fn part1() {
        let lines = input();

        let freq = lines.iter().fold(0, |freq, change| {
            freq + change
        });

        assert_eq!(freq, 508);
    }

    #[test]
    fn part2() {
        let mut frequencies = HashSet::new();
        frequencies.insert(0);
    }
}
