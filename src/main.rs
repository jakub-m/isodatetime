use std::io::{self, BufRead};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime, SecondsFormat, TimeZone, Utc};
use lazy_static::lazy_static;
use regex::{Captures, Regex};

lazy_static! {
    static ref re_datetime: Regex = Regex::new(r"datetime\.datetime\((?P<year>\d+), (?P<month>\d+), (?P<day>\d+), (?P<hour>\d+), (?P<minute>\d+)(?:, (?P<second>\d+))?(?:, (?P<microsecond>\d+))?, tzinfo=datetime\.timezone\.utc\)").unwrap();
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    for line in handle.lines().map(|line| line.unwrap()) {
        let line = replace_datetime_in_line(line.as_str());
        println!("{}", line)
    }
}

fn replace_datetime_in_line(haystack: &str) -> String {
    let replacement = |captures: &Captures| -> String {
        format_datetime_utc(captures).unwrap_or(captures.get(0).unwrap().as_str().to_owned())
    };
    re_datetime.replace_all(haystack, replacement).to_string()
}

fn format_datetime_utc(captures: &Captures) -> Option<String> {
    let zero = "0";
    let year: i32 = captures.name("year").unwrap().as_str().parse().ok()?;
    let month: u32 = captures.name("month").unwrap().as_str().parse().ok()?;
    let day: u32 = captures.name("day").unwrap().as_str().parse().ok()?;
    let hour: u32 = captures.name("hour").unwrap().as_str().parse().ok()?;
    let minute: u32 = captures.name("minute").unwrap().as_str().parse().ok()?;
    let second: u32 = captures
        .name("second")
        .map_or(zero.to_owned(), |m| m.as_str().to_owned())
        .parse()
        .unwrap();
    let microsecond: u32 = captures
        .name("microsecond")
        .map_or(zero.to_owned(), |m| m.as_str().to_owned())
        .parse()
        .unwrap();

    let date = NaiveDate::from_ymd_opt(year, month, day)?;
    let time = NaiveTime::from_hms_milli_opt(hour, minute, second, microsecond / 1000)?;
    let naive_datetime = NaiveDateTime::new(date, time);
    let datetime = Utc::from_utc_datetime(&Utc, &naive_datetime);

    Some(datetime.to_rfc3339_opts(SecondsFormat::Millis, true))
}
