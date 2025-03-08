use std::io::Write;

use anyhow::Context;

use crate::{
    args::{self, Usage, flags, positional},
    datetime::DateTime,
    parse::OsStrExt,
    timezone::TimeZone,
};

const USAGE: &'static str = r#"
Convert a datetime to be in a particular time zone.

This command accepts either one time zone first and then one or more datetimes
to convert into that time zone, or one datetime first and then one or more time
zones to convert that datetime into.

USAGE:
    biff time in <time-zone> <datetime>...
    biff time in <time-zone> < line delimited <datetime>
    biff time in <datetime> <time-zone>...
    biff time in <datatime> < line delimited <time-zone>

TIP:
    use -h for short docs and --help for long docs

EXAMPLES:
    Print the current time in Hawaii:

        biff time in Pacific/Honolulu now

    When only one time zone and one datetime are given, flipping the
    arguments leads to equivalent results:

        biff time in now Pacific/Honolulu

    %snip-start%

    When a datetime is provided first, one can ask the time in one or more
    time zones:

        $ biff time in now Pacific/Honolulu Israel Asia/Kolkata US/Eastern
        2025-03-29T06:33:00.598956755-10:00[Pacific/Honolulu]
        2025-03-29T19:33:00.598956755+03:00[Israel]
        2025-03-29T22:03:00.598956755+05:30[Asia/Kolkata]
        2025-03-29T12:33:00.598956755-04:00[US/Eastern]

    One can use shell pipelines to reformat datetimes more succinctly:

        $ biff time in now Pacific/Honolulu Israel Asia/Kolkata US/Eastern \
            | biff time fmt -f '%Y-%m-%d %H:%M %Z'
        2025-03-29 06:33 HST
        2025-03-29 19:33 IDT
        2025-03-29 22:03 IST
        2025-03-29 12:33 EDT

    Or convert your current local time to UTC:

        $ biff time in now UTC
        2025-03-29T16:34:58.708156925+00:00[UTC]

    Or, if you want to represent an RFC 3339 timestamp whose offset to local
    time is not known, use the special `Etc/Unknown` time zone:

        $ biff time in now Etc/Unknown | biff time fmt -f rfc3339
        2025-03-29T16:36:52.132039046Z

    One can also convert multiple datetimes to a particular time zone at once:

        $ biff time in America/New_York now -1mo -1yr
        2025-03-29T12:37:50.029853159-04:00[America/New_York]
        2025-02-28T12:37:50.029853159-05:00[America/New_York]
        2024-03-29T12:37:50.029853159-04:00[America/New_York]

    %snip-end%
REQUIRED ARGUMENTS:
%args%
OPTIONS:
%flags%
"#;

pub fn run(p: &mut lexopt::Parser) -> anyhow::Result<()> {
    let mut config = Config::default();
    let mut args = positional::Arguments::default();
    args::configure(p, USAGE, &mut [&mut config, &mut args])?;

    let datetime_or_tz = config
        .datetime_or_tz
        .as_ref()
        .context("at least one datetime or time zone is required")?;
    let mut wtr = std::io::stdout().lock();
    args.try_map(|arg| {
        let sum = match *datetime_or_tz {
            flags::DateTimeOrTimeZone::DateTime(ref dt) => {
                arg.to_time_zone()?.try_map(|tz| Ok(dt.in_tz(&tz)))?
            }
            flags::DateTimeOrTimeZone::TimeZone(ref tz) => {
                arg.to_datetime()?.try_map(|dt| Ok(dt.in_tz(tz)))?
            }
        };
        sum.write(&mut wtr)?;
        writeln!(wtr)?;
        Ok(true)
    })
}

#[derive(Debug, Default)]
struct Config {
    datetime_or_tz: Option<flags::DateTimeOrTimeZone>,
}

impl args::Configurable for Config {
    fn configure(
        &mut self,
        _: &mut lexopt::Parser,
        arg: &mut lexopt::Arg,
    ) -> anyhow::Result<bool> {
        match *arg {
            lexopt::Arg::Value(ref mut v) => {
                if self.datetime_or_tz.is_some() {
                    return Ok(false);
                }
                self.datetime_or_tz = Some(v.parse()?);
            }
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn usage(&self) -> &[Usage] {
        &[TimeZone::ARG_OR_STDIN, DateTime::ARG_OR_STDIN]
    }
}
