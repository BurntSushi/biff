use std::io::Write;

use crate::{
    args::{self, Usage, positional},
    round::TimeSpanBalance,
    span::TimeSpan,
};

const USAGE: &'static str = r#"
Balance spans to a specified largest unit. When a unit isn't given, it defaults
to `years`, which is the largest unit supported by Biff.

Balancing a span refers to either collapsing bigger units down into smaller
units, or allowing overflowing units to spill over into bigger units. The
former occurs when the unit given is smaller than the largest non-zero unit
in the span given. The latter occurs when the unit given is bigger than the
largest non-zero unit in the span given.

When a span contains calendar units, then a relative datetime is required to
resolve the actual physical time duration (e.g., some months are longer than
others). By default, the relative datetime is the current time, but it may be
set via the `-r/--relative` flag.

The functionality of this command is fully subsumed by `biff span round`. The
difference is that this command never does any rounding and uses `years` as
a default for the largest unit.

This accepts one or more spans as positional arguments. When no positional
arguments are given, then line delimited spans are read from stdin.

USAGE:
    biff span balance <span>...
    biff span balance < line delimited <span>

TIP:
    use -h for short docs and --help for long docs

EXAMPLES:
    Consider a span like `2h30m10s`. There are no overflowing units.
    But users might want to convert it to a span with units no bigger than
    seconds:

        $ biff span balance 2h30m10s -l seconds
        9010s

    %snip-start%

    Note though that if the span contains units less than seconds, than those
    are still preserved:

        $ biff span balance 2h30m10.123s -l seconds
        9010s 123ms

    (Rounding lower units to larger units can be done with `biff span round`.)

    Overflowing units can also be balanced up into bigger units:

        $ biff span balance 366d
        1y 1d

    And specifically for calendar units, the length of each unit can vary based
    on the date:

        $ biff span balance 366d -r 2024-01-15
        1y

    Or even the time zone:

        $ biff span balance 1d -l hour -r '2025-03-09T00-05[America/New_York]'
        23h
        $ biff span balance 1d -l hour -r '2025-03-09T00+00[Europe/London]'
        24h

    While `biff span since` will return spans with hours as the largest unit,
    one can avoid calling `biff span balance` by using the `-l/--largest` flag
    on `biff span since`. This has the advantage of computing the span using
    the same relative datetime as given to `biff span since`:

        $ biff span since -l month -r 2025-02-28 2025-03-31
        1mo

    Where as piping into `biff span balance` without specifying the same
    relative datetime will give a possibly undesirable result:

        $ biff span since -r 2025-02-28 2025-03-31 | biff span balance
        30d 23h

    %snip-end%
REQUIRED ARGUMENTS:
%args%
OPTIONS:
%flags%
"#;

pub fn run(p: &mut lexopt::Parser) -> anyhow::Result<()> {
    let mut balancer = TimeSpanBalance::default();
    let mut config = Config::default();
    let mut spans = positional::Spans::default();
    args::configure(p, USAGE, &mut [&mut balancer, &mut config, &mut spans])?;

    let mut wtr = std::io::stdout().lock();
    spans.try_map(|datum| {
        let balanced = datum.try_map(|span| balancer.balance(&span))?;
        balanced.write(&mut wtr)?;
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
