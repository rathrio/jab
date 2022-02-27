use chrono::prelude::*;
use std::collections::HashSet;
use std::env::var;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use std::process::ExitStatus;

use crate::block::Block;
use crate::cli::Args;
use crate::fmt;
use crate::fmt::OutputMode;
use crate::infer;
use crate::month::Month;

const DEFAULT_EDITOR: &str = "vim";
const DEFAULT_HOURS_DIR: &str = "./hours";

pub struct PunchCard {
    hours_dir_path: String,
    selected_dates: HashSet<NaiveDate>,
    modified_dates: HashSet<NaiveDate>,
}

impl PunchCard {
    pub fn new() -> PunchCard {
        let hours_dir_path = match var("PUNCH_HOURS_DIR") {
            Ok(dir) => dir,
            Err(_) => DEFAULT_HOURS_DIR.to_string(),
        };

        PunchCard {
            hours_dir_path,
            selected_dates: HashSet::new(),
            modified_dates: HashSet::new(),
        }
    }

    pub fn brf_file_path(&self, year: i32, month: u32) -> String {
        format!("{}/{}-{}.txt", self.hours_dir_path, year, month)
    }

    pub fn select_date(&mut self, date: &NaiveDate) {
        self.selected_dates.insert(*date);
    }

    pub fn modify_date(&mut self, date: &NaiveDate) {
        self.modified_dates.insert(*date);
    }

    pub fn was_selected(&self, date: &NaiveDate) -> bool {
        self.selected_dates.contains(date)
    }

    pub fn was_modified(&self, date: &NaiveDate) -> bool {
        self.modified_dates.contains(date)
    }

    pub fn has_modifications(&self) -> bool {
        !self.modified_dates.is_empty()
    }
}

fn open_dir(path: &str) -> Result<ExitStatus, std::io::Error> {
    Command::new("open").arg(path).status()
}

fn edit_brf(path: &Path) -> Result<ExitStatus, std::io::Error> {
    let editor = match var("EDITOR") {
        Ok(e) => e,
        Err(_) => DEFAULT_EDITOR.to_string(),
    };

    Command::new(editor).arg(path).status()
}

fn write_brf(month: &Month, card: &PunchCard, path: &Path, dry_run: bool) {
    if dry_run {
        return;
    }

    fs::write(path, fmt::format_month(month, card, OutputMode::File))
        .expect("Could not write file");
}

pub fn punch(args: &Args) {
    let mut date = Local::now().naive_local().date();
    let mut year = date.year();
    let mut month_number = date.month();

    let mut card = PunchCard::new();

    if args.brf {
        open_dir(&card.hours_dir_path).expect("Could not open hours directory");
        return;
    }

    if args.yesterday {
        date = date.pred();
    }

    if let Some(d) = &args.day {
        date = infer::infer_date(d, &date);
    }

    if args.previous {
        let my = infer::prev_month((month_number, year));
        month_number = my.0;
        year = my.1;
    } else if args.next {
        let my = infer::next_month((month_number, year));
        month_number = my.0;
        year = my.1;
    }

    if let Some(m) = &args.month {
        let my = infer::infer_month(m, (month_number, year));
        month_number = my.0;
        year = my.1;
    }

    card.select_date(&date);

    let path_str = card.brf_file_path(year, month_number);
    let file_path = Path::new(&path_str);

    if args.edit {
        edit_brf(file_path).expect("Could not edit file");
        return;
    }

    let mut file = OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open(file_path)
        .unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read current BRF file");

    let mut month = Month::from_brf(&contents, year as u16, month_number as u8);
    month.add_day(&date);
    let day = month.find_day_by_date(&date).unwrap();

    if args.clear_comment {
        day.clear_comment();
        card.modify_date(&date);
    }

    match &args.comment {
        Some(comment) => {
            day.add_comment(comment);
            card.modify_date(&date);
        }
        None => (),
    };

    if !args.blocks.is_empty() {
        args.blocks.iter().for_each(|block_str| {
            let block = Block::parse(block_str, day);
            if args.remove {
                day.remove_block(&block);
            } else {
                day.add_block(&block);
            }
        });

        card.modify_date(&date);
    }

    println!("{}", fmt::format_month(&month, &card, OutputMode::Term));
    if card.has_modifications() && !args.dry_run {
        month.cleanup();
        write_brf(&month, &card, file_path, args.dry_run);
    }
}
