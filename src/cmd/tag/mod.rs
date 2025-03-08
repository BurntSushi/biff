mod exec;
mod files;
mod lines;
mod stat;

const USAGE: &'static str = "\
Tag arbitrary data with datetimes.

USAGE:
    biff tag <command> ...

COMMANDS:
    exec     Tag files by running arbitrary commands
    files    Tag file paths by running regexes over file contents
    lines    Extract datetimes from lines in a file
    stat     Extract datetimes from file metadata
";

pub fn run(p: &mut lexopt::Parser) -> anyhow::Result<()> {
    let cmd = crate::args::next_as_command(USAGE, p)?;
    match &*cmd {
        "exec" => exec::run(p),
        "files" => files::run(p),
        "lines" => lines::run(p),
        "stat" => stat::run(p),
        unk => anyhow::bail!("unrecognized command '{}'", unk),
    }
}
