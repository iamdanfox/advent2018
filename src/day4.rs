#[cfg(test)]
mod test {
    use chrono::NaiveDateTime;
    use chrono::Timelike;
    use core::fmt::Write;
    use itertools::Itertools;
    use regex::Regex;
    use std::collections::HashMap;
    use std::fmt::Debug;
    use std::fmt::Error;
    use std::fmt::Formatter;
    use std::fs;
    use std::option::Option::Some;
    use std::slice::Iter;
    use std::str::FromStr;

    type GuardId = u32;

    #[derive(Debug, Eq, PartialEq)]
    struct LogEntry {
        pub datetime: NaiveDateTime,
        pub event: Event,
    }

    #[derive(Debug, Eq, PartialEq)]
    enum Event {
        GuardBeginsShift(GuardId),
        FallsAsleep,
        WakesUp,
    }

    #[derive(Copy, Clone)]
    pub struct SleepReport([bool; 60]);

    impl SleepReport {
        // TODO(dfox): try IntoIter instead?
        pub fn iter(&self) -> Iter<bool> {
            self.0.iter()
        }

        pub fn count(&self) -> usize {
            self.0.iter().filter(|&&b| b).count()
        }
    }

    impl Debug for SleepReport {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            for &asleep in self.0.iter() {
                f.write_char(if asleep { '#' } else { '.' })?;
            }
            Ok(())
        }
    }

    #[derive(Copy, Clone)]
    struct GuardShiftReport {
        pub guard_id: GuardId,
        /// minutes spent asleep during the midnight hour
        pub asleep_minutes: SleepReport,
    }

    impl Debug for GuardShiftReport {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            f.write_char('#')?;
            self.guard_id.fmt(f)?;
            f.write_str(": ")?;
            self.asleep_minutes.fmt(f)?;
            self.asleep_minutes.count().fmt(f)?;
            Ok(())
        }
    }

    struct GuardReport {
        pub guard_id: GuardId,
        pub asleep_minutes: Vec<SleepReport>,
    }

    fn guard_sleep_reports(entries: &[LogEntry]) -> Vec<GuardShiftReport> {
        let mut result = vec![];

        let mut guard_id = None;
        let mut asleep_since = None;
        let mut asleep_minutes = [false; 60];

        for entry in entries {
            let current_time = entry.datetime.time().minute() as usize;
            match entry.event {
                Event::GuardBeginsShift(id) => {
                    assert_eq!(asleep_since, None);
                    if let Some(prev_guard_id) = guard_id {
                        result.push(GuardShiftReport {
                            guard_id: prev_guard_id,
                            asleep_minutes: SleepReport(asleep_minutes),
                        });
                        asleep_minutes = [false; 60];
                    }

                    guard_id = Some(id);
                }
                Event::FallsAsleep => {
                    assert_eq!(asleep_since, None);
                    asleep_since = Some(current_time);
                }
                Event::WakesUp => {
                    for minute in asleep_since.unwrap()..current_time {
                        asleep_minutes[minute] = true;
                    }
                    asleep_since = None
                }
            }
        }

        if let Some(prev_guard_id) = guard_id {
            result.push(GuardShiftReport {
                guard_id: prev_guard_id,
                asleep_minutes: SleepReport(asleep_minutes),
            });
        }

        result
    }

    impl FromStr for Event {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
            if s == "falls asleep" {
                return Ok(Event::FallsAsleep);
            } else if s == "wakes up" {
                return Ok(Event::WakesUp);
            } else {
                lazy_static! {
                    static ref re: Regex =
                        Regex::new(r"^Guard #(?P<id>\d+) begins shift$").unwrap();
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
        let log: Vec<LogEntry> = fs::read_to_string("day4.txt")
            .unwrap()
            .lines()
            .map(|line| LogEntry::from_str(line).unwrap())
            .sorted_by_key(|entry| entry.datetime)
            .collect();
        let reports = guard_sleep_reports(&log);

        let reports_by_guard_id: HashMap<GuardId, Vec<&GuardShiftReport>> = reports
            .iter()
            .map(|report| (report.guard_id, report))
            .into_group_map();

        let worst_guard = &reports_by_guard_id
            .iter()
            .map(|(&guard, vec)| -> (GuardId, usize) {
//                GuardReport {
//                    guard_id,
//                    asleep_minutes: vec.iter().map(|report| report.asleep_minutes).collect()
//                }
                (
                    guard,
                    vec.iter().map(|report| report.asleep_minutes.count()).sum(),
                )
            })
            .collect_vec();

        let sleepiest_guard_id: GuardId = worst_guard
            .iter()
            .max_by_key(|(_guard, mins_asleep)| mins_asleep)
            .unwrap()
            .0;

        assert_eq!(sleepiest_guard_id, 761);

        let mut histogram: [u32; 60] = [0; 60];

        for report in reports
            .iter()
            .filter(|report| report.guard_id == sleepiest_guard_id)
        {
            for (min, &asleep) in report.asleep_minutes.iter().enumerate() {
                if asleep {
                    histogram[min] += 1
                }
            }
        }

        let sleepiest_minute = histogram
            .iter()
            .enumerate()
            .max_by_key(|(_minute, &asleep_count)| asleep_count)
            .unwrap()
            .0;
        assert_eq!(sleepiest_minute, 25);

        let final_result = sleepiest_minute * sleepiest_guard_id as usize;
        assert_eq!(final_result, 19025);
    }

    #[test]
    fn example_input() {
        let sample = r#"
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #88 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
"#;
        let log: Vec<LogEntry> = sample
            .trim()
            .lines()
            .map(|line| LogEntry::from_str(line).unwrap())
            .sorted_by_key(|entry| entry.datetime)
            .collect();

        let string = guard_sleep_reports(&log)
            .iter()
            .map(|report| format!("{:?}", report))
            .join("\n");
        let expected = r#"
#10: .....####################.....#########################.....45
#99: ........................................##########..........10
#10: ........................#####...............................5
#99: ....................................##########..............10
#88: .............................................##########.....10"#;
        assert_eq!(string, expected.trim());
    }
}
