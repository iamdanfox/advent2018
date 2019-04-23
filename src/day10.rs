#[cfg(test)]
mod test {
    use core::fmt::Write;
    use std::collections::HashSet;
    use std::fmt::{Debug, Error, Formatter};
    use std::fs;
    use std::iter::Skip;
    use std::ops::Range;
    use std::str::FromStr;

    use itertools::Itertools;
    use regex::Regex;

    type Point = (isize, isize);

    type Velocity = (isize, isize); // in units per second

    /// e.g. `position=< 9,  1> velocity=< 0,  2>`
    #[derive(Debug, PartialEq, Clone)]
    struct PointMeasurement {
        point: Point,
        velocity: Velocity,
    }

    impl PointMeasurement {
        fn step(&self, time_units: usize) -> PointMeasurement {
            let mut cloned = self.clone();
            cloned.step_mut(time_units);
            cloned
        }

        fn step_mut(&mut self, time_units: usize) {
            let (x, y) = self.point;
            self.point.0 = x + self.velocity.0 * time_units as isize;
            self.point.1 = y + self.velocity.1 * time_units as isize;
        }
    }

    fn iter_velocities(measurements: &Vec<PointMeasurement>) -> impl Iterator<Item=Grid> {
        struct GridIterator {
            points: Vec<PointMeasurement>,
            time: usize,
        }

        impl Iterator for GridIterator {
            type Item = Grid;

            fn next(&mut self) -> Option<Self::Item> {
                let grid = self.nth(self.time)?;
                self.time += 1;
                Some(grid)
            }

            fn nth(&mut self, mut n: usize) -> Option<Self::Item> {
                // TODO(dfox): should this actually mutate the iterator's time field?
                Some(Grid {
                    points: self.points.iter()
                        .map(|p| p.step(n).point)
                        .collect(),
                })
            }
        }

        GridIterator {
            points: (*measurements).clone(),
            time: 0,
        }
    }

    impl FromStr for PointMeasurement {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^position=< *(?P<point_x>-?\d+), *(?P<point_y>-?\d+)> velocity=< *(?P<velocity_x>-?\d+), *(?P<velocity_y>-?\d+)>$").unwrap();
            }

            let captures = RE
                .captures(s)
                .expect(&("Regex should match: ".to_string() + s));

            let point_x: isize = captures.name("point_x").unwrap().as_str().parse().unwrap();
            let point_y: isize = captures.name("point_y").unwrap().as_str().parse().unwrap();
            let velocity_x: isize = captures
                .name("velocity_x")
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            let velocity_y: isize = captures
                .name("velocity_y")
                .unwrap()
                .as_str()
                .parse()
                .unwrap();

            Ok(PointMeasurement {
                point: (point_x, point_y),
                velocity: (velocity_x, velocity_y),
            })
        }
    }

    struct Grid {
        points: Vec<Point>,
    }

    impl Grid {
        fn bounds(&self) -> (Range<isize>, Range<isize>) {
            let (min_x, max_x) = self
                .points
                .iter()
                .map(|&point| point.0)
                .minmax()
                .into_option()
                .expect("Vec must not be empty");
            let (min_y, max_y) = self
                .points
                .iter()
                .map(|&point| point.1)
                .minmax()
                .into_option()
                .expect("Vec must not be empty");

            return (min_x..max_x, min_y..max_y);
        }

        fn size(&self) -> (usize, usize) {
            let (xs, ys) = self.bounds();
            ((xs.end - xs.start) as usize, (ys.end - ys.start) as usize)
        }

        fn area(&self) -> usize {
            let (width, height) = self.size();
            width * height
        }
    }

    impl Debug for Grid {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            let (xs, ys) = self.bounds();

            for y in ys {
                for x in xs.clone() {
                    let current: Point = (x, y);
                    if self.points.contains(&current) {
                        f.write_char('#')?
                    } else {
                        f.write_char('.')?
                    }
                }

                f.write_char('\n')?
            }

            Ok(())
        }
    }

    #[test]
    fn single_line() {
        let result = PointMeasurement::from_str("position=< 9,  1> velocity=< 0,  2>").unwrap();
        assert_eq!(
            result,
            PointMeasurement {
                point: (9, 1),
                velocity: (0, 2),
            }
        )
    }

    #[test]
    fn single_line_with_negatives() {
        let result =
            PointMeasurement::from_str("position=<-20620, -41485> velocity=< 2,  4>").unwrap();
        assert_eq!(
            result,
            PointMeasurement {
                point: (-20620, -41485),
                velocity: (2, 4),
            }
        )
    }

    #[test]
    fn render_sample_data() {
        iter_velocities(&sample_data()).take(4).for_each(|g| {
            dbg!(g);
        });
    }

    fn sample_data() -> Vec<PointMeasurement> {
        let string = r#"position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>"#;
        string
            .lines()
            .map(|s| PointMeasurement::from_str(s).expect("line parses"))
            .collect()
    }

    fn real_data() -> Vec<PointMeasurement> {
        fs::read_to_string("day10.txt")
            .unwrap()
            .lines()
            .map(|s| s.parse().unwrap())
            .collect()
    }

    #[test]
    fn render_real_data() {
        let max_iterations = 20_000;
        let mut last_area = usize::max_value();
        let mut smallest_grid = None;
        let real_data = real_data();

        // non-linear upward search to find the rough order of magnitute of time we care about
        let mut iter = iter_velocities(&real_data);
        let mut samples = Vec::new();
        for i in 1..20 {
            let nth = 2 << i;
            let area = iter.nth(nth as usize).unwrap().area();
            samples.push((nth, area));
        }
        samples.sort_by_key(|&(_, area)| area);
        dbg!(&samples);
        samples.truncate(2);
        samples.sort_by_key(|&(time, _)| time);
        let start_time = samples[0].0;

        // TODO(dfox): use this start_time to avoid granular searches early on in time

        for (i, g) in iter_velocities(&real_data).enumerate() {
            if i == max_iterations {
                smallest_grid = None;
                break;
            }

            if g.area() > last_area {
                break;
            }

            last_area = g.area();
            smallest_grid = Some(g)
        }

        dbg!(smallest_grid); //  "PHIGRNFK"
    }
}
