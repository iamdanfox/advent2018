#[cfg(test)]
mod test {
    use chrono::NaiveDateTime;
    use chrono::Timelike;
    use core::fmt::Write;
    use itertools::Itertools;
    use regex::Regex;
    use std::fmt::Debug;
    use std::fmt::Error;
    use std::fmt::Formatter;
    use std::fs;
    use std::option::Option::Some;
    use std::str::FromStr;

    #[derive(Debug, Eq, PartialEq)]
    struct LogEntry {
        pub datetime: NaiveDateTime,
        pub event: Event,
    }

    #[derive(Debug, Eq, PartialEq)]
    enum Event {
        GuardBeginsShift(u32),
        FallsAsleep,
        WakesUp
    }

    struct GuardSleepReport {
        pub id: u32,
        /// minutes spent asleep during the midnight hour
        pub asleep_minutes: [bool; 60],
    }

    impl Debug for GuardSleepReport {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            f.write_char('#')?;
            self.id.fmt(f)?;
            f.write_str(": ")?;
            for &asleep in self.asleep_minutes.iter() {
                f.write_char(if asleep { '#' } else { '.' })?;
            }
            Ok(())
        }
    }

    fn guard_sleep_reports(entries: &[LogEntry]) -> Vec<GuardSleepReport> {
        let mut result = vec![];

        let mut guard_id = None;
        let mut asleep_since = None;
        let mut asleep_minutes = [false; 60];

        for entry in entries {
            let current_time = entry.datetime.time().minute() as usize;
            match entry.event {
                Event::GuardBeginsShift(id) => {
                    if let Some(prev_guard_id) = guard_id {
                        result.push(GuardSleepReport {
                            id: prev_guard_id,
                            asleep_minutes,
                        });
                        asleep_minutes = [false; 60];
                    }

                    guard_id = Some(id);
                    // TODO can a guard handover when the last is asleep?
                },
                Event::FallsAsleep => {
                    assert_eq!(asleep_since, None);
                    asleep_since = Some(current_time);
                },
                Event::WakesUp => {
                    for minute in asleep_since.unwrap()..current_time {
                        asleep_minutes[minute] = true;
                    }
                    asleep_since = None
                },
            }
        }

        result
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
        let log: Vec<LogEntry> = fs::read_to_string("day4.txt").unwrap().lines()
            .map(|line| LogEntry::from_str(line).unwrap())
            .sorted_by_key(|entry| entry.datetime)
            .collect();


        dbg!(guard_sleep_reports(&log));
    }
}
