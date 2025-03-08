use std::io::Write;

use crate::{
    args::{self, Usage, positional},
    datetime::DateTime,
};

const USAGE: &'static str = r#"
Sort datetimes in ascending (the default) or descending order.

This accepts one or more datetimes as positional arguments. When no positional
arguments are given, then line delimited datetimes are read from stdin. When
reading from stdin, tagged data is also accepted. In the case of tagged data
with multiple datetime tags, sorting is done lexicographically.

USAGE:
    biff time sort <datetime>...
    biff time sort < line delimited <datetime>

TIP:
    use -h for short docs and --help for long docs

EXAMPLES:
    Sort files checked into git via their last commit date and format the
    display such that the datetime is included with the file path:

        git ls-files \
            | biff tag exec git log -n1 --format='%cI' \
            | biff time sort \
            | biff untag -f '{tag} {data}'

REQUIRED ARGUMENTS:
%args%
OPTIONS:
%flags%
"#;

pub fn run(p: &mut lexopt::Parser) -> anyhow::Result<()> {
    let mut config = Config::default();
    let mut datetimes = positional::DateTimes::default();
    args::configure(p, USAGE, &mut [&mut config, &mut datetimes])?;

    let mut dts = vec![];
    datetimes.try_map(|dt| {
        dts.push(dt);
        Ok(true)
    })?;

    if config.reverse {
        dts.sort_by(|dt1, dt2| dt1.cmp(dt2).reverse());
    } else {
        dts.sort();
    }

    let mut wtr = std::io::stdout().lock();
    for dt in dts {
        dt.write(&mut wtr)?;
        writeln!(wtr)?;
    }
    Ok(())
}

#[derive(Debug, Default)]
struct Config {
    reverse: bool,
}

impl args::Configurable for Config {
    fn configure(
        &mut self,
        _: &mut lexopt::Parser,
        arg: &mut lexopt::Arg,
    ) -> anyhow::Result<bool> {
        match *arg {
            lexopt::Arg::Short('r') | lexopt::Arg::Long("reverse") => {
                self.reverse = true;
            }
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn usage(&self) -> &[Usage] {
        const REVERSE: Usage = Usage::flag(
            "-r/--reverse",
            "Sort datetimes in descending (newer to older) order.",
            r#"
Sort datetimes in descending (newer to older) order.

By default, datetimes are sorted in ascending (older to newer) order.
"#,
        );

        &[DateTime::ARG_OR_STDIN, REVERSE]
    }
}
