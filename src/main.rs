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

        lines
            .map(|line| line.parse().expect(&format!("trying to parse '{}'", line)))
            .collect()
    }

    fn accumulate_frequencies(input: Vec<i32>) -> Vec<i32> {
        input
            .iter()
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

#[cfg(test)]
mod day2 {
    use std::fs;

    #[derive(Debug, PartialEq)]
    struct Checksum(u64);

    fn checksum(entries: &[ChecksumEntry]) -> Checksum {
        let mut twos: u64 = 0;
        let mut threes: u64 = 0;
        for &entry in entries {
            twos += if entry.exactly_2_of_any { 1 } else { 0 };
            threes += if entry.exactly_3_of_any { 1 } else { 0 };
        }
        Checksum(twos * threes)
    }

    #[derive(Debug, PartialEq, Copy, Clone)]
    struct ChecksumEntry {
        pub exactly_2_of_any: bool,
        pub exactly_3_of_any: bool,
    }

    impl From<&str> for ChecksumEntry {
        fn from(string: &str) -> Self {
            let mut char_counts: [u8; 256] = [0; 256];

            for &byte in string.as_bytes() {
                let index: usize = byte as usize;
                char_counts[index] += 1
            }

            let mut two = false;
            let mut three = false;

            for &count in char_counts.iter() {
                match count {
                    2 => two = true,
                    3 => three = true,
                    _ => {}
                }
            }
            ChecksumEntry {
                exactly_2_of_any: two,
                exactly_3_of_any: three,
            }
        }
    }

    #[test]
    fn sample_testcases() {
        assert_eq!(
            ChecksumEntry::from("abcdef"),
            ChecksumEntry {
                exactly_2_of_any: false,
                exactly_3_of_any: false
            }
        );
        assert_eq!(
            ChecksumEntry::from("bababc"),
            ChecksumEntry {
                exactly_2_of_any: true,
                exactly_3_of_any: true
            }
        );
        assert_eq!(
            ChecksumEntry::from("abbcde"),
            ChecksumEntry {
                exactly_2_of_any: true,
                exactly_3_of_any: false
            }
        );
    }

    #[test]
    fn sample_testcases_2() {
        let input = [
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ];
        let foo: Vec<ChecksumEntry> = input
            .iter()
            .map(|&line| ChecksumEntry::from(line))
            .collect();
        assert_eq!(checksum(&foo), Checksum(12));
    }

    #[test]
    fn run_input() {
        let entries: Vec<ChecksumEntry> = fs::read_to_string("day2.txt")
            .unwrap()
            .lines()
            .map(ChecksumEntry::from)
            .collect();
        assert_eq!(checksum(&entries), Checksum(5928));
    }
}
