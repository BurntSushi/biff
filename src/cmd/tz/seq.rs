use std::io::Write;

use anyhow::Context;

use crate::{
    args::{self, Usage, positional},
    datetime::{DateTime, DateTimeFlexible},
    parse::OsStrExt,
    tag::MaybeTagged,
    timezone::TimeZone,
};

const USAGE_SEQ: &'static str = r#"
Emit a sequence of time zone transitions following a datetime.

The sequence emitted may be empty, for example, when there are no time zone
transitions after the given datetime in the given time zone. Time zone
transitions can be "missing" in common circumstances, such as for fixed offset
time zones or for time zones that have no daylight saving time.

By default, time zone transitions are shown that occur after the current time.
To change this reference point, use the `-r/--relative` flag.

USAGE:
    biff tz seq <time-zone>

TIP:
    use -h for short docs and --help for long docs

EXAMPLES:
    Show the next 5 time zone transitions in Sydney:

        $ biff tz seq -c5 Australia/Sydney
        2025-10-05T03:00:00+11:00[Australia/Sydney]
        2026-04-05T02:00:00+10:00[Australia/Sydney]
        2026-10-04T03:00:00+11:00[Australia/Sydney]
        2027-04-04T02:00:00+10:00[Australia/Sydney]
        2027-10-03T03:00:00+11:00[Australia/Sydney]

    %snip-start%

    Notice that in the example above, the datetimes are printed the time zone
    requested. You can of course switch them to your system's time zone:

        $ biff tz seq -c5 Australia/Sydney | biff time in system
        2025-10-04T12:00:00-04:00[America/New_York]
        2026-04-04T12:00:00-04:00[America/New_York]
        2026-10-03T12:00:00-04:00[America/New_York]
        2027-04-03T12:00:00-04:00[America/New_York]
        2027-10-02T12:00:00-04:00[America/New_York]

    %snip-end%
REQUIRED ARGUMENTS:
%args%
OPTIONS:
%flags%
"#;

const USAGE_NEXT: &'static str = r#"
Find next time zone transitions following one or more datetimes.

If there is no next time zone transition, then no datetime is emitted. For
tagged data, this results in an item without tags. Time zone transitions can
be "missing" in common circumstances, such as for fixed offset time zones or
for time zones that have no daylight saving time.

USAGE:
    biff tz next <time-zone> <datetime>...
    biff tz next <time-zone> < line delimited <datetime>

TIP:
    use -h for short docs and --help for long docs

EXAMPLES:
    Find the next time the time zone offset changes in New York:

        $ biff tz next America/New_York now

    %snip-start%

    The `--inclusive` flag can be used to change this command from "find
    transitions after a datetime" to "find transitions after or at a datetime."
    For example, without it:

        $ biff tz next America/New_York 2025-03-09T03:00-04
        2025-11-02T01:00:00-05:00[America/New_York]

    But with it:

        $ biff tz next -i America/New_York 2025-03-09T03:00-04
        2025-03-09T03:00:00-04:00[America/New_York]

    %snip-end%
REQUIRED ARGUMENTS:
%args%
OPTIONS:
%flags%
"#;

const USAGE_PREV: &'static str = r#"
Find previous time zone transitions preceding one or more datetimes.

If there is no previous time zone transition, then no datetime is emitted. For
tagged data, this results in an item without tags. Time zone transitions can
be "missing" in common circumstances, such as for fixed offset time zones or
for time zones that have no daylight saving time.

USAGE:
    biff tz prev <time-zone> <datetime>...
    biff tz prev <time-zone> < line delimited <datetime>

TIP:
    use -h for short docs and --help for long docs

EXAMPLES:
    Find the previous time the time zone offset changed in New York:

        $ biff tz prev America/New_York now

    %snip-start%

    The `--inclusive` flag can be used to change this command from "find
    transitions before a datetime" to "find transitions before or at a
    datetime." For example, without it:

        $ biff tz prev America/New_York 2025-03-09T03:00-04
        2024-11-03T01:00:00-05:00[America/New_York]

    But with it:

        $ biff tz prev -i America/New_York 2025-03-09T03:00-04
        2025-03-09T03:00:00-04:00[America/New_York]

    %snip-end%
