use std::io::Write;

use anyhow::Context;

use crate::{
    args::{
        self, Usage,
        flags::{self, ByWeekdays, CommaSequence, NumberRange},
        positional,
    },
    datetime::{DateTime, DateTimeFlexible},
    ical::{Frequency, RecurrenceRule},
    parse::OsStrExt,
};

const USAGE: &'static str = r#"
Generate a sequence of datetimes using RFC 5545 recurrence rules.

Datetimes are generated in chronological order at a given frequency from the
given starting point. If a starting point is not given, then the current time
is used.

Unless the `-c/--count` or `--until` flags are used, this command will
generate datetimes until Biff's maximum is reached. In lieu of `-c/--count`,
users may also choose to use programs like `head` to limit the output.

USAGE:
    biff time seq <frequency> <datetime>

TIP:
    use -h for short docs and --help for long docs

EXAMPLES:
    Print all Friday the 13th occurrences for the next 3 years:

        $ biff time seq monthly --until 3y -w fri -d 13
        2025-06-13T20:57:20.062111223-04:00[America/New_York]
        2026-02-13T20:57:20.062111223-05:00[America/New_York]
        2026-03-13T20:57:20.062111223-04:00[America/New_York]
        2026-11-13T20:57:20.062111223-05:00[America/New_York]
        2027-08-13T20:57:20.062111223-04:00[America/New_York]

    %snip-start%

    Schedule my cat's medication, every Monday, Wednesday and Friday with
    breakfest, for the next two weeks, starting tomorrow:

        $ biff time seq daily -w mon,wed,fri -H 8 -M 30 --until 2wks tomorrow
        2025-04-18T08:30:00-04:00[America/New_York]
        2025-04-21T08:30:00-04:00[America/New_York]
        2025-04-23T08:30:00-04:00[America/New_York]
        2025-04-25T08:30:00-04:00[America/New_York]
        2025-04-28T08:30:00-04:00[America/New_York]
        2025-04-30T08:30:00-04:00[America/New_York]

    Find the last work-day of the current month:

        $ biff time seq monthly --count 1 -w mon..fri --set-position -1
        2025-04-30T21:27:39.66489192-04:00[America/New_York]

    Find the last Saturday every other month, starting with the current month,
    for the next year:

        $ biff time seq monthly -i2 -w -1-sat --until 1y
        2025-04-26T21:44:16.816662841-04:00[America/New_York]
        2025-06-28T21:44:16.816662841-04:00[America/New_York]
        2025-08-30T21:44:16.816662841-04:00[America/New_York]
        2025-10-25T21:44:16.816662841-04:00[America/New_York]
        2025-12-27T21:44:16.816662841-05:00[America/New_York]
        2026-02-28T21:44:16.816662841-05:00[America/New_York]

    Generate every day remaining in the current month:

        $ biff time seq daily --until $(biff time end-of month now) today
        2025-04-17T00:00:00-04:00[America/New_York]
        2025-04-18T00:00:00-04:00[America/New_York]
        2025-04-19T00:00:00-04:00[America/New_York]
        2025-04-20T00:00:00-04:00[America/New_York]
        2025-04-21T00:00:00-04:00[America/New_York]
        2025-04-22T00:00:00-04:00[America/New_York]
        2025-04-23T00:00:00-04:00[America/New_York]
        2025-04-24T00:00:00-04:00[America/New_York]
        2025-04-25T00:00:00-04:00[America/New_York]
        2025-04-26T00:00:00-04:00[America/New_York]
        2025-04-27T00:00:00-04:00[America/New_York]
        2025-04-28T00:00:00-04:00[America/New_York]
        2025-04-29T00:00:00-04:00[America/New_York]
        2025-04-30T00:00:00-04:00[America/New_York]

    Or every day in the current month, including days in the past:

        $ biff time seq daily --until $(biff time end-of month now) $(biff time start-of month today)
        2025-04-01T00:00:00-04:00[America/New_York]
        2025-04-02T00:00:00-04:00[America/New_York]
        2025-04-03T00:00:00-04:00[America/New_York]
        2025-04-04T00:00:00-04:00[America/New_York]
        2025-04-05T00:00:00-04:00[America/New_York]
        2025-04-06T00:00:00-04:00[America/New_York]
        2025-04-07T00:00:00-04:00[America/New_York]
        2025-04-08T00:00:00-04:00[America/New_York]
        2025-04-09T00:00:00-04:00[America/New_York]
        2025-04-10T00:00:00-04:00[America/New_York]
        ...

    %snip-end%
