mod compatible;
mod list;
mod seq;

const USAGE: &'static str = "\
Commands for working with time zones.

USAGE:
    biff tz <command> ...

COMMANDS:
    compatible  List time zones compatible with an RFC 3339 timestamp
    list        List available time zones
    prev        Find one time zone transition preceding datetimes
    next        Find one time zone transition following datetimes
    seq         List time zone transitions after (or before) a datetime
";

pub fn run(p: &mut lexopt::Parser) -> anyhow::Result<()> {
    let cmd = crate::args::next_as_command(USAGE, p)?;
    match &*cmd {
        "compatible" => compatible::run(p),
        "list" => list::run(p),
        "prev" => seq::prev(p),
        "next" => seq::next(p),
        "seq" => seq::seq(p),
        unk => anyhow::bail!("unrecognized command '{}'", unk),
    }
}
