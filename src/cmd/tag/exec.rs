use std::{
    borrow::Cow, ffi::OsString, io::Write, process::Command, sync::Arc,
};

use {
    anyhow::Context,
    bstr::{BStr, BString, ByteSlice, ByteVec},
};

use crate::{
    args::{self, Usage, flags},
    parallel::Parallel,
    parse::{BufReadExt, LineBuf},
    tag::{Tag, Tagged},
};

const USAGE: &'static str = r#"
Tag file paths by running arbitrary commands.

This accepts a command name and zero or more arguments to pass to that command
for each file path on stdin. The command is run for every file path. Any `{}`
found in an argument is replaced with the file path. If no argument contains
`{}`, then the file path is added as the final argument to the command.

USAGE:
    biff tag exec <command> [<arg>]... < line delimited <path>

TIP:
    use -h for short docs and --help for long docs

EXAMPLES:
    Tag each file in a git repository with its last commit datetime:

        git ls-files | biff tag exec git log -n1 --format='%cI'

POSITIONAL ARGUMENTS:
%args%
OPTIONS:
%flags%
"#;

pub fn run(p: &mut lexopt::Parser) -> anyhow::Result<()> {
    let mut config = Config::default();
    args::configure(p, USAGE, &mut [&mut config])?;

    let command_parts = config.command_parts()?;
    let mut wtr = std::io::stdout();
    let mut parallel = Parallel::new(
        config.threads.get(),
        move |line: LineBuf| {
            let mut cmd = command_parts.command(line.content())?;
            let output = cmd
                .output()
                .with_context(|| format!("failed to run {cmd:?}"))?;
            anyhow::ensure!(
                output.status.success(),
                "got exit status {code:?} when running {cmd:?}, \
                 stderr: {stderr}",
                code = output.status.code(),
                stderr = output.stderr.as_bstr(),
            );

            let mut tagged = Tagged::new(line.full());
            for (i, output_line) in output.stdout.lines().enumerate() {
                let number = i + 1;
                let tag = output_line.as_bstr();
                let tag = tag.to_str().with_context(|| {
                    format!(
                        "on line {number} from command {cmd:?}, \
                         tag {tag:?} is not valid UTF-8",
                    )
                })?;
                tagged = tagged.tag(Tag::new(tag.to_string()));
            }
            Ok(tagged.into_owned())
        },
        move |tagged| {
            tagged?.write(&mut wtr)?;
            writeln!(wtr)?;
            Ok(true)
        },
    );

    let result1 = std::io::stdin()
        .lock()
        .for_byte_line(|line| parallel.send(line.to_owned()));
    let result2 = parallel.wait();
    result1?;
    result2
}

/// The parts given that make up a command.
#[derive(Clone, Debug, Default)]
struct CommandParts {
    parts: Arc<[BString]>,
}

impl CommandParts {
    /// Create a new sequence of parts that make up a command.
    ///
    /// The parts given may contain a `{}` somewhere, which will get replaced
    /// with a file path when run.
    ///
    /// If `parts` is empty, then this returns an error.
    fn new(parts: Vec<BString>) -> anyhow::Result<CommandParts> {
        anyhow::ensure!(
            !parts.is_empty(),
            "command requires at least a program name",
        );
        Ok(CommandParts { parts: parts.into() })
    }

