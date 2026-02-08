mod anni;
use std::num::Saturating;

use anni::{Anniversary, DiaryErr};
use chrono::{Local, NaiveDate};

fn today_date() -> NaiveDate {
    Local::now().date_naive()
}

fn main() {
    let today = today_date();
    println!("{}", transform("2023-02-12 Foo bar baz", &today));
}

fn transform(s: &str, today: &NaiveDate) -> String {
    match Anniversary::try_from(s) {
        Ok(anni) => match anni.anni_diff(*today) {
            Some(diff) => format!(
                "Today ({today}): {} days till {} anni of [{}]",
                diff.days_till, diff.which_anni, s
            ),
            None => format!("Could not find time delta for {s}"),
        },
        Err(DiaryErr(info)) => format!("Error: {info} when parsing {s}"),
    }
}
