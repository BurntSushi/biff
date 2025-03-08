use {
    anyhow::Context,
    jiff::{
        ToSpan,
        civil::{Date, Weekday},
    },
};

#[derive(Clone, Copy, Debug)]
pub struct WeekDate {
    // The weekday on which this week date calendar starts weeks. This is the
    // key difference from `jiff::civil::ISOWeekDate`, which always starts on
    // Monday.
    start: Weekday,
    year: i16,
    week: i8,
    weekday: Weekday,
}

impl WeekDate {
    /// Create a new week date.
    ///
    /// `year` must be in the range `-9999.=9999`. `week` must be in the range
    /// `1..=53`, although `53` is only valid for "long" years.
    ///
    /// `start` corresponds to how the week numbering scheme determines the
    /// start of a week.
    pub fn new(
        start: Weekday,
        year: i16,
        week: i8,
        weekday: Weekday,
    ) -> anyhow::Result<WeekDate> {
        if week == 53 && !is_long_year(start, year) {
            anyhow::bail!(
                "week number `{week}` (for weeks starting on {start:?}) \
                 is invalid for year `{year}`"
            );
        }
        if year == -9999 || year == 9999 {
            let start_of_year =
                week_start_of_year(start, year).with_context(|| {
                    format!(
                        "week date `{year:04}-W{week:02}-{weekday:?}` \
                         (for weeks starting on {start:?}) \
                         is invalid",
                    )
                })?;
            let mut days = i32::from(week - 1) * 7;
            days += i32::from(weekday.since(start));
            if start_of_year.checked_add(days.days()).is_err() {
                anyhow::bail!(
                    "week date `{year:04}-W{week:02}-{weekday:?}` \
                     (for weeks starting on {start:?}) \
                     is invalid",
                )
            }
        }
        Ok(WeekDate { start, year, week, weekday })
    }

    /// Returns a new week date for the given Gregorian date.
    ///
    /// The week date uses a week numbering scheme where the given weekday
    /// is the first day in the week. That is, the first week date of a year
    /// starts on the given weekday and is the first week whose majority of
    /// days (>= 4) falls in the same Gregorian year.
    pub fn from_date(start: Weekday, date: Date) -> anyhow::Result<WeekDate> {
        let mut start_of_year = week_start_of_year(start, date.year())?;
        if date < start_of_year {
            start_of_year = week_start_of_year(start, date.year() - 1)?;
        } else {
            let next = date.year() + 1;
            // This fails when `date` is in year 9999, which is Jiff's maximum.
            // But by the same token, in that case, `date` will never be
            // greater than or equal to the start of that year. So we can just
            // ignore it.
            if let Ok(next_start_of_year) = week_start_of_year(start, next) {
                if date >= next_start_of_year {
                    start_of_year = next_start_of_year;
                }
            }
        }

        assert!(date >= start_of_year);
        // OK according to Jiff's docs.
        let diff_days = start_of_year.until(date).unwrap().get_days();
        // +1 because weeks are one-indexed
        // Also, unwrap is okay because `date` cannot be more than 53 weeks
        // after `start_of_year`.
        let week = i8::try_from(diff_days / 7).unwrap() + 1;
        // The week date year is guaranteed to match the Gregorian year 4
        // days after the start of the first day of the week date year.
        let year = (start_of_year + 4.days()).year();
        let weekday = date.weekday();
        Ok(WeekDate { start, year, week, weekday })
    }

    /// Converts this week date to its corresponding Gregorian date.
    pub fn date(self) -> Date {
        // OK because otherwise it would be impossible to construct this
        // `WeekDate`.
        let start_of_year = week_start_of_year(self.start, self.year).unwrap();
        let mut days = i32::from(self.week - 1) * 7;
        days += i32::from(self.weekday.since(self.start));
        // OK, again, because otherwise it would be impossible to construct
        // this `WeekDate`.
        start_of_year.checked_add(days.days()).unwrap()
    }

