use std::io::Write;

use crate::{
    args::{self, Usage, positional},
    round::DateTimeRound,
    span::TimeSpan,
};

const USAGE: &'static str = r#"
Round a datetime.

This accepts one or more datetimes as positional arguments. When no positional
arguments are given, then line delimited datetimes are read from stdin.

Rounding only works for units of days or lower.

USAGE:
    biff time round <datetime>...
    biff time round < line delimited <datetime>

TIP:
    use -h for short docs and --help for long docs

EXAMPLES:
    Round a datetime to the nearest day:

        $ biff time round -s day 2025-03-05T12:01
        2025-03-06T00:00:00-05:00[America/New_York]

    %snip-start%

    Round a datetime to the nearest day via truncation:

        $ biff time round -s day -m trunc 2025-03-05T12:01
        2025-03-06T00:00:00-05:00[America/New_York]

    Round a datetime to the nearest half-hour:

        $ biff time round -s minute -i 30 2025-03-05T12:15
        2025-03-05T12:30:00-05:00[America/New_York]

    Rounding takes daylight saving time into account. For example, 2025-03-09
    was only 23 hours long. So a time of 12:15 on that day will round down,
    where as it would typically round up (using the default rounding mode):

        $ biff time round -s day '2025-03-09T12:15[America/New_York]'
        2025-03-09T00:00:00-05:00[America/New_York]

    Biff also tries to do the intuitive thing when rounding within daylight
    saving time transitions. For example, Biff knows to round to the nearest
    actual instant. Notice how the offset in both of these cases remains the
    same as the datetime given (the 1am hour on 2025-11-02 in New York occurred
    twice):

        $ biff time round -sminute -i30 '2025-11-02T01:29-04[America/New_York]'
        2025-11-02T01:30:00-04:00[America/New_York]
        $ biff time round -sminute -i30 '2025-11-02T01:29-05[America/New_York]'
        2025-11-02T01:30:00-05:00[America/New_York]

    %snip-end%
REQUIRED ARGUMENTS:
%args%
OPTIONS:
%flags%
"#;

pub fn run(p: &mut lexopt::Parser) -> anyhow::Result<()> {
    let mut rounder = DateTimeRound::default();
    let mut config = Config::default();
    let mut datetimes = positional::DateTimes::default();
    args::configure(
        p,
        USAGE,
        &mut [&mut rounder, &mut config, &mut datetimes],
    )?;

    let mut wtr = std::io::stdout().lock();
    datetimes.try_map(|datum| {
        let rounded = datum.try_map(|dt| rounder.round(&dt))?;
        rounded.write(&mut wtr)?;
        writeln!(wtr)?;
        Ok(true)
    })?;
    Ok(())
}

#[derive(Debug, Default)]
struct Config {}

impl args::Configurable for Config {
    fn configure(
        &mut self,
        _: &mut lexopt::Parser,
        _: &mut lexopt::Arg,
    ) -> anyhow::Result<bool> {
        Ok(false)
    }

    fn usage(&self) -> &[Usage] {
        &[TimeSpan::ARG_OR_STDIN]
    }
}
