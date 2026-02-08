mod anni;
use anni::{Anniversary, DiaryErr};
use chrono::{Local, NaiveDate};

fn today_date() -> NaiveDate {
    Local::now().date_naive()
}

fn main() {
    let s = "2023-02-12 Foo bar baz";

    match Anniversary::try_from(s) {
        Ok(anni) => {
            let today = today_date();
            if let Some(diff) = anni.anni_diff(today) {
                println!(
                    "Today ({today}): {} days till {} anni of {}",
                    diff.days_till, diff.which_anni, anni.description
                )
            };
        }
        Err(DiaryErr(info)) => println!("Error: {info} when parsing {s}"),
    }
}
