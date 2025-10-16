use std::{
    fmt::{Debug, Display, Write},
    str::FromStr,
    sync::LazyLock,
};

use {
    anyhow::Context,
    bstr::{BString, ByteVec},
    lexopt::{Arg, Parser, ValueExt},
    regex::Regex,
};

use crate::parse::{BytesExt, FromBytes};

pub mod flags;
pub mod positional;

pub trait Configurable: Debug {
    fn configure(
        &mut self,
        p: &mut Parser,
        arg: &mut Arg,
    ) -> anyhow::Result<bool>;

    /// A list of `Usage` documentation for the flags/arguments that this
    /// implementation parses.
    ///
    /// This is optional because some implementations of this trait are
    /// pretty generic, and so callers should provide more concrete docs.
    fn usage(&self) -> &[Usage] {
        &[]
    }
}

pub fn configure(
    p: &mut Parser,
    usage: &str,
    targets: &mut [&mut dyn Configurable],
) -> anyhow::Result<()> {
    /// Removes `%snip-start%`, `%snip-end%` and everything between them.
    ///
    /// This is used to strip extraneous content from a command's usage
    /// docs when rendering the "short" docs.
    static REMOVE_SNIPS: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?m)^\s*%snip-start%\p{any}*?%snip-end%\s*$").unwrap()
    });

    /// Removes `%snip-start%` and `%snip-end%` markers only, including any
    /// following whitespace.
    ///
    /// This is used to strip the markers from the visible content of a
    /// command's usage docs when rendering the "long" docs.
    static REMOVE_SNIP_MARKERS: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?m)^\s*%snip-(start|end)%\s*$").unwrap()
    });

    loop {
        // Try to pluck out `-[0-9]blah` to handle things like `-1h` specially.
        let arg = if let Some(arg) = parse_dash_number(p) {
            arg
        } else if let Some(arg) = p.next()? {
            arg
        } else {
            break;
        };

        match arg {
            Arg::Short('h') | Arg::Long("help") => {
                let args = collect_usage_for_args(targets);
                let flags = collect_usage_for_flags(targets);
                let (usage, args, flags) = if arg == Arg::Short('h') {
                    let usage =
                        REMOVE_SNIPS.replace_all(usage, "").into_owned();
                    (usage, Usage::short(&args), Usage::short(&flags))
                } else {
                    let usage = REMOVE_SNIP_MARKERS
                        .replace_all(usage, "")
                        .into_owned();
                    (usage, Usage::long(&args), Usage::long(&flags))
                };
                let usage =
                    usage.replace("%args%", &args).replace("%flags%", &flags);
                return Err(anyhow::Error::from(Help(
                    usage.trim().to_string(),
                )));
            }
            _ => {}
        }
        // We do this little dance to disentangle the lifetime of 'p' from the
        // lifetime on 'arg'. The cost is that we have to clone all long flag
        // names to give it a place to live that isn't tied to 'p'. Annoying,
        // but not the end of the world.
        let long_flag: Option<String> = match arg {
            Arg::Long(name) => Some(name.to_string()),
            _ => None,
        };
        let mut arg = match long_flag {
            Some(ref flag) => Arg::Long(flag),
            None => match arg {
                Arg::Short(c) => Arg::Short(c),
                Arg::Long(_) => unreachable!(),
                Arg::Value(value) => Arg::Value(value),
            },
        };
        // OK, now ask all of our targets whether they want this argument.
        let mut recognized = false;
        for t in targets.iter_mut() {
            if t.configure(p, &mut arg)? {
                recognized = true;
                break;
            }
        }
        if !recognized {
            return Err(arg.unexpected().into());
        }
    }
    Ok(())
}

fn collect_usage_for_args<'a>(
    targets: &[&mut dyn Configurable],
) -> Vec<Usage> {
    let mut usages = vec![];
    for t in targets.iter() {
        usages.extend(t.usage().iter().copied().filter(|u| !u.flag));
    }
    // We specifically don't sort arguments since we might want to present
    // them in a specific order, e.g., the order of positional arguments.
    usages
}

fn collect_usage_for_flags<'a>(
    targets: &[&mut dyn Configurable],
) -> Vec<Usage> {
    // Include `-h/--help` and `--version` everywhere.
    let mut usages = vec![Help::USAGE, Version::USAGE];
    for t in targets.iter() {
        usages.extend(t.usage().iter().copied().filter(|u| u.flag));
    }
    usages.sort_by_key(|u| {
        u.format.split_once(", ").map(|(_, long)| long).unwrap_or(u.format)
    });
    usages
}