REQUIRED ARGUMENTS:
%args%
OPTIONS:
%flags%
"#;

pub fn run(p: &mut lexopt::Parser) -> anyhow::Result<()> {
    let mut config = Config::default();
    let mut datetimes = positional::DateTimes::default();
    args::configure(p, USAGE, &mut [&mut config, &mut datetimes])?;

    let mut wtr = std::io::stdout().lock();
    let terminates = &config.terminates;
    let rrule = config.recurrence_rule()?;
    for dt in rrule.iter().map(DateTime::from).take(terminates.count()) {
        writeln!(wtr, "{dt}")?;
    }
    Ok(())
}

#[derive(Debug, Default)]
struct Config {
    freq: Option<Frequency>,
    start: Option<DateTime>,
    terminates: Termination,
    interval: Option<i32>,
    by_month: Vec<CommaSequence<NumberRange<flags::Month>>>,
    by_week: Vec<CommaSequence<NumberRange<i8>>>,
    by_year_day: Vec<CommaSequence<NumberRange<i16>>>,
    by_month_day: Vec<CommaSequence<NumberRange<i8>>>,
    by_week_day: Vec<CommaSequence<ByWeekdays>>,
    by_hour: Vec<CommaSequence<NumberRange<i8>>>,
    by_minute: Vec<CommaSequence<NumberRange<i8>>>,
    by_second: Vec<CommaSequence<NumberRange<i8>>>,
    by_set_pos: Vec<CommaSequence<NumberRange<i32>>>,
    week_start: flags::Weekday,
}

impl Config {
    fn recurrence_rule(&self) -> anyhow::Result<RecurrenceRule> {
        let mut b =
            RecurrenceRule::builder(self.freq()?, self.start().get().clone());
        b.interval(self.interval()).week_start(self.week_start.get());
        // It's kind of annoying that we can't just pass these iterators to
        // `b.by_whatever` directly. I tried adding the requisite trait impls,
        // but the orphan rules forbid it. I didn't try very hard though.
        for range in self.by_month.iter().flatten().map(|v| v.range()) {
            b.by_month(range.start().get()..=range.end().get());
        }
        for range in self.by_week.iter().flatten().map(|v| v.range()) {
            b.by_week(range);
        }
        for range in self.by_year_day.iter().flatten().map(|v| v.range()) {
            b.by_year_day(range);
        }
        for range in self.by_month_day.iter().flatten().map(|v| v.range()) {
            b.by_month_day(range);
        }
        for &byweekdays in self.by_week_day.iter().flatten() {
            match byweekdays {
                ByWeekdays::Range { start, end } => {
                    b.by_week_day(start..=end);
                }
                ByWeekdays::Singleton(singleton) => {
                    b.by_week_day(singleton);
                }
            }
        }
        for range in self.by_hour.iter().flatten().map(|v| v.range()) {
            b.by_hour(range);
        }
        for range in self.by_minute.iter().flatten().map(|v| v.range()) {
            b.by_minute(range);
        }
        for range in self.by_second.iter().flatten().map(|v| v.range()) {
            b.by_second(range);
        }
        for range in self.by_set_pos.iter().flatten().map(|v| v.range()) {
            b.by_set_position(range);
        }
        if let Termination::Until(ref until) = self.terminates {
            b.until(until.get().clone());
        }
        b.build()
    }

    fn freq(&self) -> anyhow::Result<Frequency> {
        self.freq.context("missing required <frequency>")
    }

    fn start(&self) -> DateTime {
        self.start.clone().unwrap_or_else(|| DateTime::now())
    }

    fn interval(&self) -> i32 {
        self.interval.unwrap_or(1)
    }
}

