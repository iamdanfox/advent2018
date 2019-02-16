#[cfg(test)]
mod test {
    use itertools::Itertools;
    use multiset::HashMultiSet;
    use regex::Regex;
    use std::collections::HashSet;
    use std::fs;
    use std::num::ParseIntError;
    use std::str::FromStr;

    type Square = (usize, usize);

    #[test]
    fn part1() {
        assert_eq!(Claim::contended_squares(&input()).len(), 117505);
    }

    #[test]
    fn part2() {
        let uncontended: Vec<Claim> = Claim::find_uncontended_claims(&input());
        assert_eq!(uncontended.len(), 1);
        assert_eq!(uncontended[0].id, 1254);
    }

    #[derive(Debug, PartialEq, Copy, Clone)]
    struct Claim {
        pub id: usize,
        offset_left: usize,
        offset_top: usize,
        width: usize,
        height: usize,
    }

    impl Claim {
        pub fn squares(&self) -> impl Iterator<Item = Square> {
            let xs = self.offset_left..(self.offset_left + self.width);
            let ys = self.offset_top..(self.offset_top + self.height);
            xs.cartesian_product(ys)
        }

        pub fn contains(&self, (x, y): Square) -> bool {
            let hit_x = self.offset_left <= x && x < self.offset_left + self.width;
            let hit_y = self.offset_top <= y && y < self.offset_top + self.height;
            hit_x && hit_y
        }

        pub fn contended_squares(claims: &[Claim]) -> HashSet<Square> {
            let squares: HashMultiSet<Square> = claims.iter().flat_map(Claim::squares).collect();

            let mut contended = HashSet::new();

            for square in squares.distinct_elements() {
                if squares.count_of(square) > 1 {
                    contended.insert(*square);
                }
            }

            contended
        }

        pub fn find_uncontended_claims(claims: &[Claim]) -> Vec<Claim> {
            let contended = Claim::contended_squares(claims);

            let is_contended =
                |claim: &Claim| -> bool { contended.iter().any(|&square| claim.contains(square)) };

            let result: Vec<Claim> = claims
                .iter()
                .filter(|claim| !is_contended(claim))
                .map(|&claim| claim)
                .collect();

            result
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
        assert_eq!(Claim::contended_squares(&[c1, c2, c3]).len(), 1);
    }

    #[test]
    fn contains_works() {
        assert!(Claim::from_str("#123 @ 0,0: 1x1").unwrap().contains((0, 0)));
        assert!(!Claim::from_str("#123 @ 0,0: 1x1").unwrap().contains((1, 1)));

        assert!(!Claim::from_str("#123 @ 0,0: 2x2").unwrap().contains((2, 5)));
        assert!(!Claim::from_str("#123 @ 0,0: 2x2").unwrap().contains((3, 3)));
    }

    fn input() -> Vec<Claim> {
        fs::read_to_string("day3.txt")
            .unwrap()
            .lines()
            .map(|l| l.parse().unwrap())
            .collect()
    }
}
