#[cfg(test)]
mod test {
    use itertools::Itertools;
    use std::collections::HashMap;
    use std::fs;

    type Point = (u32, u32);

    #[test]
    fn example() {
        let points = vec![(1, 1), (1, 6), (8, 3), (3, 4), (5, 5), (8, 9)];
        assert_eq!(run(&points), 17);
    }

    #[test]
    fn part1() {
        let points = input();
        assert_eq!(run(&points), 3620);
    }

    fn run(points: &[Point]) -> usize {
        // find bounds, to allow us to brute force
        let (xs, ys): (Vec<u32>, Vec<u32>) = points.iter().cloned().unzip();
        let range_x = *xs.iter().min().unwrap()..=*xs.iter().max().unwrap();
        let range_y = *ys.iter().min().unwrap()..=*ys.iter().max().unwrap();
        println!(
            "Total points: {}, in the range {:?} to {:?}",
            points.len(),
            range_x,
            range_y
        );

        // compute grid to exactly cover our input points
        let grid: Vec<Point> = range_x.cartesian_product(range_y).collect_vec();

        // figure out who owns each grid point
        let mut owned_points: HashMap<Point, Vec<Point>> = HashMap::new();
        for grid_point in grid {
            let owner = find_nearest(&grid_point, &points);
            owned_points.entry(owner).or_default().push(grid_point);
        }

        let (_point, owned) = owned_points
            .iter()
            .max_by_key(|(_, vec)| vec.len())
            .unwrap();
        owned.len()
    }

    fn find_nearest(home: &Point, others: &[Point]) -> Point {
        *others
            .iter()
            .min_by_key(|&other| manhattan(home, other))
            .unwrap()
    }

    fn manhattan((x1, y1): &Point, (x2, y2): &Point) -> u32 {
        let dist_x = if x1 > x2 { x1 - x2 } else { x2 - x1 };
        let dist_y = if y1 > y2 { y1 - y2 } else { y2 - y1 };
        dist_x + dist_y
    }

    fn input() -> Vec<Point> {
        fs::read_to_string("day6.txt")
            .unwrap()
            .lines()
            .map(|l| -> Point {
                let parts: Vec<&str> = l.split(", ").collect();
                (parts[0].parse().unwrap(), parts[1].parse().unwrap())
            })
            .collect()
    }
}
