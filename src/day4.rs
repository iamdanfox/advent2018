#[cfg(test)]
mod test {
    use chrono::NaiveDateTime;
    use regex::Regex;
    use std::fs;
    use std::str::FromStr;

    #[derive(Debug, PartialEq)]
    struct LogEntry {
        datetime: NaiveDateTime,
        event: Event,
    }

    #[derive(Debug, PartialEq)]
    enum Event {
        GuardBeginsShift(u32),
        FallsAsleep,
        WakesUp
    }

    impl FromStr for Event {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
            if s == "falls asleep" {
                return Ok(Event::FallsAsleep)
            } else if s == "wakes up" {
                return Ok(Event::WakesUp)
            } else {
                lazy_static! {
                    static ref re: Regex = Regex::new(r"^Guard #(?P<id>\d+) begins shift$").unwrap();
                }
                let captures = re.captures(s).unwrap();
                let id: u32 = captures.name("id").unwrap().as_str().parse().unwrap();
                return Ok(Event::GuardBeginsShift(id));
            }
        }
    }

    impl FromStr for LogEntry {
        type Err = ();

        fn from_str(line: &str) -> Result<Self, <Self as FromStr>::Err> {
            lazy_static! {
                static ref re: Regex = Regex::new(r"^\[(?P<datetime>.+)\] (?P<event>.+)$").unwrap();
            }
            let captures = re.captures(line).unwrap();

            let datetime = captures.name("datetime").unwrap().as_str();

            Ok(LogEntry {
                datetime: NaiveDateTime::parse_from_str(datetime, "%Y-%m-%d %H:%M").unwrap(),
                event: captures.name("event").unwrap().as_str().parse()?,
            })
        }
    }

    #[test]
    fn input() {
        let x: Vec<LogEntry> = fs::read_to_string("day4.txt").unwrap().lines()
            .map(|line| LogEntry::from_str(line).unwrap())
            .collect();

        dbg!(x);
    }
}