    /// Creates a `std::process::Command` from these parts using the file
    /// path given for interpolation.
    ///
    /// Basically, all instances of `{}` outside of the program name (that
    /// aren't escaped) are replaced with the given file path. If there are
    /// no such instances, then the path is added on as the final part to the
    /// command.
    ///
    /// This generally shouldn't fail, but in theory could, if any of the
    /// parts in the command are not valid UTF-8 on non-Unix environments.
    /// (If that did happen, then CLI parsing should have failed.)
    fn command(&self, path: &BStr) -> anyhow::Result<Command> {
        /// Replaces `{}` in `arg` with `path`.
        ///
        /// Returns `None` when no replacements are made. When `Some`
        /// is returned, it is guaranteed that at least one replacement
        /// was made.
        fn replace(arg: &BStr, path: &BStr) -> Option<BString> {
            enum State {
                Default,
                Backslash,
                OpeningBrace,
            }

            // If there's no `{`, then we definitely have no replacement,
            // so just bail early. We might still not have a replacement,
            // e.g., if it's just `{` by itself or `{` is escaped. But this
            // should be very rare.
            if arg.find_byte(b'{').is_none() {
                return None;
            }
            let mut state = State::Default;
            let mut new = vec![];
            for byte in arg.bytes() {
                state = match (state, byte) {
                    (State::Default, b'\\') => State::Backslash,
                    (State::Default, b'{') => State::OpeningBrace,
                    (State::Default, _) => {
                        new.push(byte);
                        State::Default
                    }
                    (State::Backslash, _) => {
                        new.push(byte);
                        State::Default
                    }
                    (State::OpeningBrace, b'}') => {
                        new.extend_from_slice(path);
                        State::Default
                    }
                    (State::OpeningBrace, _) => {
                        new.push(b'{');
                        new.push(byte);
                        State::Default
                    }
                };
            }
            // If we ended up in an invalid state, then we do nothing. I kinda
            // feel like we should probably report an error here, but it's not
            // clear if it's worth doing.
            match state {
                State::Default => {}
                State::Backslash | State::OpeningBrace => return None,
            }
            // It's possible we didn't ultimately do a replacement.
            // In which case, we don't want to indicate that we did.
            if new == arg { None } else { Some(BString::from(new)) }
        }

        let program = &self.parts[0];
        let program = program.to_os_str().with_context(|| {
            format!("program binary path {program:?} is not valid UTF-8")
        })?;
        let mut cmd = Command::new(program);

        let mut did_replacement = false;
        for part in self.parts.iter().skip(1) {
            let part = match replace(part.as_bstr(), path) {
                None => Cow::Borrowed(part),
                Some(part) => {
                    did_replacement = true;
                    Cow::Owned(part)
                }
            };
            let part = part.to_os_str().with_context(|| {
                format!("argument to command {part:?} is not valid UTF-8")
            })?;
            cmd.arg(part);
        }
        if !did_replacement {
            let path = path.to_os_str().with_context(|| {
                format!("path {path:?} given to command is not valid UTF-8")
            })?;
            cmd.arg(path);
        }
        Ok(cmd)
    }
}

#[derive(Debug, Default)]
struct Config {
    command_parts: Vec<BString>,
    threads: flags::Threads,
}

impl Config {
    fn command_parts(&self) -> anyhow::Result<CommandParts> {
        CommandParts::new(self.command_parts.clone())
    }

    fn add_command_part(&mut self, os_str: OsString) -> anyhow::Result<()> {
        let part = Vec::from_os_string(os_str).map_err(|err| {
            anyhow::anyhow!(
                "command program name and arguments \
                 must be valid UTF-8 in non-Unix \
                 environments, but `{err:?}` is not \
                 valid UTF-8",
            )
        })?;
        self.command_parts.push(BString::from(part));
        Ok(())
    }
}

impl args::Configurable for Config {
    fn configure(
        &mut self,
        p: &mut lexopt::Parser,
        arg: &mut lexopt::Arg,
    ) -> anyhow::Result<bool> {
        match *arg {
            lexopt::Arg::Short('j') | lexopt::Arg::Long("threads") => {
                self.threads = args::parse(p, "-j/--threads")?;
            }
            lexopt::Arg::Value(ref mut v) => {
                self.add_command_part(std::mem::take(v))?;
                // As soon as we see a positional argument, the
                // only reasonable thing left for us to do is consume
                // all remaining arguments as part of the command
                // we want to run.
                //
                // NOTE: I don't think `raw_args()` can ever fail
                // here, since we know we just parsed a positional
                // argument.
                for v in p.raw_args()? {
                    self.add_command_part(v)?;
                }
            }
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn usage(&self) -> &[Usage] {
        const COMMAND: Usage = Usage::arg(
            "<command>",
            "The path to a command to run.",
            r#"
The path to a command to run.

This maybe an absolute path, a relative path or just the name of command that
can be found in your `PATH` environment variable.
"#,
        );

        const ARG: Usage = Usage::arg(
            "<arg>",
            "An argument to pass to <command>.",
            r#"
An argument to pass to <command>.

If an argument contains `{}`, then it is substituted for a file path,
regardless of where it appears. To write a literal `{` or `}`, use `\{` or
`\}`, respectively. When no arguments contain a `{}`, then a file path is added
as an additional argument.
"#,
        );

        const PATH: Usage = Usage::arg(
            "<path>",
            "A file path to interpolate into <command>.",
            r#"
A file path to interpolate into <command>.

File paths must be passed on stdin in a line delimited format.

If an <arg> contains `{}`, then it is replaced with the file path. Otherwise,
the file path is added to the end of the command invocation.
"#,
        );

        &[COMMAND, ARG, PATH, flags::Threads::USAGE]
    }
}
