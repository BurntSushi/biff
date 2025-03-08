use std::io::Write;

use crate::{
    args::{self, Usage, flags, positional},
    datetime::DateTime,
};

const USAGE: &'static str = r#"
Print a datetime in a particular format.

This accepts one or more datetimes as positional arguments. When no positional
arguments are given, then line delimited datetimes are read from stdin.

USAGE:
    biff time fmt <datetime>...
    biff time fmt < line delimited <datetime>

TIP:
    use -h for short docs and --help for long docs

EXAMPLES:
    Format the current time as a date with the month written out:

        $ biff time fmt -f '%B %d, %Y' now

    %snip-start%

    Format the current time as an RFC 2822 timestamp using the correct local
    offset for that instant in your time zone:

        $ biff time fmt -f rfc2822 now

    Do the same as above, but for one month ago:

        $ biff time fmt -f rfc2822 -1mo

    Format the first instant of the given date, in your local time, as an
    RFC 9557 timestamp:

        $ biff time fmt -f rfc9557 2025-03-15

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

    let jiff_strtime_config = crate::locale::jiff_strtime_config()?;
    let mut wtr = std::io::stdout().lock();
    datetimes.try_map(|datum| {
        let formatted = datum.try_map(|datetime| {
            config.format.format(&jiff_strtime_config, &datetime)
        })?;
        formatted.write(&mut wtr)?;
        writeln!(wtr)?;
        Ok(true)
    })?;
    Ok(())
}

#[derive(Debug, Default)]
struct Config {
    format: flags::Format,
}

impl args::Configurable for Config {
    fn configure(
        &mut self,
        p: &mut lexopt::Parser,
        arg: &mut lexopt::Arg,
    ) -> anyhow::Result<bool> {
        match *arg {
            lexopt::Arg::Short('f') | lexopt::Arg::Long("format") => {
                self.format = args::parse(p, "-f/--format")?;
            }
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn usage(&self) -> &[Usage] {
        &[DateTime::ARG_OR_STDIN, flags::Format::USAGE_PRINT]
    }
}
