use chrono::prelude::*;

use crate::block::Block;
use crate::day::Day;
use crate::month::Month;

pub const DATE_FORMAT: &str = "%d.%m.%y";
pub const TOTAL_PAT: &str = "Total:";

pub fn parse_month(contents: &str, year: u16, month: u8) -> Month {
    let mut lines = contents.trim().lines();
    lines.next();

    let mut days: Vec<Day> = Vec::new();
    for line in lines {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() || trimmed_line.starts_with(TOTAL_PAT) {
            continue;
        }

        let day = Day::from_brf(trimmed_line);
        days.push(day);
    }

    Month::new(month, year, days)
}

pub fn parse_day(line: &str) -> Day {
    let mut day_iter = line.split_whitespace();
    let date = parse_date(day_iter.next().unwrap());
    let mut blocks: Vec<Block> = Vec::new();
    let mut comment = None;

    for block_str in day_iter {
        if block_str.starts_with(TOTAL_PAT) {
            comment = parse_comment(line);
            break;
        }

        blocks.push(parse_block(&date, block_str));
    }

    Day {
        date,
        blocks,
        comment,
    }
}

pub fn parse_date(date: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date, DATE_FORMAT).unwrap()
}

pub fn parse_comment(line: &str) -> Option<String> {
    let parts = line.split(TOTAL_PAT).collect::<Vec<&str>>();
    let str_after_total = parts.get(1).unwrap();
    let mut iter = str_after_total.split_whitespace();
    iter.next(); // discard total hours

    let comment_str = iter.collect::<Vec<&str>>().join(" ").trim().to_string();
    if comment_str.is_empty() {
        None
    } else {
        Some(comment_str)
    }
}

pub fn parse_hm(half_block: &str) -> (u32, u32) {
    let parts = half_block.split(':').collect::<Vec<_>>();

    (
        parts.get(0).unwrap().parse::<u32>().unwrap(),
        parts.get(1).unwrap().parse::<u32>().unwrap(),
    )
}

pub fn parse_block(date: &NaiveDate, block_str: &str) -> Block {
    let parts = block_str.split('-').collect::<Vec<_>>();
    let from_str = parts.get(0).unwrap();
    let (from_hour, from_min) = parse_hm(from_str);

    let to_str = parts.get(1).unwrap();
    let (to_hour, to_min) = parse_hm(to_str);

    let from = Local
        .ymd(date.year(), date.month(), date.day())
        .and_hms(from_hour, from_min, 0);

    let to = Local
        .ymd(date.year(), date.month(), date.day())
        .and_hms(to_hour, to_min, 0);

    Block { from, to }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_block() {
        let today = NaiveDate::from_ymd(2022, 2, 12);
        let expected = Block {
            from: Local.ymd(2022, 2, 12).and_hms(8, 15, 0),
            to: Local.ymd(2022, 2, 12).and_hms(12, 0, 0),
        };

        assert_eq!(expected, parse_block(&today, "08:15-12:00"));
        assert_eq!(expected, parse_block(&today, "8:15-12:00"));
    }

    #[test]
    fn test_parse_no_comment() {
        let line = "  28.11.14    18:00-19:00   Total:   01:00";
        assert_eq!(None, parse_day(line).comment)
    }

    #[test]
    fn test_parse_comment() {
        let line = "  28.11.14    18:00-19:00   Total:   01:00 hi There ";
        assert_eq!(Some("hi There".to_string()), parse_day(line).comment)
    }

    #[test]
    fn test_parse_empty_month() {
        let contents = r#"
            February 2022



            Total: 00:00
        "#;

        let month = parse_month(&contents, 2022, 2);
        assert_eq!(0, month.days.len());
    }
}
