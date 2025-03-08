mod add;
mod cmp;
mod fmt;
mod inn;
mod of;
mod parse;
mod relative;
mod round;
mod seq;
mod sort;

const USAGE: &'static str = "\
Commands for working with datetimes.

USAGE:
    biff time <command> ...

COMMANDS:
    add       Add a span to a datetime
    cmp       Compare datetimes
    end-of    Get the end of a year, month, week, etc
    fmt       Format a datetime
    in        Convert a datetime to a time zone
    parse     Parse a datetime
    relative  Parse a relative datetime
    round     Round a datetime
    seq       Generate a sequence of datetimes
    sort      Sort datetimes
    start-of  Get the start of a year, month, week, etc
";

pub fn run(p: &mut lexopt::Parser) -> anyhow::Result<()> {
    let cmd = crate::args::next_as_command(USAGE, p)?;
    match &*cmd {
        "add" => add::run(p),
        "cmp" => cmp::run(p),
        "end-of" => of::end(p),
        "fmt" => fmt::run(p),
        "in" => inn::run(p),
        "parse" => parse::run(p),
        "relative" => relative::run(p),
        "round" => round::run(p),
        "seq" => seq::run(p),
        "sort" => sort::run(p),
        "start-of" => of::start(p),
        unk => anyhow::bail!("unrecognized command '{}'", unk),
    }
}
