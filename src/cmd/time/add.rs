use std::io::Write;

use anyhow::Context;

use crate::{
    args::{self, Usage, flags, positional},
    datetime::DateTime,
    parse::OsStrExt,
    span::TimeSpan,
};

const USAGE: &'static str = r#"
Add a span to a datetime.

This command accepts either one span first and then one or more datetimes, or
one datetime first and then one or more spans.

USAGE:
    biff time add <span> <datetime>...
    biff time add <span> < line delimited <datetime>
    biff time add <datetime> <span>...
    biff time add <datatime> < line delimited <span>

TIP:
    use -h for short docs and --help for long docs

EXAMPLES:
    Add 1 day to the current time:

        biff time add 1d now

    %snip-start%

    Or, equivalently:

        biff time add now 1d

    Subtract 1 day from the current time:

        biff time add -1d now

    Add 1 month to a particular date:

        biff time add 1mo 2024-01-31

    This command is time zone aware, even in extreme circumstances. For
    example, in 2011, Apia didn't have a December 30:

        $ biff time add '2011-12-31[Pacific/Apia]' -1ns
        2011-12-29T23:59:59.999999999-10:00[Pacific/Apia]

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

    let datetime_or_span = config
        .datetime_or_span
        .as_ref()
        .context("at least one datetime or time span is required")?;
    let mut wtr = std::io::stdout().lock();
    args.try_map(|arg| {
        let sum = match *datetime_or_span {
            flags::DateTimeOrSpan::DateTime(ref dt) => {
                arg.to_span()?.try_map(|span| dt.add(&span))?
            }
            flags::DateTimeOrSpan::TimeSpan(ref span) => {
                arg.to_datetime()?.try_map(|dt| dt.add(&span))?
            }
        };
        sum.write(&mut wtr)?;
        writeln!(wtr)?;
        Ok(true)
    })
}

#[derive(Debug, Default)]
struct Config {
    datetime_or_span: Option<flags::DateTimeOrSpan>,
}

impl args::Configurable for Config {
    fn configure(
        &mut self,
        _: &mut lexopt::Parser,
        arg: &mut lexopt::Arg,
    ) -> anyhow::Result<bool> {
        match *arg {
            lexopt::Arg::Value(ref mut v) => {
                if self.datetime_or_span.is_some() {
                    return Ok(false);
                }
                self.datetime_or_span = Some(v.parse()?);
            }
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn usage(&self) -> &[Usage] {
        &[TimeSpan::ARG_OR_STDIN, DateTime::ARG_OR_STDIN]
    }
}
