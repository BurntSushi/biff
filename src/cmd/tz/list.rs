use std::io::Write;

use crate::{
    args::{self, Usage},
    timezone,
};

const USAGE: &'static str = r#"
List all available time zones as IANA time zone identifiers.

On Unix, this list will usually come from the time zones available in
`/usr/share/zoneinfo`. On other platforms, or Unix systems without a time zone
database, the list will come from time zones bundled with Biff itself.

Users may control where Biff looks for a time zone database via the `TZDIR`
environment variable.

The list is printed in lexicographic order.

USAGE:
    biff tz list

TIP:
    use -h for short docs and --help for long docs

REQUIRED ARGUMENTS:
%args%
OPTIONS:
%flags%
"#;

pub fn run(p: &mut lexopt::Parser) -> anyhow::Result<()> {
    let mut config = Config::default();
    args::configure(p, USAGE, &mut [&mut config])?;

    let mut wtr = std::io::stdout().lock();
    for id in timezone::available() {
        writeln!(wtr, "{id}")?;
    }
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
        &[]
    }
}
