use {bstr::BStr, jiff::Span};

use crate::{
    args::Usage,
    parse::{BytesExt, FromBytes},
};

/// Represents a biff duration.
///
/// This is just a wrapper around `jiff::Span`, which does most of the heavy
/// lifting for us. The wrapper exists so that we can keep track of what the
/// _original_ input that created the `Span` was. This is useful for error
/// reporting.
///
/// This is named `TimeSpan` mostly just so that it doesn't clash with
/// `jiff::Span`.
///
/// This type exists primarily as a target for trait impls for tailoring
/// behavior specific to `biff`.
#[derive(Clone, Debug)]
pub struct TimeSpan {
    /// The actual parsed span. i.e., The thing we operate on.
    span: Span,
}

impl TimeSpan {
    // This is phrased in a way where it accounts for spans being given as
    // positional arguments OR on stdin. This also implies that a variable
    // number of spans can be given.
    //
    // At time of writing (2025-03-16), there is no case where a span is
    // accepted as a single positional argument. If that case does arise, then
    // we'll want a different usage string (like we do for a span flag).
    pub const ARG_OR_STDIN: Usage = Usage::arg(
        "<span>",
        "A calendar or time duration, e.g., `-1d`, `-P1D`, `5yrs 2mo 1hr`.",
        r#"
A calendar or time duration.

Spans can either be passed as positional arguments or as line delimited data on
stdin, but not both. That is, spans will only be read from stdin where there
are no spans provided as positional arguments.

Spans describe a duration of time. In Biff, both calendar (years, months,
weeks, days) and physical time (hours, minutes, seconds, milliseconds,
microseconds, nanoseconds) are supported.

Spans can be in one of two formats:

ISO 8601, e.g., `PT1H2M3S`, `-P1D`, `P1Y2MT5H`

The "friendly" format, e.g., `1h2m3s`, `-1d`, `1 year, 2 months, 5 hours ago`

The ISO 8601 format comes from a standard and is widely supported. In contrast,
the "friendly" format is a bespoke format defined by Biff's underlying datetime
library (called Jiff). The "friendly" format is meant to capture a superset of
similar ad hoc formats supported in various places over the years. The benefit
of the "friendly" format is that it's terser, more flexible and arguably easier
to read.
"#,
    );

    /// Get the underlying Jiff span.
    ///
    /// If possible, prefer defining an operation on `TimeSpan` instead of
    /// using a `Span` directly. This helps centralize the operations we need,
    /// and also helps encourage consistent error reporting.
    pub fn get(&self) -> &Span {
        &self.span
    }
}

impl std::fmt::Display for TimeSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // This emits Jiff's "friendly" duration format.
        write!(f, "{:#}", self.span)
    }
}

impl std::str::FromStr for TimeSpan {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<TimeSpan> {
        s.as_bytes().parse()
    }
}

impl FromBytes for TimeSpan {
    type Err = anyhow::Error;

    fn from_bytes(s: &[u8]) -> anyhow::Result<TimeSpan> {
        let span = parse_iso_or_friendly(s)?;
        Ok(TimeSpan { span })
    }
}

impl From<Span> for TimeSpan {
    fn from(span: Span) -> TimeSpan {
        TimeSpan { span }
    }
}

impl serde::Serialize for TimeSpan {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> serde::Deserialize<'de> for TimeSpan {
    #[inline]
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<TimeSpan, D::Error> {
        use serde::de;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = TimeSpan;

            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::fmt::Result {
                f.write_str("a time span string")
            }

            #[inline]
            fn visit_bytes<E: de::Error>(
                self,
                value: &[u8],
            ) -> Result<TimeSpan, E> {
                value.parse().map_err(de::Error::custom)
            }

            #[inline]
            fn visit_str<E: de::Error>(
                self,
                value: &str,
            ) -> Result<TimeSpan, E> {
                self.visit_bytes(value.as_bytes())
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

/// A common parsing function that works in bytes.
///
/// Specifically, this parses either an ISO 8601 duration into a `Span` or
/// a "friendly" duration into a `Span`. It also tries to give decent error
/// messages.
///
/// This works because the friendly and ISO 8601 formats have non-overlapping
/// prefixes. Both can start with a `+` or `-`, but aside from that, an ISO
/// 8601 duration _always_ has to start with a `P` or `p`. We can utilize this
/// property to very quickly determine how to parse the input. We just need to
/// handle the possibly ambiguous case with a leading sign a little carefully
/// in order to ensure good error messages.
///
/// (This was copied from Jiff.)
#[inline(always)]
fn parse_iso_or_friendly(bytes: &[u8]) -> anyhow::Result<jiff::Span> {
    if bytes.is_empty() {
        anyhow::bail!(
            "an empty string is not a valid `Span`, \
             expected either a ISO 8601 or Jiff's 'friendly' \
             format",
        );
    }
    let mut first = bytes[0];
    if first == b'+' || first == b'-' {
        if bytes.len() == 1 {
            anyhow::bail!(
                "found nothing after sign `{sign}`, \
                 which is not a valid `Span`, \
                 expected either a ISO 8601 or Jiff's 'friendly' \
                 format",
                sign = BStr::new(&[first]),
            );
        }
        first = bytes[1];
    }
    if first == b'P' || first == b'p' {
        Ok(jiff::fmt::temporal::SpanParser::new().parse_span(bytes)?)
    } else {
        Ok(jiff::fmt::friendly::SpanParser::new().parse_span(bytes)?)
    }
}
