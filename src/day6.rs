#[cfg(test)]
mod test {
    use std::fs;

    type Point = (u32, u32);

    #[test]
    fn example() {
        let points = vec![(1, 1), (1, 6), (8, 3), (3, 4), (5, 5), (8, 9)];
        run(points);
    }

    fn run(points: Vec<(u32, u32)>) {
        // find bounds, to figure out which are infinite
        let (xs, ys): (Vec<u32>, Vec<u32>) = points.iter().cloned().unzip();
        let range_x = *xs.iter().min().unwrap()..=*xs.iter().max().unwrap();
        let range_y = *ys.iter().min().unwrap()..=*ys.iter().max().unwrap();
        println!(
            "Considering {} points in the range {:?} to {:?}",
            points.len(),
            range_x,
            range_y
        );

        // figure out which are internal points
        let internal: Vec<&Point> = points
            .iter()
            .filter(|&(x, y)| {
                range_x.start() < x && x < range_x.end() && range_y.start() < y && y < range_y.end()
            })
            .collect();
        println!("Found {} internal points", internal.len());

        // compute all points within bounds for brute forcing
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
