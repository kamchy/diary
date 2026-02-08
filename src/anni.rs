use chrono::Datelike;
use chrono::Months;
use chrono::NaiveDate;
use chrono::ParseError;
use std::fmt;

/// Represents an anniversary: the date and its description
#[derive(Debug)]
pub struct Anniversary {
    pub date: NaiveDate,
    pub description: String,
}
impl std::fmt::Display for Anniversary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "Anniversary date: {} description: {}",
            self.date, self.description
        ))
    }
}

/// Represents a time difference between current moment (unknown)
/// and anniversary: it states which anniversary will be celebrated
/// and in how many days to the future.
#[derive(Debug, PartialEq)]
pub struct AnnoDiff {
    pub which_anni: u32,
    pub days_till: u32,
}

impl Anniversary {
    const FORMAT: &str = "%Y-%m-%d";

    /// Calculates which anniversary is ahead of `current` date
    /// and how many days we need to wait for it
    pub fn anni_diff(&self, current: NaiveDate) -> Option<AnnoDiff> {
        if current < self.date {
            return None;
        };

        let next_anni_date = self.date.with_year(current.year())?;
        let which_anni =
            current.years_since(self.date)? + (if next_anni_date == current { 0 } else { 1 });
        let next_anni_date = if next_anni_date < current {
            next_anni_date.checked_add_months(Months::new(12))?
        } else {
            next_anni_date
        };
        let days_till = next_anni_date.signed_duration_since(current).num_days();
        Some(AnnoDiff {
            which_anni,
            days_till: days_till as u32,
        })
    }

    /// Allows to define an anniversary with given date and description
    fn from(d: NaiveDate, s: String) -> Anniversary {
        Anniversary {
            date: d,
            description: s,
        }
    }
}

type Result<T> = std::result::Result<T, DiaryErr>;

#[derive(Debug, Clone)]
pub struct DiaryErr(pub &'static str);
impl fmt::Display for DiaryErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid anniversary definition: {}", self.0)
    }
}
impl From<ParseError> for DiaryErr {
    fn from(_: ParseError) -> Self {
        DiaryErr("parse error")
    }
}

impl TryFrom<&str> for Anniversary {
    type Error = DiaryErr;

    fn try_from(value: &str) -> Result<Anniversary> {
        let split_val = value.split_once(" ");
        match split_val {
            None => Err(DiaryErr("No space separator")),
            Some((date, desc)) if date.trim().is_empty() => Err(DiaryErr("Empty date")),
            Some((date, desc)) if desc.trim().is_empty() => Err(DiaryErr("Empty description")),
            Some((date, desc)) => NaiveDate::parse_from_str(date, Anniversary::FORMAT)
                .map(|d| Anniversary::from(d, desc.to_owned()))
                .map_err(|e| e.into()),
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{Anniversary, DiaryErr, anni::AnnoDiff};
    use chrono::NaiveDate;

    #[test]
    fn try_from_parses_string() {
        let desc: &str = "My birthday";
        let date: &str = "2024-02-03";
        let expected = NaiveDate::from_ymd_opt(2024, 2, 3);
        if let Ok(actual) = Anniversary::try_from(format!("{} {}", date, desc).as_ref()) {
            assert_eq!(actual.date, expected.unwrap());
            assert_eq!(actual.description, desc);
        } else {
            panic!("Annibersary::tryFrom does not work!")
        }
    }

    #[test]
    fn anniversary_delta_should_give_info_when_before() {
        let value = "2023-03-07 Something";
        let curr_date = NaiveDate::from_ymd_opt(2025, 3, 2).unwrap();
        let expected_diff = AnnoDiff {
            which_anni: 2,
            days_till: 5,
        };
        verify_anni_diff(value, curr_date, expected_diff);
    }

    #[test]
    fn anniversary_delta_should_give_info_when_after() {
        let value = "2023-03-07 Something";
        let curr_date = NaiveDate::from_ymd_opt(2025, 3, 8).unwrap();
        let expected_diff = AnnoDiff {
            which_anni: 3,
            days_till: 364,
        };
        verify_anni_diff(value, curr_date, expected_diff);
    }

    #[test]
    fn anniversary_delta_should_give_info_when_same_date() {
        let value = "2023-03-07 Something";
        let curr_date = NaiveDate::from_ymd_opt(2025, 3, 7).unwrap();
        let expected_diff = AnnoDiff {
            which_anni: 2,
            days_till: 0,
        };
        verify_anni_diff(value, curr_date, expected_diff);
    }

    fn verify_anni_diff(value: &str, now_date: NaiveDate, epxpected_diff: AnnoDiff) {
        match Anniversary::try_from(value) {
            Ok(anni) => match anni.anni_diff(now_date) {
                Some(diff) => {
                    assert_eq!(diff, epxpected_diff);
                }
                None => panic!("Cannot cretae anni_diff"),
            },
            Err(e) => panic!("Cannot read anni: {}", e),
        }
    }

    #[test]
    fn check_no_sep() {
        match Anniversary::try_from("") {
            Ok(_) => panic!("Should not parse invalid date"),
            Err(DiaryErr(info)) => assert_eq!(info, "No space separator"),
        }
    }

    #[test]
    fn check_no_date() {
        match Anniversary::try_from(" Anniversary") {
            Ok(_) => panic!("Should not parse invalid date"),
            Err(DiaryErr(info)) => assert_eq!(info, "Empty date"),
        }
    }

    #[test]
    fn check_empty_desc_error() {
        match Anniversary::try_from("2024-03-23 ") {
            Ok(_) => panic!("Should not parse invalid date"),
            Err(DiaryErr(info)) => assert_eq!(info, "Empty description"),
        }
    }

    fn test_anno(s: &str, now: &NaiveDate, exp_anni: u32, exp_days: u32) {
        if let Ok(anni) = Anniversary::try_from(s) {
            if let Some(diff) = anni.anni_diff(*now) {
                assert_eq!(diff.which_anni, exp_anni);
                assert_eq!(diff.days_till, exp_days);
            } else {
                panic!("could not create a diff for {anni}")
            }
        } else {
            panic!("Invalid anniversatry input: {s}]");
        }
    }

    #[test]
    fn check_toms_birthday() {
        let curr_day = NaiveDate::from_ymd_opt(2026, 2, 8).unwrap();
        test_anno(
            "1989-02-09 Tom's birthday! Remember to buy him an apple.",
            &curr_day,
            37,
            1,
        );
    }

    #[test]
    fn check_cargo() {
        let curr_day = NaiveDate::from_ymd_opt(2026, 2, 8).unwrap();
        test_anno(
            "2025-12-31 Anniversary of shipping my first cargo crate!",
            &curr_day,
            1,
            326,
        );
    }

    #[test]
    fn check_diet() {
        let curr_day = NaiveDate::from_ymd_opt(2026, 2, 8).unwrap();
        test_anno("2026-02-07 Diet anniversary", &curr_day, 1, 364);
    }
}
