#[cfg(test)]
mod test {
    use regex::Regex;
    use std::fs;
    use std::num::ParseIntError;
    use std::str::FromStr;

    #[derive(Debug, PartialEq)]
    struct Claim {
        id: u32,
        offset_left: u32,
        offset_top: u32,
        width: u32,
        height: u32,
    }

    impl FromStr for Claim {
        type Err = ParseIntError;

        fn from_str(line: &str) -> Result<Self, Self::Err> {
            lazy_static! {
                static ref re: Regex = Regex::new(r"^#(?P<id>\d+) @ (?P<offset_left>\d+),(?P<offset_top>\d+): (?P<width>\d+)x(?P<height>\d+)$").unwrap();
            }
            let captures = re.captures(line).unwrap();

            Ok(Claim {
                id: captures.name("id").unwrap().as_str().parse()?,
                offset_left: captures.name("offset_left").unwrap().as_str().parse()?,
                offset_top: captures.name("offset_top").unwrap().as_str().parse()?,
                width: captures.name("width").unwrap().as_str().parse()?,
                height: captures.name("height").unwrap().as_str().parse()?,
            })
        }
    }

    #[test]
    fn parse_single_line() {
        assert_eq!(
            Claim::from_str("#123 @ 3,2: 5x4").unwrap(),
            Claim {
                id: 123,
                offset_left: 3,
                offset_top: 2,
                width: 5,
                height: 4
            }
        )
    }

    #[test]
    fn parses_entire_input() {
        assert_eq!(input().len(), 1381);
    }

    fn input() -> Vec<Claim> {
        fs::read_to_string("day3.txt")
            .unwrap()
            .lines()
            .map(|l| l.parse().unwrap())
            .collect()
    }
}
