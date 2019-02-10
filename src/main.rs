fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod day1 {
    use std::collections::HashSet;
    use std::iter;

    #[test]
    fn part1() {
        let freq = accumulate_frequencies(input());
        assert_eq!(*freq.last().unwrap(), 508);
    }

    #[test]
    fn part2() {
        // check our methodology works on the provided cases
        assert_eq!(find_first_repeat(vec![1, -1]), Some(0));
        assert_eq!(find_first_repeat(vec![3, 3, 4, -2, -4]), Some(10));
        assert_eq!(find_first_repeat(vec![-6, 3, 8, 5, -6]), Some(5));
        assert_eq!(find_first_repeat(vec![7, 7, -2, -7, -4]), Some(14));


        assert_eq!(find_first_repeat(input()), Some(549));
    }

    fn input() -> Vec<i32> {
        let result = std::fs::read_to_string("day1.txt").unwrap();
        let lines = result.lines();

        lines.map(|line| line.parse().expect(&format!("trying to parse '{}'", line))).collect()
    }

    fn accumulate_frequencies(input: Vec<i32>) -> Vec<i32> {
        input.iter()
            .scan(0, |freq, change| {
                *freq = *freq + change;
                return Some(*freq);
            })
            .collect()
    }

    fn find_first_repeat(raw_input: Vec<i32>) -> Option<i32> {
        let repeated_input = iter::repeat(raw_input).take(200).flatten().collect();

        let mut seen_already = HashSet::new();
        seen_already.insert(0);
        for freq in accumulate_frequencies(repeated_input) {
            if !seen_already.insert(freq) {
                return Some(freq);
            }
        }
        None
    }
}
