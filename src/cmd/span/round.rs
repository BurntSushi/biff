use std::io::Write;

use crate::{
    args::{self, Usage, positional},
    round::TimeSpanRound,
    span::TimeSpan,
};

const USAGE: &'static str = r#"
Round spans to a specified smallest and largest unit. When units aren't given,
the smallest unit defaults to nanoseconds (no rounding is done) and the largest
unit defaults to the largest unit in the span.

When a span contains calendar units, then a relative datetime is required to
resolve the actual physical time duration (e.g., some months are longer than
others). By default, the relative datetime is the current time, but it may be
set via the `-r/--relative` flag.

This accepts one or more spans as positional arguments. When no positional
arguments are given, then line delimited spans are read from stdin.

USAGE:
    biff span round <span>...
    biff span round < line delimited <span>

TIP:
    use -h for short docs and --help for long docs

EXAMPLES:
    Consider a span like `2h30m10s`. One can round it to the nearest minute
    with this command:

        $ biff span round 2h30m10s -s minute
        2h 30m

    One can also change the rounding mode. This command defaults to rounding
    like how you were probably taught in school, but other modes are available:

        $ biff span round 2h30m10s -s minute -m expand
        2h 31m

    %snip-start%

    This command can be quite useful in making time spans a bit more human
    friendly. For example, getting a span since some date from now can result
    in somewhat unwieldy spans:

        $ biff span since 2025-03-01
        781h 16m 23s 579ms 682µs 329ns ago

    One can balance them to make them a bit nicer:

        $ biff span since 2025-03-01 | biff span balance
        1mo 1d 14h 17m 31s 692ms 177µs 997ns ago

    But perhaps you don't care about precision beneath minutes. You can
    re-balance the span and round it all in one go:

        $ biff span since 2025-03-01 | biff span round -s minutes -l years
        1mo 1d 14h 18m ago

    Rounding is time zone aware. For example, most days are 24 hours, and so
    rounding 11.75h to the nearest day for most days will result in a zero
    span:

        $ biff span round -s day -r '2025-03-10[America/New_York]' 11.75h
        0s

    But 2025-03-09 in New York was only 23 hours. So rounding 11.75h to
    the nearest day will actually round up:

        $ biff span round -s day -r '2025-03-09[America/New_York]' 11.75h
        1d

    %snip-end%
REQUIRED ARGUMENTS:
%args%
OPTIONS:
%flags%
"#;

pub fn run(p: &mut lexopt::Parser) -> anyhow::Result<()> {
    let mut rounder = TimeSpanRound::default();
    let mut config = Config::default();
    let mut spans = positional::Spans::default();
    args::configure(p, USAGE, &mut [&mut rounder, &mut config, &mut spans])?;

    let mut wtr = std::io::stdout().lock();
    spans.try_map(|datum| {
        let rounded = datum.try_map(|span| rounder.round(&span))?;
        rounded.write(&mut wtr)?;
        writeln!(wtr)?;
        Ok(true)
    })
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
