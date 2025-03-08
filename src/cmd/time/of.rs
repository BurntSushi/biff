use std::io::Write;

use anyhow::Context;

use crate::{
    args::{self, Usage, flags, positional},
    datetime::DateTime,
    parse::OsStrExt,
};

const USAGE_START_OF: &'static str = r#"
Print the start of a particular calendar or time unit.

This command makes it easy to "snap" datetimes to the beginning of a particular
period, such as a year, month, week, day, hour and so on. The beginning of a
period is defined to be the first nanosecond in that period.

This accepts one or more datetimes as positional arguments. When no positional
arguments are given, then line delimited datetimes are read from stdin.

USAGE:
    biff time start-of <datetime>...
    biff time start-of < line delimited <datetime>

TIP:
    use -h for short docs and --help for long docs

EXAMPLES:
    Print the first instant of the current day:

        $ biff time start-of day now
        2025-04-02T00:00:00-04:00[America/New_York]

    %snip-start%

    Print the first instant of the current week, for weeks starting with
    Sunday:

        $ biff time start-of week-sunday now
        2025-03-30T00:00:00-04:00[America/New_York]

    Or, the same, but for weeks starting with Monday:

        $ biff time start-of week-monday now
        2025-03-31T00:00:00-04:00[America/New_York]

    This command is aware of time zone transitions. For example, on
    2015-10-18, Sao Paulo entered DST. Unlike most places, they set their
    clocks forward at midnight, which means the midnight hour never actually
    occurs on their clocks. So if you ask Biff for the start of 2015-10-18 in
    Sao Paulo's time zone, it will correctly report 1am:

        $ biff time start-of day '2015-10-18[America/Sao_Paulo]'
        2015-10-18T01:00:00-02:00[America/Sao_Paulo]

    %snip-end%
REQUIRED ARGUMENTS:
%args%
OPTIONS:
%flags%
"#;

const USAGE_END_OF: &'static str = r#"
Print the end of a particular calendar or time unit.

This command makes it easy to "snap" datetimes to the end of a particular
period, such as a year, month, week, day, hour and so on. The end of a period
is defined to be the last nanosecond in that period.

This accepts one or more datetimes as positional arguments. When no positional
arguments are given, then line delimited datetimes are read from stdin.

USAGE:
    biff time end-of <datetime>...
    biff time end-of < line delimited <datetime>

TIP:
    use -h for short docs and --help for long docs

EXAMPLES:
    Print the last instant of the current day:

        $ biff time end-of day now
        2025-04-02T23:59:59.999999999-04:00[America/New_York]

    %snip-start%

    Print the last instant of the current week, for weeks starting with
    Sunday:

        $ biff time end-of week-sunday now
        2025-04-05T23:59:59.999999999-04:00[America/New_York]

    Or, the same, but for weeks starting with Monday:

        $ biff time end-of week-monday now
        2025-04-06T23:59:59.999999999-04:00[America/New_York]

    %snip-end%
REQUIRED ARGUMENTS:
%args%
OPTIONS:
%flags%
"#;

pub fn start(p: &mut lexopt::Parser) -> anyhow::Result<()> {
    let mut config = StartOf::default();
    let mut datetimes = positional::DateTimes::default();
    args::configure(p, USAGE_START_OF, &mut [&mut config, &mut datetimes])?;

    let of = config.of.context("missing required <start-of> argument")?;
    let mut wtr = std::io::stdout().lock();
    datetimes.try_map(|datum| {
        let dt = datum.try_map(|dt| of.start(&dt))?;
        dt.write(&mut wtr)?;
        writeln!(wtr)?;
        Ok(true)
    })?;
    Ok(())
}

pub fn end(p: &mut lexopt::Parser) -> anyhow::Result<()> {
    let mut config = EndOf::default();
    let mut datetimes = positional::DateTimes::default();
    args::configure(p, USAGE_END_OF, &mut [&mut config, &mut datetimes])?;

    let of = config.of.context("missing required <start-of> argument")?;
    let mut wtr = std::io::stdout().lock();
    datetimes.try_map(|datum| {
        let dt = datum.try_map(|dt| of.end(&dt))?;
        dt.write(&mut wtr)?;
        writeln!(wtr)?;
        Ok(true)
    })?;
    Ok(())
}

#[derive(Debug, Default)]
struct StartOf {
    of: Option<flags::Of>,
}

impl args::Configurable for EndOf {
    fn configure(
        &mut self,
        _: &mut lexopt::Parser,
        arg: &mut lexopt::Arg,
    ) -> anyhow::Result<bool> {
        match *arg {
            lexopt::Arg::Value(ref mut v) => {
                if self.of.is_some() {
                    return Ok(false);
                }
                self.of = Some(v.to_str()?.parse()?);
            }
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn usage(&self) -> &[Usage] {
        &[DateTime::ARG_OR_STDIN, flags::Of::USAGE_ARG_START]
    }
}

#[derive(Debug, Default)]
struct EndOf {
    of: Option<flags::Of>,
}

impl args::Configurable for StartOf {
    fn configure(
        &mut self,
        _: &mut lexopt::Parser,
        arg: &mut lexopt::Arg,
    ) -> anyhow::Result<bool> {
        match *arg {
            lexopt::Arg::Value(ref mut v) => {
                if self.of.is_some() {
                    return Ok(false);
                }
                self.of = Some(v.to_str()?.parse()?);
            }
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn usage(&self) -> &[Usage] {
        &[DateTime::ARG_OR_STDIN, flags::Of::USAGE_ARG_END]
    }
}
