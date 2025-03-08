mod span;
mod tag;
mod time;
mod tz;
mod untag;

const USAGE: &'static str = "\
A simple utility for doing datetime arithmetic, parsing and formatting.

USAGE:
    biff <command> ...

COMMANDS:
    span   Tools for manipulating time spans/durations
    time   Tools for manipulating datetimes
    tag    Tag arbitrary data with datetimes or spans
    tz     Commands for working directly with time zones
    untag  Remove tags from previously tagged data
";

pub fn run(p: &mut lexopt::Parser) -> anyhow::Result<()> {
    // For convenience, running `biff` with no arguments prints the current
    // time in a somewhat nice format (roughly matches `date` on my system).
    if p.try_raw_args().map_or(false, |args| args.as_slice().is_empty()) {
        use jiff::fmt::{StdIoWrite, strtime};
        use std::io::Write;

        let tm = strtime::BrokenDownTime::from(&*crate::NOW);
        tm.format_with_config(
            &crate::locale::jiff_strtime_config()?,
            "%c",
            &mut StdIoWrite(std::io::stdout()),
        )?;
        writeln!(std::io::stdout())?;
        return Ok(());
    }

    let cmd = crate::args::next_as_command(USAGE, p)?;
    match &*cmd {
        "span" => span::run(p),
        "time" => time::run(p),
        "tag" => tag::run(p),
        "tz" => tz::run(p),
        "untag" => untag::run(p),
        unk => anyhow::bail!("unrecognized command '{}'", unk),
    }
}