    /// Returns the number of weeks in the year containing this week date.
    pub fn weeks_in_year(self) -> i8 {
        if is_long_year(self.start, self.year) { 53 } else { 52 }
    }
}

/// Returns the start of the week that the given date resides in.
///
/// The starting point of the week is determined by `start`.
pub fn first_of_week(start: Weekday, date: Date) -> anyhow::Result<Date> {
    let wd = date.weekday();
    if start == wd {
        Ok(date)
    } else {
        date.nth_weekday(-1, start).with_context(|| {
            format!(
                "failed to find first day of week containing \
                 {date}, for weeks starting on {start:?}",
            )
        })
    }
}

/// Returns the end of the week that the given date resides in.
///
/// The starting point of the week is determined by `start`.
pub fn last_of_week(start: Weekday, date: Date) -> anyhow::Result<Date> {
    let last = start.wrapping_sub(1);
    let wd = date.weekday();
    if last == wd {
        Ok(date)
    } else {
        date.nth_weekday(1, last).with_context(|| {
            format!(
                "failed to find last day of week containing \
                 {date}, for weeks starting on {start:?}",
            )
        })
    }
}

/// Returns true if the given week year (with weeks starting on `start`) is a
/// "long" year or not.
///
/// A "long" year is a year with 53 weeks. Otherwise, it's a "short" year with
/// 52 weeks.
fn is_long_year(start: Weekday, year: i16) -> bool {
    // Inspired by: https://en.wikipedia.org/wiki/ISO_week_date#Weeks_per_year
    let last = jiff::civil::date(year, 12, 31);
    let weekday = last.weekday();
    weekday == start.wrapping_add(3)
        || (last.in_leap_year() && weekday == start.wrapping_add(4))
}

/// Returns the first date in the first week of the given year.
///
/// The date returned is guaranteed to have a weekday equivalent to `start`.
fn week_start_of_year(start: Weekday, year: i16) -> anyhow::Result<Date> {
    // RFC 5545 says:
    //
    // > A week is defined as a seven day period, starting on the day of the
    // > week defined to be the week start (see WKST). Week number one of the
    // > calendar year is the first week that contains at least four (4) days
    // > in that calendar year.
    //
    // Which means that Jan 4 *must* be in the first week of the year.
    let date_in_first_week = Date::new(year, 1, 4).with_context(|| {
        format!(
            "failed to find first week date of year `{year}` for \
             weeks starting with {start:?}",
        )
    })?;
    // Now find the number of days since the start of the week from a date that
    // we know is in the first week of `year`.
    let diff_from_start = date_in_first_week.weekday().since(start);
    let span = diff_from_start.days();
    date_in_first_week.checked_sub(span).with_context(|| {
        format!(
            "first date of `{year}` for weeks starting with \
             {start:?} is out of Biff's supported range",
        )
    })
}

#[cfg(test)]
mod tests {
    use jiff::civil::{ISOWeekDate, Weekday::*};

    use super::*;

    /// Just some sanity tests around the boundaries of a year for a weekday
    /// that isn't Sunday/Monday.
    #[test]
    fn week_date_start_of_year() {
        let date = jiff::civil::date(2025, 1, 4);
        let wd = WeekDate::from_date(Saturday, date).unwrap();
        assert_eq!((wd.year, wd.week, wd.weekday), (2025, 1, Saturday));
        assert_eq!(date, wd.date());

        let date = jiff::civil::date(2025, 1, 3);
        let wd = WeekDate::from_date(Saturday, date).unwrap();
        assert_eq!((wd.year, wd.week, wd.weekday), (2024, 53, Friday));
        assert_eq!(date, wd.date());

        let date = jiff::civil::date(2025, 1, 5);
        let wd = WeekDate::from_date(Saturday, date).unwrap();
        assert_eq!((wd.year, wd.week, wd.weekday), (2025, 1, Sunday));
        assert_eq!(date, wd.date());
    }

