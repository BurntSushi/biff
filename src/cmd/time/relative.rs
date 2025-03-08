use std::io::Write;

use anyhow::Context;

use crate::{
    args::{self, Usage, positional},
    datetime::{DateTime, DateTimeFlexible},
    parse::OsStrExt,
};

const USAGE: &'static str = r#"
Interpret a relative description of a datetime with one or more datetimes given
as reference points.

USAGE:
    biff time relative <relative-description> <datetime>...
    biff time relative <relative-description> < line delimited <datetime>

TIP:
    use -h for short docs and --help for long docs

EXAMPLES:
    Get the first Monday on or after a particular date:

        $ biff time relative 'this monday' 2025-04-22
        2025-04-28T00:00:00-04:00[America/New_York]

REQUIRED ARGUMENTS:
%args%
OPTIONS:
%flags%
"#;

pub fn run(p: &mut lexopt::Parser) -> anyhow::Result<()> {
    let mut config = Config::default();
    let mut datetimes = positional::DateTimes::default();
    args::configure(p, USAGE, &mut [&mut config, &mut datetimes])?;

    let relative = config
        .relative
        .as_ref()
        .context("missing required <relative> argument")?;
    let mut wtr = std::io::stdout().lock();
    datetimes.try_map(|datum| {
        let dt = datum.try_map(|dt| {
            DateTimeFlexible::parse_only_relative(dt.get(), relative)
                .map(DateTime::from)
        })?;
        dt.write(&mut wtr)?;
        writeln!(wtr)?;
        Ok(true)
    })?;
    Ok(())
}

#[derive(Debug, Default)]
struct Config {
    relative: Option<Vec<u8>>,
}

impl args::Configurable for Config {
    fn configure(
        &mut self,
        _: &mut lexopt::Parser,
        arg: &mut lexopt::Arg,
    ) -> anyhow::Result<bool> {
        match *arg {
            lexopt::Arg::Value(ref v) => {
                if self.relative.is_none() {
                    self.relative = Some(v.to_bytes()?.to_vec());
                    return Ok(true);
                }
                Ok(false)
            }
            _ => Ok(false),
        }
    }

    fn usage(&self) -> &[Usage] {
        const RELATIVE_DESCRIPTION: Usage = Usage::arg(
            "<relative-description>",
            "A relative description of a datetime.",
            r#"
A relative description of a datetime. This can be a time span, a special
string like `yesterday` or `tomorrow`, or even just a time or a weekday.

A time span is a convenient way to write dates relative to a particular time.
For example, to get 1 day from the current time, you can use `1 day`, or more
succinctly, `1d`. To get 1 day in the past from the current time, you can use
`1 day ago`, or more succinctly, `-1d`. Mixing calendar and time units is
allowed, for example, `1 year 1 second` or `1y1s`.

Some special strings are supported as well:

`now` refers to the current datetime to the highest precision supported by
your system. The current datetime is computed once when Biff starts, or if the
`BIFF_NOW` environment variable is set, that time is used instead.

`today` refers to the first instant of the current day.

`yesterday` refers to the first instant of the previous day.

`tomorrow` refers to the first instant of the next day.

Other examples of things that work:

`this thurs` refers to the current day (if it's a Thursday) or the soonest
date that falls on a Thursday.

`last FRIDAY` refers to the previously occurring Friday, up to 1 week in the
past (if the current day is a Friday).

`next saturday` refers to the next Saturday, up to 1 week in the future (if
the current day is a Saturday).

`5pm tomorrow`, `5pm next Wed` or `5pm 1 week` refer to 5pm tomorrow, 5pm on
"#,
        );
        &[RELATIVE_DESCRIPTION, DateTime::ARG_OR_STDIN]
    }
}
