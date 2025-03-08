use std::{borrow::Cow, path::Path};

use {
    anyhow::Context,
    bstr::{BStr, BString, ByteSlice, ByteVec},
};

use crate::{
    args::Configurable,
    datetime::{DateTime, DateTimeFlexible},
    parse::{BufReadExt, BytesExt, Line, LineBuf},
    span::TimeSpan,
    tag::MaybeTagged,
    timezone::TimeZone,
};

/// The CLI parsing configuration for reading datetimes.
///
/// This will greedily consume all remaining positional arguments as datetimes.
///
/// When there are no positional arguments to consume, then this will read
/// datetimes from `stdin` in a line delimited fashion.
#[derive(Clone, Debug, Default)]
pub struct DateTimes(Arguments);

impl DateTimes {
    /// Run the given function over each datetime read from the CLI.
    ///
    /// If there were no positional datetimes, then this tries to read them
    /// from stdin, one per line.
    ///
    /// Iteration stops when the closure returns false or returns an error.
    pub fn try_map(
        self,
        mut f: impl FnMut(MaybeTagged<'static, DateTime>) -> anyhow::Result<bool>,
    ) -> anyhow::Result<()> {
        self.0.try_map(|arg| f(arg.to_datetime()?))
    }
}

impl Configurable for DateTimes {
    fn configure(
        &mut self,
        p: &mut lexopt::Parser,
        arg: &mut lexopt::Arg,
    ) -> anyhow::Result<bool> {
        self.0.configure(p, arg)
    }
}

/// The CLI parsing configuration for reading spans.
///
/// This will greedily consume all remaining positional arguments as spans.
///
/// When there are no positional arguments left to consume, then asking this
/// for an iterator of spans read will instead read from `stdin` in a streaming
/// fashion for newline-delimited spans.
#[derive(Clone, Debug, Default)]
pub struct Spans(Arguments);

impl Spans {
    /// Run the given function over each span read from the CLI.
    ///
    /// If there were no positional spans, then this tries to read them
    /// from stdin, one per line.
    ///
    /// Iteration stops when the closure returns false or returns an error.
    pub fn try_map(
        self,
        mut f: impl FnMut(MaybeTagged<'static, TimeSpan>) -> anyhow::Result<bool>,
    ) -> anyhow::Result<()> {
        self.0.try_map(|arg| f(arg.to_span()?))
    }
}

impl Configurable for Spans {
    fn configure(
        &mut self,
        p: &mut lexopt::Parser,
        arg: &mut lexopt::Arg,
    ) -> anyhow::Result<bool> {
        self.0.configure(p, arg)
    }
}

/// The CLI parsing configuration for arbitrary arguments that may be tagged.
///
/// This will greedily consume all remaining positional arguments as spans.
///
/// When there are no positional arguments left to consume, then asking this
/// for an iterator of arguments read will instead read from `stdin` in a
/// streaming fashion for newline-delimited arguments.
#[derive(Clone, Debug, Default)]
pub struct MaybeTaggedArguments(Arguments);

impl MaybeTaggedArguments {
    /// Run the given function over each argument read from the CLI.
    ///
    /// If there were no positional arguments, then this tries to read them
    /// from stdin, one per line.
    ///
    /// Iteration stops when the closure returns false or returns an error.
    pub fn try_map(
        self,
        mut f: impl FnMut(
            MaybeTagged<'static, Cow<'static, BStr>>,
        ) -> anyhow::Result<bool>,
    ) -> anyhow::Result<()> {
        self.0.try_map(|arg| match arg {
            Argument::Positional(arg) => {
                f(MaybeTagged::Untagged(Cow::Owned(arg)))
            }
            Argument::StdinLine(line) => {
                let maybe_tagged = line
                    .content()
                    .parse::<MaybeTagged<'static, Cow<'static, BStr>>>()?;
                f(maybe_tagged)
            }
            Argument::StdinLineBuf(line) => {
                let maybe_tagged = line
                    .content()
                    .parse::<MaybeTagged<'static, Cow<'static, BStr>>>()?;
                f(maybe_tagged)
            }
        })
    }
}

impl Configurable for MaybeTaggedArguments {
    fn configure(
        &mut self,
        p: &mut lexopt::Parser,
        arg: &mut lexopt::Arg,
    ) -> anyhow::Result<bool> {
        self.0.configure(p, arg)
    }
}

/// The parsing configuration for reading arguments either as positional
/// arguments on the CLI, or as line-delimited data on `stdin`.
///
/// This will greedily consume all remaining positional arguments. That is,
/// this is generally intended for use cases where a variable number of
/// arguments can be given.
///
/// When there are _zero_ positional arguments, then this will read lines from
/// stdin instead.
#[derive(Clone, Debug, Default)]
pub struct Arguments {
    positional: Vec<Argument<'static>>,
}

impl Arguments {
    /// Run the given function over each argument read from the CLI.
    ///
    /// If there were no positional arguments, then this tries to read them
    /// from stdin, one per line. Stated differently, the argument given
    /// to the closure is either always `Positional` or always `StdinLine`.
    /// You can never get a mix.
    ///
    /// Iteration stops when the closure returns false or returns an error.
    pub fn try_map(
        self,
        mut f: impl FnMut(Argument<'_>) -> anyhow::Result<bool>,
    ) -> anyhow::Result<()> {
        if !self.positional.is_empty() {
            for arg in self.positional {
                if !f(arg)? {
                    return Ok(());
                }
            }
            return Ok(());
        }
        std::io::stdin().lock().for_byte_line(|line| {
            f(Argument::StdinLine(line))
                .with_context(|| format!("line {} of <stdin>", line.number()))
        })
    }
}

impl Configurable for Arguments {
    fn configure(
        &mut self,
        _: &mut lexopt::Parser,
        arg: &mut lexopt::Arg,
    ) -> anyhow::Result<bool> {
        match *arg {
            lexopt::Arg::Value(ref mut v) => {
                let v = std::mem::take(v);
                let bytes = Vec::from_os_string(v).map_err(|arg| {
                    anyhow::anyhow!(
                        "biff requires that positional arguments \
                         be valid UTF-8 in non-Unix environments, \
                         but `{arg:?}` is not valid UTF-8",
                    )
                })?;
                self.positional
                    .push(Argument::Positional(BString::from(bytes)));
            }
            _ => return Ok(false),
        }
        Ok(true)
    }
}

/// A generic argument parsed from either positional args on the CLI, or
/// as a single line from stdin.
#[derive(Clone, Debug)]
pub enum Argument<'a> {
    /// Just arbitrary bytes.
    ///
    /// On Windows, we require that this is valid UTF-8.
    Positional(BString),
    /// A line containing arbitrary ASCII compatible bytes.
    ///
    /// This specifically provides access to the line terminator for cases
    /// where commands want to pass through the original data unchanged.
    StdinLine(Line<'a>),
    /// Like `StdinLine`, but an owned copy.
    StdinLineBuf(LineBuf),
}

impl<'a> Argument<'a> {
    /// Converts this argument into a possibly owned file path.
    ///
    /// On non-Unix systems, this fails when the argument is not valid UTF-8.
    /// On Unix, this is guaranteed to never return an error.
    pub fn into_path(self) -> anyhow::Result<Cow<'a, Path>> {
        match self {
            Argument::Positional(arg) => {
                let path = Vec::from(arg).into_path_buf().map_err(|err| {
                    anyhow::anyhow!(
                        "invalid file path `{path}`: \
                         file paths in non-Unix environments \
                         must be valid UTF-8",
                        path = err.as_bytes().as_bstr(),
                    )
                })?;
                Ok(Cow::Owned(path))
            }
            Argument::StdinLine(line) => {
                let path = line.content().to_path().with_context(|| {
                    format!(
                        "invalid file path `{path}` on line {line_number}: \
                         file paths in non-Unix environments \
                         must be valid UTF-8",
                        path = line.content(),
                        line_number = line.number(),
                    )
                })?;
                Ok(Cow::Borrowed(path))
            }
            Argument::StdinLineBuf(line) => {
                let line_number = line.number();
                let line = line.into_content();
                let path = Vec::from(line).into_path_buf().map_err(|err| {
                    anyhow::anyhow!(
                        "invalid file path `{path}` on line {line_number}: \
                         file paths in non-Unix environments \
                         must be valid UTF-8",
                        path = err.as_bytes().as_bstr(),
                    )
                })?;
                Ok(Cow::Owned(path))
            }
        }
    }

