use chrono::prelude::*;
use std::cmp::Ord;
use std::time::Duration;

use crate::block::Block;
use crate::parse;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Day {
    pub date: NaiveDate,
    pub blocks: Vec<Block>,
    pub comment: Option<String>,
}

impl Day {
    pub fn from_date(date: &NaiveDate) -> Self {
        Self {
            date: *date,
            blocks: vec![],
            comment: None,
        }
    }

    pub fn duration(&self) -> Duration {
        self.blocks.iter().map(|b| b.duration()).sum::<Duration>()
    }

    pub fn from_brf(line: &str) -> Self {
        parse::parse_day(line)
    }

    pub fn add_comment(&mut self, comment: &str) {
        self.comment = Some(comment.to_string());
    }

    pub fn clear_comment(&mut self) {
        self.comment = None;
    }

    pub fn add_block(&mut self, to_add: &Block) {
        let mut block = Block {
            from: to_add.from,
            to: to_add.to,
        };

        // Ignore when there are existing blocks containing the new block
        if self.blocks.iter().any(|b| b.contains(&block)) {
            return;
        }

        // Handle overlaps
        if let Some(b) = self.blocks.iter().find(|b| block.contains_dt(b.to)) {
            if b.from <= block.from {
                block.from = b.from;
            }
        };
        self.blocks.retain(|b| !block.contains(b));

        if let Some(b) = self.blocks.iter().find(|b| block.contains_dt(b.from)) {
            if b.to >= block.to {
                block.to = b.to;
            }
        };
        self.blocks.retain(|b| !block.contains(b));

        self.blocks.push(block);
        self.blocks.sort();
    }

    pub fn remove_block(&mut self, to_remove: &Block) {
        self.blocks.retain(|b| !to_remove.contains(b));

        let to_split = self
            .blocks
            .iter_mut()
            .find(|b| b.strictly_contains(to_remove));

        if let Some(b) = to_split {
            let new_block = Block {
                from: to_remove.to,
                to: b.to,
            };
            b.to = to_remove.from;
            self.add_block(&new_block);
            return;
        }

        // Remove at end
        if let Some(b) = self.find_block_containing_dt(to_remove.from) {
            b.to = to_remove.from;
        };

        // Remove at start
        if let Some(b) = self.find_block_containing_dt(to_remove.to) {
            b.from = to_remove.to;
        };
    }

    pub fn find_ongoing_block(&self) -> Option<&Block> {
        self.blocks.iter().find(|b| b.is_ongoing())
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty() && self.comment.is_none()
    }

    fn find_block_containing_dt(&mut self, dt: DateTime<Local>) -> Option<&mut Block> {
        self.blocks.iter_mut().find(|b| b.contains_dt(dt))
    }
}

impl Ord for Day {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.date.cmp(&other.date)
    }
}

impl PartialOrd for Day {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.date.cmp(&other.date))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fmt;
    use crate::parse;

    #[test]
    fn test_add_block() {
        let today = NaiveDate::from_ymd(2022, 2, 12);
        let block1 = parse::parse_block(&today, "08:00-10:00");
        let block2 = parse::parse_block(&today, "11:00-14:00");

        let mut day = Day::from_date(&today);
        day.add_block(&block1);
        day.add_block(&block2);

        assert_eq!(day.blocks.len(), 2);
        assert_eq!(fmt::format_duration(&day.duration()), "05:00");
    }

    #[test]
    fn test_add_blocks_with_overlaps() {
        let today = NaiveDate::from_ymd(2022, 2, 12);
        let block1 = parse::parse_block(&today, "08:00-10:00");
        let block2 = parse::parse_block(&today, "09:00-12:00");

        let mut day = Day::from_date(&today);
        day.add_block(&block1);
        day.add_block(&block2);

        assert_eq!(day.blocks.len(), 1);
        assert_eq!(fmt::format_duration(&day.duration()), "04:00");
    }

    #[test]
    fn test_add_contained_blocks() {
        let today = NaiveDate::from_ymd(2022, 2, 12);
        let block1 = parse::parse_block(&today, "08:00-10:00");
        let block2 = parse::parse_block(&today, "09:00-10:00");

        let mut day = Day::from_date(&today);
        day.add_block(&block1);
        day.add_block(&block2);

        assert_eq!(day.blocks.len(), 1);
        assert_eq!(fmt::format_duration(&day.duration()), "02:00");
    }

    #[test]
    fn test_add_blocks_containing_existing_blocks() {
        let today = NaiveDate::from_ymd(2022, 2, 12);
        let block1 = parse::parse_block(&today, "09:00-10:00");
        let block2 = parse::parse_block(&today, "08:00-10:00");

        let mut day = Day::from_date(&today);
        day.add_block(&block1);
        day.add_block(&block2);

        assert_eq!(day.blocks.len(), 1);
        assert_eq!(fmt::format_duration(&day.duration()), "02:00");
    }

    #[test]
    fn test_merging_blocks() {
        let today = NaiveDate::from_ymd(2022, 2, 12);
        let block1 = parse::parse_block(&today, "09:00-10:00");
        let block2 = parse::parse_block(&today, "11:00-14:00");
        let block3 = parse::parse_block(&today, "09:30-11:30");

        let mut day = Day::from_date(&today);
        day.add_block(&block1);
        day.add_block(&block2);
        day.add_block(&block3);

        assert_eq!(day.blocks.len(), 1);
        assert_eq!(fmt::format_duration(&day.duration()), "05:00");
    }

    #[test]
    fn test_removing_block_by_shadowing() {
        let mut day = parse::parse_day("12.02.20  12:00-14:00  Total: 02:00");
        let block = parse::parse_block(&day.date, "12:00-14:00");
        day.remove_block(&block);
        assert_eq!(true, day.is_empty());
    }

    #[test]
    fn test_removing_block_by_splitting() {
        let mut day = parse::parse_day("12.02.20  08:00-17:00  Total: 09:00");
        let block = parse::parse_block(&day.date, "12:00-13:00");
        day.remove_block(&block);

        assert_eq!(2, day.blocks.len());
        assert_eq!("08:00", fmt::format_duration(&day.duration()));
    }

    #[test]
    fn test_removing_block_at_end() {
        let mut day = parse::parse_day("12.02.20  08:00-17:00  Total: 09:00");
        let block = parse::parse_block(&day.date, "15:00-17:00");
        day.remove_block(&block);

        assert_eq!(1, day.blocks.len());
        assert_eq!("07:00", fmt::format_duration(&day.duration()));
    }

    #[test]
    fn test_removing_block_at_start() {
        let mut day = parse::parse_day("12.02.20  08:00-17:00  Total: 09:00");
        let block = parse::parse_block(&day.date, "07:00-09:00");
        day.remove_block(&block);

        assert_eq!(1, day.blocks.len());
        assert_eq!("08:00", fmt::format_duration(&day.duration()));
    }
}
