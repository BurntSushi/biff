use std::{borrow::Cow, io::Write};

use {
    anyhow::Context,
    bstr::{BStr, BString, ByteSlice, ByteVec},
    lexopt::{Arg, Parser},
};

use crate::{
    args::{self, Usage, flags},
    parse::{BufReadExt, BytesExt},
    style::Theme,
    tag::{Tag, Tagged},
};

const USAGE: &'static str = r#"
Untag tagged data.

This effectively undoes any "tagging" done by the `biff tag` commands. This
is useful to get back the original data, or even to replace tagged values in
the original data.

USAGE:
    biff untag <path>
    biff untag < line delimited tagged data

TIP:
    use -h for short docs and --help for long docs

EXAMPLES:
    Sort files checked into git via their last commit date and format the
    display such that the datetime is included with the file path:

        git ls-files \
            | biff tag exec git log -n1 --format='%cI' \
            | biff time sort \
            | biff untag -f '{tag} {data}'

    %snip-start%

    Consider a situation where you regularly work with others in different
    time zones. Perhaps you have a file with a list of co-workers and their
    time zones:

        $ cat time-zones
        Hopper	America/New_York
        Lovelace	Asia/Kolkata
        Liskov	Europe/Kyiv
        Goldberg	Australia/Tasmania

    With Biff's tagging feature, one can very easily query the current time
    for each of your co-workers and format it into easily readable output:

        $ biff tag lines --auto timezone time-zones \
            | biff time in now \
            | biff time fmt -f '%Y-%m-%d %H:%M %Z' \
            | biff untag -f '{data}\t{tag}' \
            | tabwriter
        Hopper    America/New_York    2025-03-29 12:56 EDT
        Lovelace  Asia/Kolkata        2025-03-29 22:26 IST
        Liskov    Europe/Kyiv         2025-03-29 18:56 EET
        Goldberg  Australia/Tasmania  2025-03-30 03:56 AEDT

    (One can get `tabwriter` via `cargo install tabwriter-bin`. Or use
    `column --table` from `util-linux`, which will achieve something similar.)

    Or replace RFC 3339 UTC timestamps in a Caddy log file with datetimes in
    your time zone:

        $ biff tag lines access.log \
            | biff time in system \
            | biff time fmt -f '%Y-%m-%d %H:%M:%S' \
            | biff untag -s

    %snip-end%
REQUIRED ARGUMENTS:
%args%
OPTIONS:
%flags%
"#;

pub fn run(p: &mut Parser) -> anyhow::Result<()> {
    let mut config = Config::default();
    args::configure(p, USAGE, &mut [&mut config])?;

    let mut wtr = std::io::stdout().lock();
    let mut buf = BString::new(vec![]);
    let result = config.input.reader()?.for_byte_line(|line| {
        let tagged: Tagged<String> =
            line.content().parse().with_context(|| {
                format!("line {}: failed to parse tagged data", line.number())
            })?;
        let mut data = Cow::Borrowed(tagged.data());
        if config.substitute {
            data = substitute(data, tagged.tags());
        } else if !Theme::stdout().is_none() {
            data = stylize(data, tagged.tags());
        }
        let Some(ref format) = config.format else {
            wtr.write_all(&data)?;
            return Ok(true);
        };
        let data = data.trim_end_with(|ch| ch == '\r' || ch == '\n');
        for tag in tagged.tags() {
            buf.clear();
            format.interpolate(
                tag.value().as_bytes().as_bstr(),
                data.as_bstr(),
                &mut buf,
            );
            wtr.write_all(&buf)?;
            writeln!(wtr)?;
        }
        Ok(true)
    });
    result.with_context(|| format!("{}", config.input.display()))?;
    Ok(())
}

/// Substitute each of the tags into the `data` given.
///
/// This only applies for tags that have a corresponding range into the given
/// data. e.g., They were extracted directly as literals from the data.
fn substitute<'a>(
    mut data: Cow<'a, BStr>,
    tags: &[Tag<String>],
) -> Cow<'a, BStr> {
    // When we replace multiple tags, the ranges of tags that haven't
    // been replaced yet are no longer correct unless the replacement
    // is exactly the same length, in bytes, as the tagged value. To
    // account for this, we "offset" our ranges based on the difference
    // in length between the tag's range in the original data and the
    // tag's length.
    let mut offset: isize = 0;
    for tag in tags {
        let Some(range) = tag.range() else { continue };
        let range = range.offset(offset);
        let replacement = Theme::stdout().highlight(tag.value()).to_string();
        offset += range.diff(replacement.len());
        let replacement = replacement.as_bytes().iter().copied();

        let mut new = data.into_owned();
        drop(new.splice(range.range(), replacement));
        data = Cow::Owned(new);
    }
    data
}

/// Like `substitute`, but just colorizes the ranges.
fn stylize<'a>(
    mut data: Cow<'a, BStr>,
    tags: &[Tag<String>],
) -> Cow<'a, BStr> {
    let mut offset: isize = 0;
    for tag in tags {
        let Some(range) = tag.range() else { continue };
        let range = range.offset(offset);
        let replacement =
            Theme::stdout().highlight(&data[range.range()]).to_string();
        offset += range.diff(replacement.len());
        let replacement = replacement.as_bytes().iter().copied();

        let mut new = data.into_owned();
        drop(new.splice(range.range(), replacement));
        data = Cow::Owned(new);
    }
    data
}

/// A representation of a format string.
///
/// The representation is a sequence of literals interspersed by formatting
/// directives.
#[derive(Debug)]
struct Format {
    items: Vec<FormatItem>,
}

