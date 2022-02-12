use chrono::prelude::*;

use crate::block::Block;
use crate::day::Day;
use crate::fmt;
use crate::fmt::BLOCK_FORMAT;
use crate::fmt::BLOCK_SEP;
use crate::parse;

pub type MonthYear = (u32, i32);

pub fn prev_month(my: MonthYear) -> MonthYear {
    match my {
        (1, y) => (12, y - 1),
        (m, y) => (m - 1, y),
    }
}

pub fn next_month(my: MonthYear) -> MonthYear {
    match my {
        (12, y) => (1, y + 1),
        (m, y) => (m + 1, y),
    }
}

pub fn infer_month(input: &str, my: MonthYear) -> MonthYear {
    let parts = input.split('.').collect::<Vec<_>>();
    let m = parts
        .get(0)
        .map(|m| m.parse::<u32>().unwrap())
        .expect("Could not parse month");

    if parts.len() == 1 {
        return (m, my.1);
    }

    let mut y = parts
        .get(1)
        .map(|y| y.parse::<i32>().unwrap())
        .expect("Could not parse year");

    if y < 100 {
        y += 2000;
    }

    (m, y)
}

pub fn infer_date(input: &str, date: &NaiveDate) -> NaiveDate {
    let parts = input.split('.').collect::<Vec<_>>();

    let day = parts.get(0).map(|s| s.parse::<u32>().unwrap());
    let month = parts.get(1).map(|s| s.parse::<u32>().unwrap());
    let year = parts.get(2).map(|s| s.parse::<i32>().unwrap());

    match (year, month, day) {
        (Some(y), Some(m), Some(d)) => NaiveDate::from_ymd(y, m, d),
        (None, Some(m), Some(d)) => NaiveDate::from_ymd(date.year(), m, d),
        (None, None, Some(d)) => NaiveDate::from_ymd(date.year(), date.month(), d),
        _ => todo!("Improve error handling"),
    }
}

fn normalize_half_block(s: &str) -> String {
    if s.contains(':') {
        return s.to_string();
    }

    if s == "now" {
        return Local::now().format(BLOCK_FORMAT).to_string();
    }

    match s.len() {
        5 => s.to_string(),
        4 => format!("{}:{}", &s[0..2], &s[2..4]),
        3 => format!("0{}:{}", &s[0..1], &s[1..3]),
        _ => format!("{}:00", s),
    }
}

fn normalize_block(s: &str) -> String {
    s.split(BLOCK_SEP)
        .collect::<Vec<_>>()
        .iter()
        .map(|half_block| normalize_half_block(half_block))
        .collect::<Vec<_>>()
        .join(BLOCK_SEP)
}

pub fn infer_block(block: &str, day: &Day) -> Block {
    let mut normalized_block = normalize_block(block);

    // Full block, no need to infer completion
    if normalized_block.contains(BLOCK_SEP) {
        return parse::parse_block(&day.date, normalized_block.as_str());
    }

    let ongoing_block = day.find_ongoing_block();
    normalized_block = match ongoing_block {
        Some(b) => format!(
            "{}{}{}",
            fmt::format_block_date(&b.from),
            BLOCK_SEP,
            normalized_block
        ),
        None => format!("{}{}{}", normalized_block, BLOCK_SEP, normalized_block),
    };

    parse::parse_block(&day.date, normalized_block.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_blocks() {
        assert_eq!("08:30", normalize_half_block("08:30"));
        assert_eq!("08:30", normalize_half_block("0830"));
        assert_eq!("08:30", normalize_half_block("830"));
        assert_eq!("08:00", normalize_half_block("08"));
        assert_eq!("8:00", normalize_half_block("8"));

        // We'll rely on the BRF parser to report on errors. Keeps normalization stupid.
        assert_eq!("80000", normalize_half_block("80000"));
        assert_eq!("asdfasdf:00", normalize_half_block("asdfasdf"));

        assert_eq!("12:30-18:00", normalize_block("1230-18"));
        assert_eq!("8:00-09:33", normalize_block("8-933"));
    }

    #[test]
    fn test_prev_month() {
        assert_eq!((12, 2022), prev_month((1, 2023)));
        assert_eq!((1, 2022), prev_month((2, 2022)));
    }

    #[test]
    fn test_next_month() {
        assert_eq!((1, 2024), next_month((12, 2023)));
        assert_eq!((2, 2022), next_month((1, 2022)));
    }
}
