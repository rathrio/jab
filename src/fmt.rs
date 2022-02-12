use crate::parse::DATE_FORMAT;
use chrono::prelude::*;

use colored::*;
use std::time::Duration;

use crate::block::Block;
use crate::clock::PunchCard;
use crate::day::Day;
use crate::month::Month;

pub const BLOCK_FORMAT: &str = "%H:%M";
pub const BLOCK_SEP: &str = "-";
pub const EMPTY_BLOCK: &str = "           ";
pub const EMPTY_HALF_BLOCK: &str = "     ";
pub const SPACER: &str = "   ";
pub const NEWLINE: &str = "\n";
pub const TERM_DATE_FORMAT: &str = "%a   %d.%m.%y";

#[derive(Copy, Clone)]
pub enum OutputMode {
    Term,
    File,
}

pub fn format_month(month: &Month, card: &PunchCard, mode: OutputMode) -> String {
    let pad_blocks = month.max_num_blocks_in_day();
    let days = match mode {
        OutputMode::Term => month.full_sorted_days(),
        OutputMode::File => month.sorted_days(),
    }
    .iter()
    .enumerate()
    .map(|(index, d)| format_day(d, index, pad_blocks, card, mode))
    .collect::<Vec<_>>()
    .join(NEWLINE);

    let total = format!("Total: {}", format_duration(&month.duration()));
    format!(
        "{}{}{}{}{}",
        month.title(),
        NEWLINE.repeat(2),
        days,
        NEWLINE.repeat(2),
        total
    )
}

fn format_date(date: &NaiveDate, mode: OutputMode, index: usize) -> String {
    let monday_is_first_day_in_month = index == 0;
    match (mode, date.weekday(), monday_is_first_day_in_month) {
        (OutputMode::Term, Weekday::Mon, false) => {
            format!("{}{}", NEWLINE, date.format(TERM_DATE_FORMAT))
        }
        (OutputMode::Term, _, _) => date.format(TERM_DATE_FORMAT).to_string(),
        _ => date.format(DATE_FORMAT).to_string(),
    }
}

fn format_day(
    day: &Day,
    index: usize,
    pad_blocks: usize,
    card: &PunchCard,
    mode: OutputMode,
) -> String {
    let date = format_date(&day.date, mode, index);

    let blocks = if day.blocks.is_empty() {
        "".to_string()
    } else {
        format!(
            "{}{}",
            SPACER,
            day.blocks
                .iter()
                .map(|b| format_block(b, mode))
                .collect::<Vec<_>>()
                .join(SPACER)
        )
    };

    let padding = if day.blocks.len() < pad_blocks {
        format!("{}{}", SPACER, EMPTY_BLOCK).repeat(pad_blocks - day.blocks.len())
    } else {
        "".to_string()
    };

    let total = format!("Total: {}", format_duration(&day.duration()));

    let comment = match &day.comment {
        Some(c) => format!("{}{}", SPACER, c),
        None => "".to_string(),
    };

    let output = format!(
        "{}{}{}{}{}{}",
        date, blocks, padding, SPACER, total, comment
    );

    match mode {
        OutputMode::File => output,
        OutputMode::Term => {
            if card.was_modified(&day.date) {
                output.truecolor(255, 146, 209).to_string()
            } else if card.was_selected(&day.date) {
                output.truecolor(201, 169, 250).to_string()
            } else {
                output
            }
        }
    }
}

pub fn format_block_date(dt: &DateTime<Local>) -> String {
    dt.format(BLOCK_FORMAT).to_string()
}

fn format_block(block: &Block, mode: OutputMode) -> String {
    let from_str = format_block_date(&block.from);
    let to_str = match (block.is_ongoing(), mode) {
        (true, OutputMode::Term) => EMPTY_HALF_BLOCK.to_string(),
        _ => format_block_date(&block.to),
    };

    format!("{}{}{}", from_str, BLOCK_SEP, to_str)
}

pub fn format_duration(duration: &Duration) -> String {
    let minutes = duration.as_secs() / 60;
    let hours = minutes / 60;
    let remaining_minutes = minutes - (hours * 60);
    format!("{:02}:{:02}", hours, remaining_minutes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_block() {
        let from = Local.ymd(2022, 01, 01).and_hms(8, 15, 0);
        let to = Local.ymd(2022, 01, 01).and_hms(14, 45, 0);
        let block = Block { from, to };
        assert_eq!("08:15-14:45", format_block(&block, OutputMode::File));
    }

    #[test]
    fn test_format_ongoing_block() {
        let from = Local.ymd(2022, 01, 01).and_hms(8, 15, 0);
        let to = Local.ymd(2022, 01, 01).and_hms(8, 15, 0);
        let block = Block { from, to };
        assert_eq!("08:15-08:15", format_block(&block, OutputMode::File));
        assert_eq!("08:15-     ", format_block(&block, OutputMode::Term));
    }

    #[test]
    fn test_format_day() {
        let card = PunchCard::new();
        let day =
            Day::from_brf("  04.05.20    08:30-12:00    12:30-17:30             Total: 08:30");

        assert_eq!(
            "04.05.20   08:30-12:00   12:30-17:30   Total: 08:30",
            format_day(&day, 0, 0, &card, OutputMode::File)
        );
    }

    #[test]
    fn test_format_day_with_padding() {
        let card = PunchCard::new();
        let day =
            Day::from_brf("  04.05.20    08:30-12:00    12:30-17:30             Total: 08:30");

        assert_eq!(
            "04.05.20   08:30-12:00   12:30-17:30                                             Total: 08:30",
            format_day(&day, 0, 5, &card, OutputMode::File)
        );
    }

    #[test]
    fn test_format_duration() {
        let duration = Duration::new(30600, 0);
        let actual = format_duration(&duration);
        assert_eq!("08:30", actual);
    }
}
