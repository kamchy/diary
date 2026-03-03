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

fn anno_diff(anno: &NaiveDate, now: &NaiveDate) -> Option<AnnoDiff> {
    if let Some(anno_date) = anno.with_year(now.year()).map(|a| {
        if now.signed_duration_since(a).num_days() > 0 {
            a.checked_add_months(Months::new(12))
        } else {
            Some(a)
        }
    }) {
        if let Some(ad) = anno_date {
            let which_anni = (ad.year() - anno.year()) as u32;
            let days_till = ad.signed_duration_since(*now).num_days() as u32;
            Some(AnnoDiff {
                which_anni,
                days_till,
            })
        } else {
            None
        }
    } else {
        None
    }
}
impl Anniversary {
    const FORMAT: &str = "%Y-%m-%d";

    /// Calculates which anniversary is ahead of `current` date
    /// and how many days we need to wait for it
    pub fn anni_diff(&self, current: NaiveDate) -> Result<AnnoDiff, DiaryErr> {
        anno_diff(&self.date, &current).ok_or_else(|| DiaryErr::AnniversaryCalculationImpossible)
    }

    /// Allows to define an anniversary with given date and description
    fn from(d: NaiveDate, s: String) -> Anniversary {
        Anniversary {
            date: d,
            description: s,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
/// DiaryErr represents an error in diary line string.
pub enum DiaryErr {
    InvalidEntryFormat,
    EmptyDate,
    EmptyDesc,
    NoSpaceSep,
    AnniversaryCalculationImpossible,
}

impl fmt::Display for DiaryErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiaryErr::InvalidEntryFormat => {
                write!(f, "Invalid diary entry definition: ")
            }
            DiaryErr::AnniversaryCalculationImpossible => {
                write!(f, "Cannot calculate anniversary:")
            }
            DiaryErr::EmptyDate => write!(f, "Date is empty"),
            DiaryErr::EmptyDesc => write!(f, "Description is empty"),
            DiaryErr::NoSpaceSep => write!(f, "No space separator"),
        }
    }
}
impl From<ParseError> for DiaryErr {
    fn from(_: ParseError) -> Self {
        DiaryErr::InvalidEntryFormat
    }
}

impl TryFrom<&str> for Anniversary {
    type Error = DiaryErr;

    fn try_from(value: &str) -> Result<Anniversary, DiaryErr> {
        let split_val = value.split_once(" ");
        match split_val {
            None => Err(DiaryErr::NoSpaceSep),
            Some((date, desc)) if date.trim().is_empty() => Err(DiaryErr::EmptyDate),
            Some((date, desc)) if desc.trim().is_empty() => Err(DiaryErr::EmptyDesc),
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
            panic!("Anniversary::tryFrom does not work!")
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

    fn verify_anni_diff(value: &str, now_date: NaiveDate, expected_diff: AnnoDiff) {
        Anniversary::try_from(value)
            .map(|a| a.anni_diff(now_date))
            .map_or_else(
                |_| panic!("error found"),
                |diff| assert_eq!(diff, Ok(expected_diff)),
            )
    }

    #[test]
    fn check_no_sep() {
        match Anniversary::try_from("") {
            Ok(_) => panic!("Should not parse invalid date"),
            Err(e) => assert_eq!(e, DiaryErr::NoSpaceSep),
        }
    }

    #[test]
    fn check_no_date() {
        match Anniversary::try_from(" Anniversary") {
            Ok(_) => panic!("Should not parse invalid date"),
            Err(e) => assert_eq!(e, DiaryErr::EmptyDate),
        }
    }

    #[test]
    fn check_empty_desc_error() {
        match Anniversary::try_from("2024-03-23 ") {
            Ok(_) => panic!("Should not parse invalid date"),
            Err(err) => assert_eq!(err, DiaryErr::EmptyDesc),
        }
    }

    #[test]
    fn check_toms_birthday() {
        let curr_day = NaiveDate::from_ymd_opt(2026, 2, 8).unwrap();
        verify_anni_diff(
            "1989-02-09 Tom's birthday! Remember to buy him an apple.",
            curr_day,
            AnnoDiff {
                which_anni: 37,
                days_till: 1,
            },
        );
    }

    #[test]
    fn check_cargo() {
        let curr_day = NaiveDate::from_ymd_opt(2026, 2, 8).unwrap();
        verify_anni_diff(
            "2025-12-31 Anniversary of shipping my first cargo crate!",
            curr_day,
            AnnoDiff {
                which_anni: 1,
                days_till: 326,
            },
        )
    }

    #[test]
    fn check_diet() {
        let curr_day = NaiveDate::from_ymd_opt(2026, 2, 8).unwrap();
        verify_anni_diff(
            "2026-02-07 Diet anniversary",
            curr_day,
            AnnoDiff {
                which_anni: 1,
                days_till: 364,
            },
        );
    }
}