/// An individual item in a format string.
#[derive(Debug)]
enum FormatItem {
    Literal(BString),
    Tag,
    Data,
}

impl Format {
    /// Interpolate the formatting directives into `dst` using the given
    /// `tag` and `data`.
    ///
    /// Callers are responsible for clearing `dst`.
    fn interpolate(&self, tag: &BStr, data: &BStr, dst: &mut BString) {
        for item in self.items.iter() {
            match *item {
                FormatItem::Literal(ref literal) => {
                    dst.extend_from_slice(&literal);
                }
                FormatItem::Tag => {
                    if Theme::stdout().is_none() {
                        dst.extend_from_slice(tag);
                    } else {
                        let tag = Theme::stdout().highlight(tag).to_string();
                        dst.extend_from_slice(tag.as_bytes());
                    }
                }
                FormatItem::Data => {
                    dst.extend_from_slice(data);
                }
            }
        }
    }
}

impl std::str::FromStr for Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Format> {
        enum State {
            Default,
            InBrace,
            Backslash,
            BackslashInBrace,
        }

        let mut items = vec![];
        let mut literal = BString::new(vec![]);
        let mut name = BString::new(vec![]);
        let mut state = State::Default;
        for byte in Vec::unescape_bytes(s) {
            state = match (state, byte) {
                (State::Default, b'{') => {
                    if !literal.is_empty() {
                        let literal = std::mem::take(&mut literal);
                        items.push(FormatItem::Literal(literal));
                    }
                    State::InBrace
                }
                (State::Default, b'\\') => State::Backslash,
                (State::Default, _) => {
                    literal.push(byte);
                    State::Default
                }
                (State::InBrace, b'}') => {
                    let item = match name.as_bytes() {
                        b"tag" => FormatItem::Tag,
                        b"data" => FormatItem::Data,
                        _ => anyhow::bail!(
                            "unrecognized format directive `{{{name}}}`, \
                             allowed directives are `{{tag}}` and `{{data}}`",
                        ),
                    };
                    name.clear();
                    items.push(item);
                    State::Default
                }
                (State::InBrace, b'\\') => State::BackslashInBrace,
                (State::InBrace, _) => {
                    name.push(byte);
                    State::InBrace
                }
                (State::Backslash, _) => {
                    literal.push(byte);
                    State::Default
                }
                (State::BackslashInBrace, _) => {
                    name.push(byte);
                    State::InBrace
                }
            };
        }
        match state {
            State::Default => {
                if !literal.is_empty() {
                    items.push(FormatItem::Literal(literal));
                }
            }
            State::InBrace => anyhow::bail!(
                "found unclosed brace, which might be an invalid \
                 format directive (to write a brace literally, escape \
                 it with a backslash)",
            ),
            State::Backslash | State::BackslashInBrace => anyhow::bail!(
                "found dangling backslash (to write a backslash \
                 literally, escape it with a backslash)",
            ),
        }
        Ok(Format { items })
    }
}

#[derive(Debug, Default)]
struct Config {
    input: flags::FileOrStdin,
    substitute: bool,
    format: Option<Format>,
}

impl args::Configurable for Config {
    fn configure(
        &mut self,
        p: &mut Parser,
        arg: &mut Arg,
    ) -> anyhow::Result<bool> {
        match *arg {
            Arg::Short('s') | Arg::Long("substitute") => {
                self.substitute = true;
            }
            Arg::Short('f') | Arg::Long("format") => {
                self.format = Some(args::parse(p, "-f/--format")?);
            }
            Arg::Value(ref mut v) => {
                self.input.set(std::mem::take(v))?;
            }
            _ => return Ok(false),
        }
        Ok(true)
    }

    fn usage(&self) -> &[Usage] {
        const PATH: Usage = Usage::arg(
            "<path>",
            "A file path to read line delimited tagged data from.",
            r#"
A file path to read line delimited tagged data from.

In lieu of a specific file path, users may also pass line delimited tagged data
into stdin.
"#,
        );

        const SUBSTITUTE: Usage = Usage::flag(
            "-s/--substitute",
            "Substitue tags back into their original location.",
            r#"
Substitue tags back into their original location.

This only applies when the tag contains an associated byte range into the
original data it was extracted from. For example, this range is created by the
`bigg tag lines` command.

This is useful when one extracts a datetime tag and turns it into a different
format with, e.g., `biff time fmt`. This flag will then substitute the original
datetime in the source with the formatted datetime. For example, this can be
used to localize RFC 3339 Zulu timestamps in arbitrary line oriented data.
"#,
        );

        const FORMAT: Usage = Usage::flag(
            "-f/--format",
            "An interpolation format string to use for untagging.",
            r#"
An interpolation format string to use for untagging.

The default operation of this command is to "untag" tagged data by removing
the tags and printing the raw data that was tagged. (Unless `-s/--substitute`
is used, in which case, the tags are inserted into the raw data before being
printed if they have a byte range associated with them.) In contrast, this
flag permits changing what data is printed.

This flag accepts an arbitrary string that is printed as-is for each piece of
tagged data. The string may contain any number of the following formatting
directives:

`{tag}`: interpolate the tagged value. When there are multiple tags,
interpolation occurs for each tag.

`{data}`: interpolate the original data. This is replaced with the original
data for each tag.

When using a format string, if there are no tags for a piece of tagged data,
then interpolation is skipped entirely for that data.
"#,
        );

        &[PATH, SUBSTITUTE, FORMAT]
    }
}
