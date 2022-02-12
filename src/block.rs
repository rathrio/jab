use chrono::prelude::*;
use std::time::Duration;

use crate::day::Day;
use crate::infer::infer_block;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Block {
    pub from: DateTime<Local>,
    pub to: DateTime<Local>,
}

impl Block {
    pub fn parse(block: &str, day: &Day) -> Self {
        infer_block(block, day)
    }

    pub fn duration(&self) -> Duration {
        (self.to - self.from).to_std().unwrap()
    }

    pub fn is_ongoing(&self) -> bool {
        self.from == self.to
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    pub fn strictly_contains(&self, other: &Self) -> bool {
        self.from < other.from && self.to > other.to
    }

    pub fn contains_dt(&self, dt: DateTime<Local>) -> bool {
        self.from <= dt && self.to >= dt
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.from.cmp(&other.from)
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.from.cmp(&other.from))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_ongoing() {
        let from = Local.ymd(2022, 2, 12).and_hms(8, 0, 0);
        let to = Local.ymd(2022, 2, 12).and_hms(9, 0, 0);
        let block = Block { from, to };

        assert_eq!(block.is_ongoing(), false);
    }

    #[test]
    fn test_ongoing() {
        let date = Local.ymd(2022, 2, 12).and_hms(8, 0, 0);
        let block = Block {
            from: date,
            to: date,
        };

        assert_eq!(block.is_ongoing(), true);
    }

    #[test]
    fn test_contains() {
        let inner_from = Local.ymd(2022, 2, 12).and_hms(8, 0, 0);
        let inner_to = Local.ymd(2022, 2, 12).and_hms(9, 0, 0);
        let inner_block = Block {
            from: inner_from,
            to: inner_to,
        };

        let outer_from = Local.ymd(2022, 2, 12).and_hms(8, 0, 0);
        let outer_to = Local.ymd(2022, 2, 12).and_hms(10, 0, 0);
        let outer_block = Block {
            from: outer_from,
            to: outer_to,
        };

        assert_eq!(true, outer_block.contains(&inner_block));
        assert_eq!(false, inner_block.contains(&outer_block));
    }

    #[test]
    fn test_block_inferrence() {
        let today = NaiveDate::from_ymd(2022, 2, 12);
        let day = Day::from_date(&today);
        let block = Block::parse("08:00", &day);
        assert_eq!(true, block.is_ongoing());
    }
}
