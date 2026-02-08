mod anni;
use std::io::{self, BufRead};

use anni::{Anniversary, DiaryErr};
use chrono::{Local, NaiveDate};

fn today_date() -> NaiveDate {
    Local::now().date_naive()
}

fn main() {
    let today = today_date();
    println!("Today ({today}): ");
    for line in io::stdin().lock().lines() {
        println!("{}", transform(line.unwrap().as_str(), &today));
    }
}

fn transform(s: &str, today: &NaiveDate) -> String {
    match Anniversary::try_from(s) {
        Ok(anni) => match anni.anni_diff(*today) {
            Some(diff) => format!(
                "{} days till #{} anniversary of [{}] ({})",
                diff.days_till, diff.which_anni, anni.description, anni.date
            ),
            None => format!("Could not find time delta for {s}"),
        },
        Err(DiaryErr(info)) => format!("Error: {info} when parsing {s}"),
    }
}
