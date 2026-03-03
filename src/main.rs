mod anni;
use std::io::{self, BufRead};

use anni::{Anniversary, AnnoDiff, DiaryErr};
use chrono::{Local, NaiveDate};

fn main() {
    let today = today_date();
    println!("Today is ({today}): ");
    for line in io::stdin().lock().lines() {
        println!("{}", process(line.unwrap().as_str(), &today));
    }
}

fn today_date() -> NaiveDate {
    Local::now().date_naive()
}

fn transform_to_diff(s: &str, today: &NaiveDate) -> Result<(Anniversary, AnnoDiff), DiaryErr> {
    let anni = Anniversary::try_from(s)?;
    anni.anni_diff(*today).map(|d| (anni, d))
}

fn process(s: &str, today: &NaiveDate) -> String {
    match transform_to_diff(s, today) {
        Ok((anni, diff)) => format!(
            "{} days till #{} anniversary of [{}] ({})",
            diff.days_till, diff.which_anni, anni.description, anni.date
        ),
        Err(e) => e.to_string(),
    }
}
