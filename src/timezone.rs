use std::sync::LazyLock;

use jiff::fmt::{StdFmtWrite, temporal};

use crate::{
    args::Usage,
    parse::{BytesExt, FromBytes},
};

/// Returns a list of available time zones sorted by IANA time zone identifier.
pub fn available() -> &'static [String] {
    static IDS: LazyLock<Vec<String>> = LazyLock::new(|| {
        let mut names: Vec<String> = jiff::tz::db()
            .available()
            .filter_map(|name| {
                let name = name.as_str();
                // We skip these time zones since they are a little weird and
                // not usually what users want. They are still available when
                // present and Biff will happily accept them. But they usually
                // have an entry for each "normal" time zone, which ends up
                // making the available list very noisy.
                if name.starts_with("posix/") || name.starts_with("right/") {
                    return None;
                }
                Some(name.to_string())
            })
            .collect();
        names.sort();
        names
    });
    &**IDS
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimeZone {
    /// The actual parsed time zone. i.e., The thing we operate on.
    tz: jiff::tz::TimeZone,
}

impl TimeZone {
    pub const ARG_OR_STDIN: Usage = Usage::arg(
        "<time-zone>",
        "A time zone string, e.g., `Australia/Sydney` or `+11:00`.",
        r#"
A time zone string.

Time zones can either be passed as positional arguments or as line delimited
data on stdin, but not both. That is, time zones will only be read from stdin
when there are no time zones provided as positional arguments.

Biff accepts a few different formats for time zones automatically. They fall
into three broad categories:

IANA time zone identifiers such as `America/New_York` or `Australia/Sydney`.

Specific offsets such as `-05:00` or `+1100`.

POSIX time zone strings such as `EST5EDT,M3.2.0,M11.1.0`.

The special string `system` is also accepted. This refers to the time zone
automatically detected by Biff from your system's configuration. On Unix
systems for example, this is usually determined by examining the symbolic link
at `/etc/localtime`. This can also be overridden via the `TZ` environment
variable.
"#,
    );

    pub const ARG: Usage = Usage::arg(
        "<time-zone>",
        "A time zone string, e.g., `Australia/Sydney` or `+11:00`.",
        r#"
A time zone string.

Biff accepts a few different formats for time zones automatically. They fall
into three broad categories:

IANA time zone identifiers such as `America/New_York` or `Australia/Sydney`.

Specific offsets such as `-05:00` or `+1100`.

POSIX time zone strings such as `EST5EDT,M3.2.0,M11.1.0`.

The special string `system` is also accepted. This refers to the time zone
automatically detected by Biff from your system's configuration. On Unix
systems for example, this is usually determined by examining the symbolic link
at `/etc/localtime`. This can also be overridden via the `TZ` environment
variable.
"#,
    );

    pub fn system() -> TimeZone {
        TimeZone { tz: crate::TZ.clone() }
    }

    pub fn get(&self) -> &jiff::tz::TimeZone {
        &self.tz
    }
}

impl std::fmt::Display for TimeZone {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        static PRINTER: temporal::DateTimePrinter =
            temporal::DateTimePrinter::new();

        PRINTER
            .print_time_zone(&self.tz, StdFmtWrite(f))
            .map_err(|_| std::fmt::Error)
    }
}

impl std::str::FromStr for TimeZone {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<TimeZone> {
        s.as_bytes().parse()
    }
}

impl FromBytes for TimeZone {
    type Err = anyhow::Error;

    fn from_bytes(s: &[u8]) -> anyhow::Result<TimeZone> {
        static PARSER: temporal::DateTimeParser =
            temporal::DateTimeParser::new();

        if s == b"system" {
            return Ok(TimeZone::system());
        }
        Ok(PARSER.parse_time_zone(s).map(|tz| TimeZone { tz })?)
    }
}

impl serde::Serialize for TimeZone {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> serde::Deserialize<'de> for TimeZone {
    #[inline]
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<TimeZone, D::Error> {
        use serde::de;

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = TimeZone;

            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::fmt::Result {
                f.write_str("a time zone string")
            }

            #[inline]
            fn visit_bytes<E: de::Error>(
                self,
                value: &[u8],
            ) -> Result<TimeZone, E> {
                value.parse().map_err(de::Error::custom)
            }

            #[inline]
            fn visit_str<E: de::Error>(
                self,
                value: &str,
            ) -> Result<TimeZone, E> {
                self.visit_bytes(value.as_bytes())
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}