    /// Tests that for the case of ISO weeks (weeks starting on Monday), the
    /// `WeekDate` gets the same result as Jiff's `ISOWeekDate`.
    #[test]
    fn week_date_start_of_year_consistent_jiff() {
        let years = &[-9999..=-9900, -100..=100, 1800..=2300, 9900..=9999];
        let month_days: &[(i8, i8)] = &[
            (1, 1),
            (1, 2),
            (1, 3),
            (1, 4),
            (1, 5),
            (1, 6),
            (1, 7),
            (1, 8),
            (1, 9),
            (1, 10),
            (7, 1),
            (12, 22),
            (12, 23),
            (12, 24),
            (12, 25),
            (12, 26),
            (12, 27),
            (12, 28),
            (12, 29),
            (12, 30),
            (12, 31),
        ];
        let mkiso = |wd: WeekDate| {
            // This conversion only applies to week dates with
            // Monday as the start of the week.
            assert_eq!(wd.start, Monday);
            ISOWeekDate::new(wd.year, wd.week, wd.weekday).unwrap()
        };
        for range in years.iter().cloned() {
            for year in range {
                for &(month, day) in month_days {
                    let date = jiff::civil::date(year, month, day);
                    let expected = date.iso_week_date();
                    let wd = WeekDate::from_date(Monday, date).unwrap();
                    let got = mkiso(wd);
                    assert_eq!(
                        expected, got,
                        "given {year:04}-{month:02}-{day:02}, expected ISO \
                         week year to be {expected:?}, but got {got:?}",
                    );

                    // While we're here, test that going back to Gregorian
                    // works.
                    assert_eq!(date, wd.date());
                }
            }
        }
    }

    #[test]
    fn boundaries_different_week_starts() {
        assert!(WeekDate::new(Monday, 9999, 52, Friday).is_ok());
        assert!(WeekDate::new(Monday, 9999, 52, Saturday).is_err());
        assert!(WeekDate::new(Monday, 9999, 52, Sunday).is_err());
        assert!(WeekDate::new(Monday, 9999, 53, Friday).is_err());

        assert!(WeekDate::new(Monday, -9999, 1, Monday).is_ok());
        assert!(WeekDate::new(Tuesday, -9999, 1, Tuesday).is_ok());
        assert!(WeekDate::new(Wednesday, -9999, 1, Wednesday).is_ok());
        assert!(WeekDate::new(Thursday, -9999, 1, Thursday).is_ok());
        assert!(WeekDate::new(Friday, -9999, 1, Friday).is_err());
        assert!(WeekDate::new(Saturday, -9999, 1, Saturday).is_err());
        assert!(WeekDate::new(Sunday, -9999, 1, Sunday).is_err());
    }

    #[test]
    fn boundaries_from_jiff() {
        let (min, max) = (jiff::civil::Date::MIN, jiff::civil::Date::MAX);

        assert!(WeekDate::from_date(Monday, min).is_ok());
        assert!(WeekDate::from_date(Monday, max).is_ok());

        assert!(WeekDate::from_date(Tuesday, min).is_err());
        assert!(WeekDate::from_date(Tuesday, max).is_ok());

        assert!(WeekDate::from_date(Wednesday, min).is_err());
        assert!(WeekDate::from_date(Wednesday, max).is_ok());

        assert!(WeekDate::from_date(Thursday, min).is_err());
        assert!(WeekDate::from_date(Thursday, max).is_ok());

        assert!(WeekDate::from_date(Friday, min).is_err());
        assert!(WeekDate::from_date(Friday, max).is_ok());

        assert!(WeekDate::from_date(Saturday, min).is_err());
        assert!(WeekDate::from_date(Saturday, max).is_ok());

        assert!(WeekDate::from_date(Sunday, min).is_err());
        assert!(WeekDate::from_date(Sunday, max).is_ok());
    }
}