impl args::Configurable for Config {
    fn configure(
        &mut self,
        p: &mut lexopt::Parser,
        arg: &mut lexopt::Arg,
    ) -> anyhow::Result<bool> {
        use lexopt::Arg::*;

        match *arg {
            Value(ref v) => {
                if self.freq.is_none() {
                    self.freq = Some(v.to_str()?.parse()?);
                    return Ok(true);
                }
                if self.start.is_none() {
                    let dt: DateTimeFlexible = v.parse()?;
                    self.start = Some(dt.into());
                    return Ok(true);
                }
                return Ok(false);
            }
            Short('u') | Long("until") => {
                anyhow::ensure!(
                    !matches!(self.terminates, Termination::Count(_)),
                    "the -u/--until flag cannot be used with -c/--count",
                );
                let until: DateTimeFlexible = args::parse(p, "-u/--until")?;
                self.terminates = Termination::Until(until.into());
            }
            Short('c') | Long("count") => {
                anyhow::ensure!(
                    !matches!(self.terminates, Termination::Until(_)),
                    "the -c/--count flag cannot be used with -u/--until",
                );
                self.terminates =
                    Termination::Count(args::parse(p, "-c/--count")?);
            }
            Short('i') | Long("interval") => {
                self.interval = Some(args::parse(p, "-i/--interval")?);
            }
            Short('m') | Long("month") => {
                self.by_month.push(args::parse(p, "-m/--month")?);
            }
            Long("week") => {
                self.by_week.push(args::parse(p, "--week")?);
            }
            Long("doy") => {
                self.by_year_day.push(args::parse(p, "--doy")?);
            }
            Short('d') | Long("day") => {
                self.by_month_day.push(args::parse(p, "-d/--day")?);
            }
            Short('w') | Long("week-day") => {
                self.by_week_day.push(args::parse(p, "-w/--week-day")?);
            }
            Short('H') | Long("hour") => {
                self.by_hour.push(args::parse(p, "-H/--hour")?);
            }
            Short('M') | Long("minute") => {
                self.by_minute.push(args::parse(p, "-M/--minute")?);
            }
            Short('S') | Long("second") => {
                self.by_second.push(args::parse(p, "-S/--second")?);
            }
            Long("set-position") => {
                self.by_set_pos.push(args::parse(p, "--set-position")?);
            }
            Long("week-start") => {
                self.week_start = args::parse(p, "--week-start")?;
            }
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn usage(&self) -> &[Usage] {
        const INTERVAL: Usage = Usage::flag(
            "-i/--interval <number>",
            "Sets the interval at which the sequence repeats.",
            r#"
Sets the interval at which the sequence repeats.

Zero is a legal value, but always results in an empty sequence.
"#,
        );
        const BY_MONTH: Usage = Usage::flag(
            "-m/--month <month-list>",
            "Provide one or more months of the year.",
            r#"
Provide one or more months of the year.

Legal values are the integers 1 through 12.

Contiguous ranges of months may be specified. For example, `5..7` corresponds
to the months May, June and July.

Multiple months or ranges can be specified with repeated use of this flag, or
by separating values with a comma. For example, `2,5..7,12` corresponds to the
months February, May, June, July and December.

When generating a sequence at yearly frequency, this expands the set of
datetimes generated at each interval. Otherwise, this limits the set of
datetimes generated at each interval.
"#,
        );
        const BY_WEEK: Usage = Usage::flag(
            "--week <week-number-list>",
            "Provide one or more weeks of the year.",
            r#"
Provide one or more weeks of the year.

Legal values are the integers 1 through 53 or -53 through -1. Negative weeks
count backwards from the end. A 53rd week only exists in "long" years. That
is, for weeks starting on a Monday, years for which Jan 1 is a Thursday or
leap years for which Jan 1 is a Wednesday.

Users can change the start of the week to any day via `--week-start`. The
default is to begin the week on Monday.

Contiguous ranges of weeks may be specified. For example, `5..7` corresponds
to the 5th, 6th and 7th weeks of the year.

Multiple weeks or ranges can be specified with repeated use of this flag, or
by separating values with a comma. For example, `2,5..7,-1` corresponds to the
weeks 2, 5, 6, 7 and the last week of the year.

When generating a sequence at yearly frequency, this expands the set of
datetimes generated at each interval to all of the days in the week.

This flag cannot be used with anything other than yearly frequency.
"#,
        );
        const BY_YEAR_DAY: Usage = Usage::flag(
            "--doy <day-of-year-list>",
            "Provide one or more days of the year.",
            r#"
Provide one or more days of the year.

Legal values are the integers 1 through 366 or -366 through -1. Negative days
count backwards from the end of a year. A 366th day only exists in leap years.

Contiguous ranges of days may be specified. For example, `100..102` corresponds
to the 100th, 101st and 102nd days of the year.

Multiple days or ranges can be specified with repeated use of this flag, or
by separating values with a comma. For example, `1,100..102,-1` corresponds to
the first and last days of the year, along with the 100th, 101st and 102nd
days of the year.

When generating a sequence at yearly frequency, this expands the set of
datetimes generated at each interval. Otherwise, this limits the set of
datetimes.

This flag can only be used with yearly, hourly, minutely or secondly frequency.
"#,
        );
        const BY_MONTH_DAY: Usage = Usage::flag(
            "-d/--day <day-of-month-list>",
            "Provide one or more days of the month.",
            r#"
Provide one or more days of the month.

Legal values are the integers 1 through 31 or -31 through -1. Negative days
count backwards from the end of a month. For some months, the days 29, 30 or
31 may not exist. (The integers are still accepted, but dates with those days
won't be included in the sequence returned.)

Contiguous ranges of days may be specified. For example, `13..15` corresponds
to the 13th, 14th and 15th days of the month.

Multiple days or ranges can be specified with repeated use of this flag, or by
separating values with a comma. For example, `1,13..15,-1` corresponds to the
first and last days of the month, along with the 13th, 14th and 15th days of
the month.

When generating a sequence at a monthly or yearly frequency, this expands the
set of datetimes generated at each interval. Otherwise, this limits the set of
datetimes.

This flag cannot be used with weekly frequency.
"#,
        );
        const BY_WEEK_DAY: Usage = Usage::flag(
            "-w/--week-day <week-day-list>",
            "Provide one or more days of the week.",
            r#"
Provide one or more days of the week.

Legal values are any day of the week (e.g., sun, mon, tue, wed, thu, fri, sat)
or a numbered day of the week (e.g., 1-fri, -1-fri, 3-tue). Negative numbered
days count backwards from the end of the year or month.

Contiguous ranges of weekdays may be specified. For example, `Mon..Wed`
corresponds Monday, Tuesday and Wednesday. Ranges of numbered weekdays are
not allowed.

Multiple weekdays or ranges can be specified with repeated use of this flag, or
by separating values with a comma. For example, `Sun,Tue-Thu,Sat` corresponds
to every day of the week except for Monday and Friday.

When generating a sequence at a daily or shorter frequency, this limits the set
of datetimes generated at each interval. At a weekly frequency, this expands
the set of datetimes generated. At a monthly frequency, this limits when a day
of the month is set, and otherwise expands. At a yearly frequency, this limits
when a day of the year or month is set, and otherwise expands.

Numbered weekdays can only be used at monthly or yearly frequencies. And when
at a yearly frequency, this can't be used with week numbers.
"#,
        );
        const BY_HOUR: Usage = Usage::flag(
            "-H/--hour <hour-list>",
            "Provide one or more hours of the day.",
            r#"
Provide one or more hours of the day.

Legal values are the integers 0 through 23.

Contiguous ranges of hours may be specified. For example, `13..15` corresponds
to the 13th, 14th and 15th hours of the day.

Multiple hours or ranges can be specified with repeated use of this flag, or by
separating values with a comma. For example, `0,13..15,23` corresponds to the
first and last hours of the day, along with the 13th, 14th and 15th hours of
the day.

When generating a sequence at a daily or greater frequency, this expands the
set of datetimes generated at each interval. Otherwise, this limits the set of
datetimes.
"#,
        );
        const BY_MINUTE: Usage = Usage::flag(
            "-M/--minute <minute-list>",
            "Provide one or more minutes of the hour.",
            r#"
Provide one or more minutes of the hour.

Legal values are the integers 0 through 59.

Contiguous ranges of minutes may be specified. For example, `13..15`
corresponds to the 13th, 14th and 15th minutes of the hour.

Multiple minutes or ranges can be specified with repeated use of this flag,
or by separating values with a comma. For example, `0,13..15,59` corresponds
to the first and last minutes of the hour, along with the 13th, 14th and 15th
minutes of the hour.

When generating a sequence at a hourly or greater frequency, this expands the
set of datetimes generated at each interval. Otherwise, this limits the set of
datetimes.
"#,
        );
        const BY_SECOND: Usage = Usage::flag(
            "-S/--second <second-list>",
            "Provide one or more seconds of the minute.",
            r#"
Provide one or more seconds of the minute.

Legal values are the integers 0 through 59.

Contiguous ranges of seconds may be specified. For example, `13..15`
corresponds to the 13th, 14th and 15th seconds of the minute.

Multiple seconds or ranges can be specified with repeated use of this flag, or
by separating values with a comma. For example, `0,13..15,59` corresponds to
the first and last seconds of the minute, along with the 13th, 14th and 15th
seconds of the minute.

When generating a sequence at a minutely or greater frequency, this expands the
set of datetimes generated at each interval. Otherwise, this limits the set of
datetimes.
"#,
        );
        const BY_SET_POS: Usage = Usage::flag(
            "--set-position <number-list>",
            "Provide one or more positions in a recurrence set.",
            r#"
Provide one or more positions in a recurrence set.

Legal values are positive or negative integers. Positive integers refer to
the Nth datetime in a particular interval, counting chronologically forward.
Negative integers are the same, but count chronologically backward.

Contiguous ranges of set positions may be specified. For example, `100..102`
corresponds to the 100th, 101st and 102nd positions.

Multiple positions or ranges can be specified with repeated use of this
flag, or by separating values with a comma. For example, `1,100..102,-1`
corresponds to the first and last positions, along with the 100th, 101st and
102nd positions.

This always has the effect of limiting the sequence of datetimes generated.

This flag can only be used in conjunction with some other rule.
"#,
        );

        &[
            Frequency::USAGE,
            DateTime::ARG,
            INTERVAL,
            Termination::USAGE_UNTIL,
            Termination::USAGE_COUNT,
            flags::Weekday::USAGE_WEEK_START,
            BY_MONTH,
            BY_WEEK,
            BY_YEAR_DAY,
            BY_MONTH_DAY,
            BY_WEEK_DAY,
            BY_HOUR,
            BY_MINUTE,
            BY_SECOND,
            BY_SET_POS,
        ]
    }
}

#[derive(Clone, Debug, Default)]
enum Termination {
    #[default]
    Never,
    Until(DateTime),
    Count(usize),
}

impl Termination {
    const USAGE_UNTIL: Usage = Usage::flag(
        "-u/--until <datetime>",
        "Repeat a sequence until this datetime (inclusive).",
        r#"
Repeat a sequence until this datetime (inclusive).

This flag conflicts with `-c/--count`. That is, one or the other can be set
(or neither), but not both.
"#,
    );

    const USAGE_COUNT: Usage = Usage::flag(
        "-c/--count <number>",
        "Repeat a sequence to generate this number of datetimes.",
        r#"
Repeat a sequence to generate this number of datetimes.

Zero is a legal value, but always results in an empty sequence.

This flag conflicts with `-u/--until`. That is, one or the other can be set
(or neither), but not both.
"#,
    );

    /// Returns a finite count on how many datetimes should be emitted.
    ///
    /// If no finite count can be determined then `usize::MAX` is returned.
    fn count(&self) -> usize {
        match *self {
            Termination::Never | Termination::Until(_) => usize::MAX,
            Termination::Count(count) => count,
        }
    }
}
