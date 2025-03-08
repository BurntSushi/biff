use std::io::Write;

use anyhow::Context;

use crate::{
    args::{self, Usage, flags::Op, positional},
    datetime::{DateTime, DateTimeFlexible},
    parse::OsStrExt,
    tag::MaybeTagged,
};

const USAGE: &'static str = r#"
Print only datetimes that satisfy an inequality.

This is useful for filtering datetimes according to whether they are older or
newer than a reference time.

USAGE:
    biff time cmp <op> <datetime> <datetime>...
    biff time cmp <op> <datetime> < line delimited <datetime>

TIP:
    use -h for short docs and --help for long docs

EXAMPLES:
    Print only the datetimes that are more recent than 2025-03-01:

        $ biff time cmp gt 2025-03-01 2025-02-28 2025-03-02
        2025-03-02T00:00:00-05:00[America/New_York]

    %snip-start%

    This can also be applied to tagged data. For example, to only print
    lines in a log with a datetime older than 2025-03-10 11:01 (local time):

        $ biff tag lines access.log \
            | biff time cmp lt 2025-03-10T11:01 \
            | biff untag -s

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

    let op = config.op.context("missing comparison operator")?;
    let base =
        config.base.context("missing initial datetime for comparison")?;
    let predicate = |dt: &DateTime| -> bool {
        match op {
            Op::Eq => dt == &base,
            Op::Ne => dt != &base,
            Op::Lt => dt < &base,
            Op::Gt => dt > &base,
            Op::Le => dt <= &base,
            Op::Ge => dt >= &base,
        }
    };

    let mut wtr = std::io::stdout().lock();
    datetimes.try_map(|datum| {
        match datum {
            MaybeTagged::Untagged(dt) => {
                if predicate(&dt) {
                    writeln!(wtr, "{dt}")?;
                }
            }
            MaybeTagged::Tagged(mut tagged) => {
                let original_len = tagged.tags().len();
                tagged.retain(|dt| predicate(dt));
                if (!config.all && !tagged.tags().is_empty())
                    || (config.all && original_len == tagged.tags().len())
                {
                    tagged.write(&mut wtr)?;
                    writeln!(wtr)?;
                }
            }
        }
        Ok(true)
    })
}

#[derive(Debug, Default)]
struct Config {
    op: Option<Op>,
    base: Option<DateTime>,
    all: bool,
}

impl args::Configurable for Config {
    fn configure(
        &mut self,
        _: &mut lexopt::Parser,
        arg: &mut lexopt::Arg,
    ) -> anyhow::Result<bool> {
        match *arg {
            lexopt::Arg::Long("all") => {
                self.all = true;
            }
            lexopt::Arg::Value(ref mut v) => {
                if self.base.is_some() {
                    return Ok(false);
                }
                if self.op.is_some() {
                    self.base = Some(v.parse::<DateTimeFlexible>()?.into());
                    return Ok(true);
                }
                self.op = Some(v.parse()?);
            }
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn usage(&self) -> &[Usage] {
        const ALL: Usage = Usage::flag(
            "--all",
            "Require all tags to satisfy the inequality.",
            r#"
Require all tags to satisfy the inequality.

When providing datetimes via tagged data on stdin, this command will by default
compare the reference time with each datetime tag. Any tag that doesn't satisfy
the inequality is removed. If there are no tags remaining, then that tagged
data is omitted. If there is at least one tag remaining, then that tagged data
is included.

When this flag is given, tagged data is only included in the output when *all*
of the tags satisfy the inequality.

The default behavior may be more lenient than one expects, but it is unlikely
to filter anything out that wasn't intended. That is, you might get false
positives but should never get false negatives.
"#,
        );

        &[Op::ARG, DateTime::ARG_OR_STDIN, ALL]
    }
}
