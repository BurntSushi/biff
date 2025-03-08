use std::io::Write;

use crate::{
    args::{self, Usage, flags, positional},
    datetime::{DateTime, DateTimeFlexible},
};

const USAGE: &'static str = r#"
Calculate a span until some datetime.

By default, the largest non-zero units of the span returned are hours. To get
spans with calendar units, use the `-l/--largest` flag to specify the largest
units that you want. The reason that hours are used by default is because it
makes the operation reversible. (See below for examples.)

This accepts one or more datetimes as positional arguments. When no positional
arguments are given, then line delimited datetimes are read from stdin.

This is like `biff span since`, except the order of the arguments are flipped.
Or stated differently, the span returned is the negation of what would be
returned by `biff span since`.

USAGE:
    biff span until <datetime>...
    biff span until < line delimited <datatime>

TIP:
    use -h for short docs and --help for long docs

EXAMPLES:
    Return the amount of time until a signed 32-bit integer can no longer
    represent the current time using the Unix epoch:

        $ biff time parse -f '%s' 2147483648 | biff span until
        112184h 4m 5s 885ms 385µs 852ns

    Or rounded to something more meaningful to a human:

        $ biff time parse -f '%s' 2147483648 \
            | biff span until \
            | biff span round -lyear -sday
        12y 9mo 16d

    %snip-start%

    Find the time, rounded to the nearest number of days, until June 30:

        $ biff span until 2025-06-30 | biff span round -s days
        88d

    Units up to hours are returned by default so that operations are
    reversible:

        $ biff span until -r 2024-04-30 2024-05-31
        744h
        $ biff time add 744h 2024-04-30
        2024-05-31T00:00:00-04:00[America/New_York]

    In contrast, when using calendar units, reversibility is no longer
    guaranteed:

        $ biff span until -l month -r 2024-05-31 2024-04-30
        1mo ago
        $ biff time add 1mo 2024-04-30
        2024-05-30T00:00:00-04:00[America/New_York]

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

    let largest = config.largest.get();
    let mut wtr = std::io::stdout().lock();
    datetimes.try_map(|datum| {
        let span = datum
            .try_map(|datetime| config.relative.until(largest, &datetime))?;
        span.write(&mut wtr)?;
        writeln!(wtr)?;
        Ok(true)
    })
}

#[derive(Debug)]
struct Config {
    relative: DateTime,
    largest: flags::Unit,
}

impl Default for Config {
    fn default() -> Config {
        Config { relative: DateTime::now(), largest: jiff::Unit::Hour.into() }
    }
}

impl args::Configurable for Config {
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
            lexopt::Arg::Short('l') | lexopt::Arg::Long("largest") => {
                self.largest = args::parse(p, "-l/--largest")?;
            }
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn usage(&self) -> &[Usage] {
        &[
            DateTime::ARG_OR_STDIN,
            DateTime::RELATIVE_FLAG,
            flags::Unit::LARGEST,
        ]
    }
}
