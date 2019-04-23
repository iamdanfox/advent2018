#[cfg(test)]
mod test {
    use std::str::FromStr;
    use regex::Regex;

    type Point = (usize, usize);

    type Velocity = (usize, usize); // in units per second

    /// e.g. `position=< 9,  1> velocity=< 0,  2>`
    #[derive(Debug, PartialEq)]
    struct PointMeasurement {
        point: Point,
        velocity: Velocity,
    }

    impl FromStr for PointMeasurement {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            lazy_static! {
                static ref re: Regex = Regex::new(r"^position=< *(?P<point_x>\d+), *(?P<point_y>\d+)> velocity=< *(?P<velocity_x>\d+), *(?P<velocity_y>\d+)>$").unwrap();
            }

            let captures = re.captures(s).expect("Regex should match");

            let point_x: usize = captures.name("point_x").unwrap().as_str().parse().unwrap();
            let point_y: usize = captures.name("point_y").unwrap().as_str().parse().unwrap();
            let velocity_x: usize = captures.name("velocity_x").unwrap().as_str().parse().unwrap();
            let velocity_y: usize = captures.name("velocity_y").unwrap().as_str().parse().unwrap();

            Ok(PointMeasurement {
                point: (point_x, point_y),
                velocity: (velocity_x, velocity_y),
            })
        }
    }

    #[test]
    fn single_line() {
        let result = PointMeasurement::from_str("position=< 9,  1> velocity=< 0,  2>").unwrap();
        assert_eq!(result, PointMeasurement {
            point: (9, 1),
            velocity: (0, 2),
        })
    }
}