/// Attempts to parse a `-[0-9]{remaining}` and convert it to a positional
/// argument.
///
/// We make `-[0-9]` always be interpreted as a positional argument, because,
/// for `biff`, things like `-1h` are very common. And forcing users to prefix
/// that with `--` to indicate the start of unambiguously positional arguments
/// is super annoying. In exchange, we can't have a short flag corresponding to
/// an ASCII digit.
///
/// We also allow `-P1D` for ISO 8601 durations, for example, as well. This
/// means `P` can't be used as a short flag either.
///
/// Ref: https://docs.rs/lexopt/latest/lexopt/struct.Parser.html#method.try_raw_args
fn parse_dash_number(parser: &mut Parser) -> Option<Arg<'_>> {
    parser
        .try_raw_args()?
        .next_if(|arg| {
            let value = arg.as_encoded_bytes();
            value.len() >= 2
                && value[0] == b'-'
                && (value[1].is_ascii_digit() || value[1] == b'P')
        })
        .map(Arg::Value)
}

/// Parses the argument from the given parser as a command name, and returns
/// it. If the next arg isn't a simple value then this returns an error.
///
/// This also handles the case where -h/--help is given, in which case, the
/// given usage information is converted into an error and printed. Similarly
/// for `--version`.
pub fn next_as_command(usage: &str, p: &mut Parser) -> anyhow::Result<String> {
    let usage = usage.trim();
    let arg = match p.next()? {
        Some(arg) => arg,
        None => anyhow::bail!("{}", usage),
    };
    let cmd = match arg {
        Arg::Value(cmd) => cmd.string()?,
        Arg::Short('h') | Arg::Long("help") => {
            anyhow::bail!("{}", Help(usage.to_string()))
        }
        Arg::Long("version") => return Err(anyhow::Error::from(Version)),
        arg => return Err(arg.unexpected().into()),
    };
    Ok(cmd)
}

