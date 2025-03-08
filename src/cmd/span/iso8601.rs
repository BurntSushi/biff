use std::io::Write;

use jiff::fmt::temporal;

use crate::{
    args::{self, Usage, positional},
    span::TimeSpan,
};

const USAGE: &'static str = r#"
Format spans into the ISO 8601 duration format.

This will permit weeks to be combined with other units, which is not permitted
in the ISO 8601 duration format. However, it is permitted in Temporal's
extension to the ISO 8601 duration format. If you need strict ISO 8601
compatibility, you should use `biff span balance` or `biff span round` to
remove weeks from your span before formatting them as ISO 8601 durations.

To format a span into a more friendly and "human" readable format, use
`biff span fmt` instead.

USAGE:
    biff span iso8601 <span>...
    biff span iso8601 < line delimited <span>

TIP:
    use -h for short docs and --help for long docs

EXAMPLES:
    To print a span in the ISO 8601 duration format:

        $ biff span iso8601 75y5mo22d5h30m12s
        P75Y5M22DT5H30M12S

    %snip-start%

    One can make the unit designators lowercase, which can improve
    readability at the expense of portability:

        $ biff span iso8601 -l 75y5mo22d5h30m12s
        P75y5m22dT5h30m12s

    ISO 8601 durations do not have units smaller than seconds. Therefore,
    milliseconds, microseconds and nanoseconds are all represented as
    fractional seconds:

        $ biff span iso8601 123ms456us789ns
        PT0.123456789S

    This does mean that, unlike the friendly format, not all spans will
    roundtrip without some loss in how the span is expressed. This example
    loses the fact that `2 seconds` is expressed in units of milliseconds:

        $ biff span iso8601 2000ms
        PT2S

    %snip-end%
REQUIRED ARGUMENTS:
%args%
OPTIONS:
%flags%
"#;

pub fn run(p: &mut lexopt::Parser) -> anyhow::Result<()> {
    let mut config = Config::default();
    let mut spans = positional::Spans::default();
    args::configure(p, USAGE, &mut [&mut config, &mut spans])?;

    let printer = config.printer();
    let mut wtr = std::io::stdout().lock();
    spans.try_map(|datum| {
        let formatted =
            datum.try_map(|span| Ok(printer.span_to_string(span.get())))?;
        formatted.write(&mut wtr)?;
        writeln!(wtr)?;
        Ok(true)
    })
}

#[derive(Debug, Default)]
struct Config {
    lowercase: bool,
}

impl Config {
    fn printer(&self) -> temporal::SpanPrinter {
        temporal::SpanPrinter::new().lowercase(self.lowercase)
    }
}

impl args::Configurable for Config {
    fn configure(
        &mut self,
        _: &mut lexopt::Parser,
        arg: &mut lexopt::Arg,
    ) -> anyhow::Result<bool> {
        match *arg {
            lexopt::Arg::Short('l') | lexopt::Arg::Long("lowercase") => {
                self.lowercase = true;
            }
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn usage(&self) -> &[Usage] {
        const LOWERCASE: Usage = Usage::flag(
            "-l/--lowercase",
            "Use lowercase unit designators instead of uppercase.",
            r#"
Use lowercase unit designators instead of uppercase.

This is a non-standard extension to the ISO 8601 duration format and doesn't
enjoy broad support. However, it can be a little easier to read and is
supported by the Tempora ISO 8601 duration format.
"#,
        );

        &[TimeSpan::ARG_OR_STDIN, LOWERCASE]
    }
}
