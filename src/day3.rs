#[cfg(test)]
mod test {
    use itertools::Itertools;
    use multiset::HashMultiSet;
    use regex::Regex;
    use std::fs;
    use std::num::ParseIntError;
    use std::str::FromStr;

    type Square = (usize, usize);

    #[test]
    fn part1() {
        assert_eq!(Claim::count_overlapping_squares(input()), 117505);
    }

    #[derive(Debug, PartialEq)]
    struct Claim {
        id: usize,
        offset_left: usize,
        offset_top: usize,
        width: usize,
        height: usize,
    }

    impl Claim {
        fn squares(&self) -> impl Iterator<Item = Square> {
            let xs = self.offset_left..(self.offset_left + self.width);
            let ys = self.offset_top..(self.offset_top + self.height);
            xs.cartesian_product(ys)
        }

        fn count_overlapping_squares(claims: Vec<Claim>) -> usize {
            let squares: HashMultiSet<Square> = claims.iter().flat_map(Claim::squares).collect();

            let mut overlaps = 0;
            for square in squares.distinct_elements() {
                if squares.count_of(square) > 1 {
                    overlaps += 1;
                }
            }

            overlaps
        }
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
                height: 4,
            }
        )
    }

    #[test]
    fn parses_entire_input() {
        assert_eq!(input().len(), 1381);
    }

    #[test]
    fn to_square_produces_one_tuple_for_every_square_in_claim() {
        let squares: Vec<Square> = Claim::from_str("#123 @ 3,2: 5x4")
            .unwrap()
            .squares()
            .collect();
        assert_eq!(squares.len(), 5 * 4);
    }

    #[test]
    fn find_overlap_of_three_claims() {
        let c1 = Claim::from_str("#123 @ 0,0: 1x1").unwrap();
        let c2 = Claim::from_str("#124 @ 0,0: 1x1").unwrap();
        let c3 = Claim::from_str("#125 @ 0,0: 2x2").unwrap();
        assert_eq!(Claim::count_overlapping_squares(vec![c1, c2, c3]), 1);
    }

    fn input() -> Vec<Claim> {
        fs::read_to_string("day3.txt")
            .unwrap()
            .lines()
            .map(|l| l.parse().unwrap())
            .collect()
    }
}
