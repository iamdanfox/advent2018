#[cfg(test)]
mod test {
    use itertools::Itertools;
    use std::collections::HashSet;
    use std::fs;
    use std::iter::FromIterator;
    use std::string::String;

    #[test]
    fn part1() {
        let checksum: Checksum = fs::read_to_string("day2.txt")
            .unwrap()
            .lines()
            .map(ChecksumEntry::from)
            .collect();
        assert_eq!(checksum, Checksum(5928));
    }

    fn find_strings_differing_by_1(input: Vec<&str>) -> HashSet<String> {
        let mut results = HashSet::new();

        for (left, right) in input.iter().cartesian_product(input.iter()) {
            let (num, matches) = diff_strings(left, right);
            if num == 1 {
                results.insert(matches);
            }
        }

        results
    }

    #[test]
    fn part2() {
        assert_eq!(
            find_strings_differing_by_1(vec!(
                "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz"
            )),
            HashSet::from_iter(vec!("fgij".to_owned()))
        );

        assert_eq!(
            find_strings_differing_by_1(fs::read_to_string("day2.txt").unwrap().lines().collect()),
            HashSet::from_iter(vec!("bqlporuexkwzyabnmgjqctvfs".to_owned()))
        )
    }

    fn diff_strings(left: &str, right: &str) -> (usize, String) {
        assert_eq!(left.len(), right.len());

        let matching: Vec<u8> = left
            .as_bytes()
            .iter()
            .zip(right.as_bytes())
            .filter(|(&left_byte, &right_byte)| left_byte == right_byte)
            .map(|(&left_byte, _)| left_byte)
            .collect();

        let differing_chars = left.len() - matching.len();
        let matching_str: &str = std::str::from_utf8(&matching).unwrap();

        (differing_chars, matching_str.to_owned())
    }

    #[test]
    fn count_differing_characters_samples() {
        assert_eq!(diff_strings("fghij", "fguij"), (1, "fgij".to_owned()));
        assert_eq!(diff_strings("abcde", "axcye"), (2, "ace".to_owned()));
    }

    #[derive(Debug, PartialEq)]
    struct Checksum(u64);

    impl FromIterator<ChecksumEntry> for Checksum {
        fn from_iter<I>(iter: I) -> Self
        where
            I: IntoIterator<Item = ChecksumEntry>,
        {
            let mut twos: u64 = 0;
            let mut threes: u64 = 0;
            for entry in iter {
                twos += if entry.exactly_2_of_any { 1 } else { 0 };
                threes += if entry.exactly_3_of_any { 1 } else { 0 };
            }
            Checksum(twos * threes)
        }
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
        let foo: Checksum = input
            .iter()
            .map(|&line| ChecksumEntry::from(line))
            .collect();
        assert_eq!(foo, Checksum(12));
    }
}
