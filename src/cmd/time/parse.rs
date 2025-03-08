use std::io::Write;

use crate::{
    args::{self, Usage, flags, positional},
    datetime::{DateTime, DateTimeFlexible},
};

const USAGE: &'static str = r#"
Parse a datetime in a particular format.

This accepts one or more strings to parse as positional arguments. When no
positional arguments are given, then line delimited strings are read from
stdin.

When `-f/--format` is used with an `strftime`-like format string and no
offset or time zone is parsed, then the parsed time is assumed to be local
relative to your system time zone. (Which can be overridden via the `TZ`
environment variable.)

By default, this only parses RFC 9557 timestamps. For example,
`2025-05-01T17:30-04[America/New_York]`. To accept a more flexible format
used by Biff for datetimes passed on the CLI, including relative datetimes,
use `-f flexible`.

USAGE:
    biff time parse <string>...
    biff time parse < line delimited <string>

TIP:
    use -h for short docs and --help for long docs

EXAMPLES:
    Parse an "American" date with the month first and two digits for the year:

        $ biff time parse -f '%m/%d/%y' 03/15/25

    %snip-start%

    Parse a datetime with an IANA time zone identifier (not commonly supported
    among other `strptime` implementations):

        $ biff time parse -f '%Y-%m-%d %Q' '2025-03-15 Australia/Tasmania'

    Parse a Unix timestamp, in seconds:

        $ biff time parse -f '%s' 999999999

    Parse an ISO 8601 week date in your local time:

        $ biff time parse -f '%G-W%V-%u' 2025-W12-1

    Parse a relative datetime from stdin:

        $ echo '1 hour ago' | biff time parse -f flexible

    %snip-end%
REQUIRED ARGUMENTS:
%args%
OPTIONS:
%flags%
"#;

pub fn run(p: &mut lexopt::Parser) -> anyhow::Result<()> {
    let mut config = Config::default();
    let mut args = positional::MaybeTaggedArguments::default();
    args::configure(p, USAGE, &mut [&mut config, &mut args])?;

    let mut wtr = std::io::stdout().lock();
    args.try_map(|datum| {
        let parsed = match datum
            .try_map(|arg| config.format.parse(&config.relative, &arg))
        {
            Ok(parsed) => parsed,
            Err(err) => {
                if !config.ignore_invalid {
                    return Err(err);
                }
                log::warn!("{err}");
                return Ok(true);
            }
        };

        parsed.write(&mut wtr)?;
        writeln!(wtr)?;
        Ok(true)
    })?;
    Ok(())
}

#[derive(Debug, Default)]
struct Config {
    format: flags::Format,
    ignore_invalid: bool,
    relative: DateTime,
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
            lexopt::Arg::Short('i') | lexopt::Arg::Long("ignore-invalid") => {
                self.ignore_invalid = true;
            }
            lexopt::Arg::Short('r') | lexopt::Arg::Long("relative") => {
                let relative: DateTimeFlexible =
                    args::parse(p, "-r/--relative")?;
                self.relative = relative.into();
            }
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn usage(&self) -> &[Usage] {
        const IGNORE_INVALID: Usage = Usage::flag(
            "-i/--ignore-invalid",
            "Ignore strings that don't parse in the requested format.",
            r#"
Ignore strings that don't parse in the requested format.

When enabled, these strings are dropped and parsing continues to the next
input. To see error messages, enable logging with `BIFF_LOG=warn`. When
disabled, if parsing fails, then execution stops and an error is printed.
"#,
        );
        &[
            DateTime::ARG_OR_STDIN,
            flags::Format::USAGE_PARSE,
            IGNORE_INVALID,
            DateTime::RELATIVE_FLAG,
        ]
    }
}
