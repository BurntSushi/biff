use std::io::Write;

use anyhow::Context;

use crate::{
    args::{self, Usage},
    datetime::DateTime,
    parse::OsStrExt,
    timezone,
};

const USAGE: &'static str = r#"
List all compatible time zones for the given datetime.

That is, for a given instant and every available time zone, if the instant
converted to that time zone results in the same offset for the datetime given,
then that time zone is considered compatible with the given instant.

If the datetime given already has an IANA time zone identifier (i.e., it is an
RFC 9557 timestamp with an IANA time zone identifier annotation), then only
that time zone is returned. Additionally, if the datetime indicates that the
offset from UTC is explicitly unknown (e.g., the `Z` or `-00:00` offsets), then
the special `Etc/Unknown` identifier is returned.

The list is printed in lexicographic order.

USAGE:
    biff tz compatible <datetime>

TIP:
    use -h for short docs and --help for long docs

EXAMPLES:
    This command makes it easy to print the time zones compatible with a
    particular instant:

        $ biff tz compatible '2025-03-09T17:00+10:30'
        Australia/Adelaide
        Australia/Broken_Hill
        Australia/South
        Australia/Yancowinna

REQUIRED ARGUMENTS:
%args%
OPTIONS:
%flags%
"#;

pub fn run(p: &mut lexopt::Parser) -> anyhow::Result<()> {
    let mut config = Config::default();
    args::configure(p, USAGE, &mut [&mut config])?;

    let mut wtr = std::io::stdout().lock();
    let dt = config.timestamp.with_context(|| {
        format!("missing datetime to list compatible time zones for")
    })?;
    let zdt = dt.get();
    let tz = zdt.time_zone();
    if tz.is_unknown() {
        writeln!(wtr, "Etc/Unknown")?;
        return Ok(());
    }
    if let Some(iana) = tz.iana_name() {
        if tz.to_fixed_offset().is_err() {
            writeln!(wtr, "{iana}")?;
            return Ok(());
        }
    }
    for id in timezone::available() {
        let candidate = jiff::tz::TimeZone::get(id)?;
        let offset = candidate.to_offset(zdt.timestamp());
        if offset != zdt.offset() {
            continue;
        }
        writeln!(wtr, "{id}")?;
    }
    Ok(())
}

#[derive(Debug, Default)]
struct Config {
    timestamp: Option<DateTime>,
}

impl args::Configurable for Config {
    fn configure(
        &mut self,
        _: &mut lexopt::Parser,
        arg: &mut lexopt::Arg,
    ) -> anyhow::Result<bool> {
        match *arg {
            lexopt::Arg::Value(ref mut v) => {
                if self.timestamp.is_some() {
                    return Ok(false);
                }
                self.timestamp = Some(v.parse()?);
            }
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn usage(&self) -> &[Usage] {
        &[DateTime::ARG]
    }
}