    /// Converts this argument into a file path borrowed from this `Argument`.
    ///
    /// On non-Unix systems, this fails when the argument is not valid UTF-8.
    /// On Unix, this is guaranteed to never return an error.
    pub fn to_path(&self) -> anyhow::Result<&Path> {
        let path = self.raw();
        path.to_path().with_context(|| {
            if let Some(line_number) = self.line_number() {
                format!(
                    "invalid file path `{path}` on line {line_number}: \
                     file paths in non-Unix environments \
                     must be valid UTF-8",
                )
            } else {
                format!(
                    "invalid file path `{path}`: \
                     file paths in non-Unix environments \
                     must be valid UTF-8",
                )
            }
        })
    }

    /// Parse this argument into a possibly tagged datetime.
    pub fn to_datetime(
        &self,
    ) -> anyhow::Result<MaybeTagged<'static, DateTime>> {
        match *self {
            Argument::Positional(ref arg) => {
                let dt: DateTimeFlexible =
                    arg.parse().context("invalid datetime")?;
                Ok(MaybeTagged::Untagged(dt.into()))
            }
            Argument::StdinLine(line) => line
                .content()
                .parse::<MaybeTagged<'static, DateTime>>()
                .context("invalid datetime"),
            Argument::StdinLineBuf(ref line) => line
                .content()
                .parse::<MaybeTagged<'static, DateTime>>()
                .context("invalid datetime"),
        }
    }

