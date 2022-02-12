use chrono::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

use crate::day::Day;
use crate::parse;

type DayMap = HashMap<NaiveDate, Day>;

#[derive(Debug, PartialEq)]
pub struct Month {
    pub month: u8,
    pub year: u16,

    pub days: DayMap,
}

impl Month {
    pub fn new(month: u8, year: u16, days_vector: Vec<Day>) -> Self {
        let mut days = DayMap::new();
        for day in days_vector {
            days.insert(day.date, day);
        }

        Self { month, year, days }
    }

    pub fn from_brf(contents: &str, year: u16, month: u8) -> Self {
        parse::parse_month(contents, year, month)
    }

    pub fn add_day(&mut self, date: &NaiveDate) {
        match self.days.get(date) {
            Some(_day) => (),
            None => {
                let day = Day::from_date(date);
                self.days.insert(day.date, day);
            }
        }
    }

    pub fn cleanup(&mut self) {
        self.days.retain(|_, d| !d.is_empty());
    }

    pub fn duration(&self) -> Duration {
        self.days.values().map(|d| d.duration()).sum::<Duration>()
    }

    pub fn max_num_blocks_in_day(&self) -> usize {
        if self.days.is_empty() {
            return 0;
        }

        self.days.values().map(|d| d.blocks.len()).max().unwrap()
    }

    pub fn title(&self) -> String {
        format!("{} {}", self.name(), self.year)
    }

    pub fn find_day_by_date(&mut self, date: &NaiveDate) -> Option<&mut Day> {
        self.days.get_mut(date)
    }

    pub fn sorted_days(&self) -> Vec<Day> {
        sorted_by_key(&self.days)
    }

    pub fn full_sorted_days(&self) -> Vec<Day> {
        let mut days = self.days.clone();

        for day in 1..=self.total_num_days() {
            let date = NaiveDate::from_ymd(self.year as i32, self.month as u32, day as u32);

            if let None = days.get(&date) {
                days.insert(date, Day::from_date(&date));
            }
        }

        sorted_by_key(&days)
    }

    fn total_num_days(&self) -> i64 {
        NaiveDate::from_ymd(
            match self.month {
                12 => self.year as i32 + 1,
                _ => self.year as i32,
            },
            match self.month {
                12 => 1,
                _ => self.month as u32 + 1,
            },
            1,
        )
        .signed_duration_since(NaiveDate::from_ymd(self.year as i32, self.month as u32, 1))
        .num_days()
    }

    fn name(&self) -> &str {
        match self.month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => panic!("invalid month number"),
        }
    }
}

fn sorted_by_key(days: &DayMap) -> Vec<Day> {
    let mut dates = days.keys().collect::<Vec<_>>();
    dates.sort();

    dates
        .into_iter()
        .map(|k| days.get(k).unwrap().clone())
        .collect()
}