/// Parses the next 'p.value()' into 'T'. Any error messages will include the
/// given flag name in them.
pub fn parse<T>(p: &mut Parser, flag_name: &'static str) -> anyhow::Result<T>
where
    T: FromStr,
    <T as FromStr>::Err: Display + Debug + Send + Sync + 'static,
{
    // This is written somewhat awkwardly and the type signature is also pretty
    // funky primarily because of the following two things: 1) the 'FromStr'
    // impls in this crate just use 'anyhow::Error' for their error type and 2)
    // 'anyhow::Error' does not impl 'std::error::Error'.
    let osv = p.value().context(flag_name)?;
    let strv = match osv.to_str() {
        Some(strv) => strv,
        None => {
            let err = lexopt::Error::NonUnicodeValue(osv.into());
            return Err(anyhow::Error::from(err).context(flag_name));
        }
    };
    let parsed = match strv.parse() {
        Err(err) => return Err(anyhow::Error::msg(err).context(flag_name)),
        Ok(parsed) => parsed,
    };
    Ok(parsed)
}

/// Parses the next 'p.value()' into 'T'. Any error messages will include the
/// given flag name in them.
pub fn parse_bytes<T>(
    p: &mut Parser,
    flag_name: &'static str,
) -> anyhow::Result<T>
where
    T: FromBytes,
    <T as FromBytes>::Err: Display + Debug + Send + Sync + 'static,
{
    // This is written somewhat awkwardly and the type signature is also pretty
    // funky primarily because of the following two things: 1) the 'FromBytes'
    // impls in this crate just use 'anyhow::Error' for their error type and 2)
    // 'anyhow::Error' does not impl 'std::error::Error'.
    let osv = p.value().context(flag_name)?;
    let bytes = match Vec::from_os_string(osv) {
        Ok(bytes) => BString::from(bytes),
        Err(err) => {
            let err = lexopt::Error::NonUnicodeValue(err);
            return Err(anyhow::Error::from(err).context(flag_name));
        }
    };
    let parsed = match bytes.parse() {
        Err(err) => return Err(anyhow::Error::msg(err).context(flag_name)),
        Ok(parsed) => parsed,
    };
    Ok(parsed)
}

/// A type for expressing the documentation of a flag.
///
/// The `Usage::short` and `Usage::long` functions take a slice of usages and
/// format them into a human readable display. It does simple word wrapping and
/// column alignment for you.
#[derive(Clone, Copy, Debug)]
pub struct Usage {
    /// Whether this is docs for a flag (optional) or an argument (required).
    pub flag: bool,
    /// The format of the flag, for example, `-k, --match-kind <kind>`.
    pub format: &'static str,
    /// A very short description of the flag. Should fit on one line along with
    /// the format.
    pub short: &'static str,
    /// A longer form description of the flag. May be multiple paragraphs long
    /// (but doesn't have to be).
    pub long: &'static str,
}

impl Usage {
    /// Create a new usage for an optional flag from the given components.
    pub const fn flag(
        format: &'static str,
        short: &'static str,
        long: &'static str,
    ) -> Usage {
        Usage { flag: true, format, short, long }
    }

    /// Create a new usage for an required argument from the given components.
    pub const fn arg(
        format: &'static str,
        short: &'static str,
        long: &'static str,
    ) -> Usage {
        Usage { flag: false, format, short, long }
    }

    /// Format a two column table from the given usages, where the first
    /// column is the format and the second column is the short description.
    pub fn short(usages: &[Usage]) -> String {
        const MIN_SPACE: usize = 2;

        let mut result = String::new();
        let max_len = match usages.iter().map(|u| u.format.len()).max() {
            None => return result,
            Some(len) => len,
        };
        for usage in usages.iter() {
            let padlen = MIN_SPACE + (max_len - usage.format.len());
            let padding = " ".repeat(padlen);
            writeln!(result, "    {}{}{}", usage.format, padding, usage.short)
                .unwrap();
        }
        result
    }

    /// Print the format of each usage and its long description below the
    /// format. This also does appropriate indentation with the assumption that
    /// it is in an OPTIONS section of a bigger usage message.
    pub fn long(usages: &[Usage]) -> String {
        let wrap_opts = textwrap::Options::new(79)
            .initial_indent("        ")
            .subsequent_indent("        ");
        let mut result = String::new();
        for (i, usage) in usages.iter().enumerate() {
            if i > 0 {
                writeln!(result, "").unwrap();
            }
            writeln!(result, "    {}", usage.format).unwrap();
            for (i, paragraph) in usage.long.trim().split("\n\n").enumerate() {
                if i > 0 {
                    result.push('\n');
                }
                let flattened = paragraph.replace("\n", " ");
                for line in textwrap::wrap(&flattened, &wrap_opts) {
                    result.push_str(&line);
                    result.push('\n');
                }
            }
        }
        result
    }
}

/// An error type indicating that the error is a `-h/--help` message.
///
/// In other words, it should be printed to stdout with a success exit code.
///
/// We sniff this out in `main` via downcasting an `anyhow::Error`.
#[derive(Debug)]
pub struct Help(String);

impl Help {
    const USAGE: Usage = Usage::flag(
        "-h/--help",
        "This flag prints the help output for Biff.",
        r#"
This flag prints the help output for Biff.

Unlike most other flags, the behavior of the short flag, -h, and the long flag,
--help, is different. The short flag will show a condensed help output while
the long flag will show a verbose help output. The verbose help output has
complete documentation, where as the condensed help output will show only a
single line for every flag.
"#,
    );
}

impl std::fmt::Display for Help {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl std::error::Error for Help {}

/// An error type indicating that the error is a `--version` message.
///
/// In other words, it should be printed to stdout with a success exit code.
///
/// We sniff this out in `main` via downcasting an `anyhow::Error`.
#[derive(Debug)]
pub struct Version;

impl Version {
    const USAGE: Usage = Usage::flag(
        "--version",
        "This flag prints the version of Biff.",
        r#"
This flag prints the version of Biff.
"#,
    );
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let semver = option_env!("CARGO_PKG_VERSION").unwrap_or("N/A");
        let version = match option_env!("BIFF_BUILD_GIT_HASH") {
            None => semver.to_string(),
            Some(hash) => format!("{semver} (rev {hash})"),
        };
        let locale = if cfg!(feature = "locale") {
            " (locale support enabled)"
        } else {
            ""
        };
        write!(f, "Biff {version}{locale}")
    }
}

impl std::error::Error for Version {}