    /// Parse this argument into a possibly tagged time span.
    pub fn to_span(&self) -> anyhow::Result<MaybeTagged<'static, TimeSpan>> {
        match *self {
            Argument::Positional(ref arg) => {
                let span = arg.parse().context("invalid time span")?;
                Ok(MaybeTagged::Untagged(span))
            }
            Argument::StdinLine(line) => line
                .content()
                .parse::<MaybeTagged<'static, TimeSpan>>()
                .context("invalid time span"),
            Argument::StdinLineBuf(ref line) => line
                .content()
                .parse::<MaybeTagged<'static, TimeSpan>>()
                .context("invalid time span"),
        }
    }

    /// Parse this argument into a possibly tagged time zone.
    pub fn to_time_zone(
        &self,
    ) -> anyhow::Result<MaybeTagged<'static, TimeZone>> {
        match *self {
            Argument::Positional(ref arg) => {
                let span = arg.parse().context("invalid time zone")?;
                Ok(MaybeTagged::Untagged(span))
            }
            Argument::StdinLine(line) => line
                .content()
                .parse::<MaybeTagged<'static, TimeZone>>()
                .context("invalid time zone"),
            Argument::StdinLineBuf(ref line) => line
                .content()
                .parse::<MaybeTagged<'static, TimeZone>>()
                .context("invalid time zone"),
        }
    }

    /// Returns the original argument as a byte string with a line terminator.
    ///
    /// This is a bit of a weird method, but its purpose is to pass through
    /// data, as given by the end user, unchanged while abstracting over
    /// arguments from the CLI versus from stdin.
    ///
    /// When an argument is from stdin, it either has a line terminator or it's
    /// the last line in the input without a line terminator. In this case,
    /// the original line, in full, with its line terminator, is returned.
    /// When it's the last line without a line terminator, then no line
    /// terminator is included, because that matches the source data.
    ///
    /// But if an argument is from the CLI, then a `\n` line terminator is
    /// specifically inserted in _all_ cases. Which is why a `Cow` is returend.
    ///
    /// In other words, for arguments from stdin, you just get line delimited
    /// data as-is with no copying. But for positional arguments, you get
    /// a copied argument with an artificial line terminator inserted.
    ///
    /// Basically, this abstraction lets callers treat arguments *as if* it
    /// were line delimited data and without needing to worry about inserting
    /// line terminators themselves, and while preserving the exact data
    /// coming from the end user.
    pub fn original_with_line_terminator(&self) -> Cow<'a, BStr> {
        match self {
            Argument::Positional(arg) => {
                let mut data = arg.clone();
                data.push(b'\n');
                Cow::Owned(data)
            }
            Argument::StdinLine(line) => Cow::Borrowed(line.full()),
            Argument::StdinLineBuf(line) => Cow::Owned(line.full().into()),
        }
    }

    /// Return this argument as an owned value with no borrowed data.
    pub fn into_owned(self) -> Argument<'static> {
        match self {
            Argument::Positional(arg) => Argument::Positional(arg),
            Argument::StdinLine(line) => {
                Argument::StdinLineBuf(line.to_owned())
            }
            Argument::StdinLineBuf(line) => Argument::StdinLineBuf(line),
        }
    }

    /// Return the raw argument value.
    pub fn raw(&self) -> &BStr {
        match *self {
            Argument::Positional(ref arg) => arg.as_bstr(),
            Argument::StdinLine(line) => line.content(),
            Argument::StdinLineBuf(ref line) => line.content(),
        }
    }

    /// Consume this argument into its raw value.
    ///
    /// If this was a borrowed line, then this makes a copy of the line
    /// contents. If it was a borrowed or owned line, then the line terminator
    /// is dropped.
    #[expect(dead_code)]
    pub fn into_raw(self) -> BString {
        match self {
            Argument::Positional(arg) => arg,
            Argument::StdinLine(line) => line.content().into(),
            Argument::StdinLineBuf(line) => line.into_content(),
        }
    }

    /// Returns a line number associated with this argument, if it was parsed
    /// from stdin.
    fn line_number(&self) -> Option<usize> {
        match *self {
            Argument::Positional(_) => None,
            Argument::StdinLine(line) => Some(line.number()),
            Argument::StdinLineBuf(ref line) => Some(line.number()),
        }
    }
}
