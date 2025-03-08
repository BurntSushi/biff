mod balance;
mod fmt;
mod iso8601;
mod round;
mod since;
mod until;

const USAGE: &'static str = r#"
Commands for working with calendar and time durations.

USAGE:
    biff span <command> ...

COMMANDS:
    balance  Change the largest non-zero unit in a span
    fmt      Format a span as a "friendly" duration
    iso8601  Format span as an ISO 8601 duration
    round    Round a span
    since    Calculate a span since a datetime
    until    Calculate a span until a datetime
"#;

pub fn run(p: &mut lexopt::Parser) -> anyhow::Result<()> {
    let cmd = crate::args::next_as_command(USAGE, p)?;
    match &*cmd {
        "balance" => balance::run(p),
        "fmt" => fmt::run(p),
        "iso8601" => iso8601::run(p),
        "round" => round::run(p),
        "since" => since::run(p),
        "until" => until::run(p),
        unk => anyhow::bail!("unrecognized command '{}'", unk),
    }
}