REQUIRED ARGUMENTS:
%args%
OPTIONS:
%flags%
"#;

const INCLUSIVE: Usage = Usage::flag(
    "-i/--inclusive",
    "Include time zone transitions equal to the given datetimes.",
    r#"
Include time zone transitions equal to the given datetimes.

By default, this command always returns a transition that is strictly greater
(for `biff tz next`) or strictly less than (for `biff tz prev`) than the given
datetimes. When this flag is given, the command is permitted to return
transitions that are greater than or equal or less than or equal to the given
datetimes.
"#,
);

const NTH: Usage = Usage::flag(
    "-c/--count",
    "Returns the nth time zone transition after the given datetimes.",
    r#"
Returns the nth time zone transition after the given datetimes.

The value given must be greater than zero. A value of 1 is the default and
means the first time zone transition after (for `biff tz next`) or before (for
`biff tz prev`) the given datetimes.
"#,
);

const COUNT: Usage = Usage::flag(
    "-c/--count",
    "Shows only the next (or previous) N transitions.",
    r#"
Shows only the next (or previous) N transitions.

The value may be zero. By default, all transitions before Biff's maximum
datetime (or after tzdb's minimum transition when `-p/--past` is given) are
shown.
"#,
);

const PAST: Usage = Usage::flag(
    "-p/--past",
    "Show time zone transitions before the given datetime.",
    r#"
Show time zone transitions before the given datetime.

When no datetime is given, it defaults to now. Therefore, this flag defaults
to showing time zone transitions in the past relative to the current time.
"#,
);

pub fn seq(p: &mut lexopt::Parser) -> anyhow::Result<()> {
    let mut config = Seq::default();
    args::configure(p, USAGE_SEQ, &mut [&mut config])?;

    let tz =
        config.tz.as_ref().context("missing required <time-zone> argument")?;
    let count = config.count.unwrap_or(usize::MAX);
    let relative = config.relative()?;
    let mut wtr = std::io::stdout().lock();
    if config.past {
        for dt in relative.tz_preceding(tz).take(count) {
            writeln!(wtr, "{dt}")?;
        }
    } else {
        for dt in relative.tz_following(tz).take(count) {
            writeln!(wtr, "{dt}")?;
        }
    }
    Ok(())
}

pub fn next(p: &mut lexopt::Parser) -> anyhow::Result<()> {
    let mut config = NextOrPrev::default();
    let mut datetimes = positional::DateTimes::default();
    args::configure(p, USAGE_NEXT, &mut [&mut config, &mut datetimes])?;

    let tz =
        config.tz.as_ref().context("missing required <time-zone> argument")?;
    let nth = config.nth;
    let mut wtr = std::io::stdout().lock();
    datetimes.try_map(|datum| {
        match datum {
            MaybeTagged::Untagged(dt) => {
                let relative = config.relative_or_before(dt)?;
                if let Some(next) = relative.tz_following(tz).nth(nth) {
                    writeln!(wtr, "{next}")?;
                }
            }
            MaybeTagged::Tagged(mut tagged) => {
                tagged.retain(|dt| {
                    let Ok(relative) = config.relative_or_before(dt.clone())
                    else {
                        return false;
                    };
                    let Some(next) = relative.tz_following(tz).nth(nth) else {
                        return false;
                    };
                    *dt = next;
                    true
                });
                tagged.write(&mut wtr)?;
                writeln!(wtr)?;
            }
        }
        Ok(true)
    })?;
    Ok(())
}

pub fn prev(p: &mut lexopt::Parser) -> anyhow::Result<()> {
    let mut config = NextOrPrev::default();
    let mut datetimes = positional::DateTimes::default();
    args::configure(p, USAGE_PREV, &mut [&mut config, &mut datetimes])?;

    let tz =
        config.tz.as_ref().context("missing required <time-zone> argument")?;
    let nth = config.nth;
    let mut wtr = std::io::stdout().lock();
    datetimes.try_map(|datum| {
        match datum {
            MaybeTagged::Untagged(dt) => {
                let relative = config.relative_or_after(dt)?;
                if let Some(next) = relative.tz_preceding(tz).nth(nth) {
                    writeln!(wtr, "{next}")?;
                }
            }
            MaybeTagged::Tagged(mut tagged) => {
                tagged.retain(|dt| {
                    let Ok(relative) = config.relative_or_after(dt.clone())
                    else {
                        return false;
                    };
                    let Some(next) = relative.tz_preceding(tz).nth(nth) else {
                        return false;
                    };
                    *dt = next;
                    true
                });
                tagged.write(&mut wtr)?;
                writeln!(wtr)?;
            }
        }
        Ok(true)
    })?;
    Ok(())
}

#[derive(Debug, Default)]
struct Seq {
    tz: Option<TimeZone>,
    relative: DateTime,
    inclusive: bool,
    count: Option<usize>,
    past: bool,
}

impl Seq {
    fn relative(&self) -> anyhow::Result<DateTime> {
        if !self.inclusive {
            Ok(self.relative.clone())
        } else if self.past {
            self.relative.instant_after()
        } else {
            self.relative.instant_before()
        }
    }
}

impl args::Configurable for Seq {
    fn configure(
        &mut self,
        p: &mut lexopt::Parser,
        arg: &mut lexopt::Arg,
    ) -> anyhow::Result<bool> {
        match *arg {
            lexopt::Arg::Short('r') | lexopt::Arg::Long("relative") => {
                let relative: DateTimeFlexible =
                    args::parse(p, "-r/--relative")?;
                self.relative = relative.into();
            }
            lexopt::Arg::Value(ref mut v) => {
                if self.tz.is_some() {
                    return Ok(false);
                }
                self.tz = Some(v.parse()?);
            }
            lexopt::Arg::Short('i') | lexopt::Arg::Long("inclusive") => {
                self.inclusive = true;
            }
            lexopt::Arg::Short('c') | lexopt::Arg::Long("count") => {
                self.count = Some(args::parse(p, "-c/--count")?);
            }
            lexopt::Arg::Short('p') | lexopt::Arg::Long("past") => {
                self.past = true;
            }
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn usage(&self) -> &[Usage] {
        &[TimeZone::ARG, DateTime::RELATIVE_FLAG, INCLUSIVE, COUNT, PAST]
    }
}

#[derive(Debug, Default)]
struct NextOrPrev {
    tz: Option<TimeZone>,
    inclusive: bool,
    nth: usize,
}

impl NextOrPrev {
    fn relative_or_before(&self, dt: DateTime) -> anyhow::Result<DateTime> {
        if !self.inclusive { Ok(dt) } else { dt.instant_before() }
    }

    fn relative_or_after(&self, dt: DateTime) -> anyhow::Result<DateTime> {
        if !self.inclusive { Ok(dt) } else { dt.instant_after() }
    }
}

impl args::Configurable for NextOrPrev {
    fn configure(
        &mut self,
        p: &mut lexopt::Parser,
        arg: &mut lexopt::Arg,
    ) -> anyhow::Result<bool> {
        match *arg {
            lexopt::Arg::Value(ref mut v) => {
                if self.tz.is_some() {
                    return Ok(false);
                }
                self.tz = Some(v.parse()?);
            }
            lexopt::Arg::Short('i') | lexopt::Arg::Long("inclusive") => {
                self.inclusive = true;
            }
            lexopt::Arg::Short('c') | lexopt::Arg::Long("count") => {
                let count: usize = args::parse(p, "-c/--count")?;
                self.nth = count
                    .checked_sub(1)
                    .context("-c/--count must be greater than zero")?;
            }
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn usage(&self) -> &[Usage] {
        &[TimeZone::ARG, DateTime::ARG_OR_STDIN, INCLUSIVE, NTH]
    }
}
