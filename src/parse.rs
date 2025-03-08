use std::{
    borrow::Cow,
    convert::Infallible,
    ffi::{OsStr, OsString},
};

use bstr::{BStr, BString, ByteSlice, ByteVec};

/// The `FromStr` analog for `&[u8]`.
pub trait FromBytes: Sized {
    type Err;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Err>;
}

impl FromBytes for Vec<u8> {
    type Err = Infallible;

    fn from_bytes(bytes: &[u8]) -> Result<Vec<u8>, Infallible> {
        Ok(bytes.to_vec())
    }
}

impl FromBytes for Cow<'static, [u8]> {
    type Err = Infallible;

    fn from_bytes(bytes: &[u8]) -> Result<Cow<'static, [u8]>, Infallible> {
        Ok(bytes.to_vec().into())
    }
}

impl FromBytes for BString {
    type Err = Infallible;

    fn from_bytes(bytes: &[u8]) -> Result<BString, Infallible> {
        Ok(bytes.to_vec().into())
    }
}

impl FromBytes for Cow<'static, BStr> {
    type Err = Infallible;

    fn from_bytes(bytes: &[u8]) -> Result<Cow<'static, BStr>, Infallible> {
        Ok(BString::from(bytes.to_vec()).into())
    }
}

/// A simple extension trait that adds some methods to byte slices.
///
/// `bstr` already gives us most of what we need.
pub trait BytesExt {
    // This would be more naturally named `as_bytes()`, but that creates
    // conflicts with other `as_bytes()` methods.
    fn as_byte_slice(&self) -> &[u8];

    fn parse<T: FromBytes>(&self) -> Result<T, <T as FromBytes>::Err> {
        FromBytes::from_bytes(self.as_byte_slice())
    }
}

impl BytesExt for [u8] {
    fn as_byte_slice(&self) -> &[u8] {
        self
    }
}

/// A simple extension trait that adds some methods to OS strings.
pub trait OsStrExt {
    // Named more verbosely for similar reasons as `BytesExt::as_byte_slice`.
    fn as_os_str_slice(&self) -> &OsStr;

    /// Converts this OS string to a byte slice.
    ///
    /// On Unix, this is a no-op and can never fail. Otherwise, this requires
    /// that the OS string be valid UTF-8.
    fn to_bytes(&self) -> anyhow::Result<&[u8]> {
        let osstr = self.as_os_str_slice();
        <[u8]>::from_os_str(osstr).ok_or_else(|| {
            anyhow::anyhow!(
                "{osstr:?} is not valid UTF-8 but must be \
                 in non-Unix environments",
            )
        })
    }

    /// Converts this OS string to a string slice.
    fn to_str(&self) -> anyhow::Result<&str> {
        Ok(self.to_bytes()?.to_str()?)
    }

    fn parse<T: FromBytes<Err = anyhow::Error>>(&self) -> anyhow::Result<T> {
        FromBytes::from_bytes(self.to_bytes()?)
    }
}

impl OsStrExt for OsStr {
    fn as_os_str_slice(&self) -> &OsStr {
        self
    }
}

impl OsStrExt for OsString {
    fn as_os_str_slice(&self) -> &OsStr {
        self.as_os_str()
    }
}

/// A borrowed line parsed from a stream.
///
/// This is meant to give you access to various
/// parts of the line including the *full* line.
/// It absolves callers of needing to futz with
/// line terminators or line numbers.
#[derive(Clone, Copy, Debug)]
pub struct Line<'a> {
    /// The line number, 1-indexed.
    number: usize,
    /// The full line including its line terminator if present.
    full: &'a BStr,
}

impl<'a> Line<'a> {
    pub fn new(number: usize, full: &'a [u8]) -> Line<'a> {
        Line { number, full: full.as_bstr() }
    }

    /// Return the one-indexed line number of this line.
    pub fn number(&self) -> usize {
        self.number
    }

    /// Return the full line including its optional terminator.
    pub fn full(&self) -> &'a BStr {
        self.full
    }

    /// Return only the content of the line, i.e., the line without its
    /// terminator (if present).
    pub fn content(&self) -> &'a BStr {
        let (content, _) = split_line_terminator(self.full);
        content.as_bstr()
    }

    /// Return only the line's terminator. This is guaranteed to be empty,
    /// `\r\n` or `\n`.
    #[expect(dead_code)]
    pub fn terminator(&self) -> &'a BStr {
        let (_, terminator) = split_line_terminator(self.full);
        terminator.as_bstr()
    }

    /// Turn this borrowed line into an owned line.
    pub fn to_owned(self) -> LineBuf {
        LineBuf { number: self.number, full: self.full.into() }
    }
}

/// An owned line parsed from a stream.
///
/// This is meant to give you access to various
/// parts of the line including the *full* line.
/// It absolves callers of needing to futz with
/// line terminators or line numbers.
#[derive(Clone, Debug)]
pub struct LineBuf {
    /// The line number, 1-indexed.
    number: usize,
    /// The full line including its line terminator if present.
    full: BString,
}

impl LineBuf {
    /// Return the one-indexed line number of this line.
    pub fn number(&self) -> usize {
        self.number
    }

    /// Return the full line including its optional terminator.
    pub fn full(&self) -> &BStr {
        self.full.as_bstr()
    }

    /// Consume this `Line` and return the original line in its entirety,
    /// including its optional line terminator.
    #[expect(dead_code)]
    pub fn into_full(self) -> BString {
        self.full
    }

    /// Return only the content of the line, i.e., the line without its
    /// terminator (if present).
    pub fn content(&self) -> &BStr {
        let (content, _) = split_line_terminator(&self.full);
        content.as_bstr()
    }

    /// Consume this `Line`, strip any existing line terminator and return
    /// whatever remains.
    pub fn into_content(mut self) -> BString {
        if self.full.last_byte() == Some(b'\n') {
            self.full.pop().unwrap();
            if self.full.last_byte() == Some(b'\r') {
                self.full.pop().unwrap();
            }
        }
        self.full
    }

    /// Return only the line's terminator. This is guaranteed to be empty,
    /// `\r\n` or `\n`.
    #[expect(dead_code)]
    pub fn terminator(&self) -> &BStr {
        let (_, terminator) = split_line_terminator(&self.full);
        terminator.as_bstr()
    }

    /// Return this owned line as a borrowed line.
    #[expect(dead_code)]
    pub fn as_ref(&self) -> Line<'_> {
        Line { number: self.number, full: self.full.as_bstr() }
    }
}

// BREADCRUMBS: I think we should revisit the API below. Right now, it's
// oriented around lines. But I wonder if it would be better oriented around
// buffers. The idea being, we may want to send a buffer to another thread.
// The problem there is reuse. Ideally we could reuse the buffers, but this
// means getting the buffer back from the thread it was sent to once the
// thread is done with it. Which seems... non-ideal? Maybe think more about
// "iterating over lines, in parallel" more holistically...

/// An extension trait for `std::io::BufRead` which provides convenience APIs
/// for dealing with byte strings.
///
/// This is a stripped down version of what's in `bstr::io`. It's copied here
/// instead of just using `bstr::io` because having a `std::io::Result`
/// return type is supremely annoying when working with `anyhow`. This is just
/// supremely annoying and it feels like an API design mistake in `bstr`. But
/// it's not totally clear how to fix it without more API machinery to make
/// the error type generic but still support propagating `std::io::Error`.
pub trait BufReadExt: std::io::BufRead {
    /// Executes the given closure on each (`\n`|`\r\n`)-terminated line in the
    /// underlying reader.
    fn for_byte_line<F>(&mut self, mut for_each_line: F) -> anyhow::Result<()>
    where
        Self: Sized,
        F: FnMut(Line<'_>) -> anyhow::Result<bool>,
    {
        let mut number = 0;
        let mut bytes = vec![];
        let mut res = Ok(());
        let mut consumed = 0;
        'outer: loop {
            // Lend out complete record slices from our buffer
            {
                let mut buf = self.fill_buf()?;
                if buf.is_empty() {
                    break;
                }
                while let Some(index) = buf.find_byte(b'\n') {
                    let (record, rest) = buf.split_at(index + 1);
                    buf = rest;
                    consumed += record.len();
                    number += 1;
                    match for_each_line(Line::new(number, record)) {
                        Ok(false) => break 'outer,
                        Err(err) => {
                            res = Err(err);
                            break 'outer;
                        }
                        _ => (),
                    }
                }

                // Copy the final record fragment to our local buffer. This
                // saves read_until() from re-scanning a buffer we know
                // contains no remaining terminators.
                bytes.extend_from_slice(buf);
                consumed += buf.len();
            }

            self.consume(consumed);
            consumed = 0;

            // N.B. read_until uses a different version of memchr that may
            // be slower than the memchr crate that bstr uses. However, this
            // should only run for a fairly small number of records, assuming a
            // decent buffer size.
            self.read_until(b'\n', &mut bytes)?;
            if bytes.is_empty() {
                break;
            }
            number += 1;
            if !for_each_line(Line::new(number, &bytes))? {
                break;
            }
            bytes.clear();
        }
        self.consume(consumed);
        res
    }
}

impl<B: std::io::BufRead> BufReadExt for B {}

/// A type that wraps `Cow<'a, [u8]>` and treats the bytes as text if possible.
///
/// The value of this type comes from its Serde integration. Specifically,
/// if the data is UTF-8, then it (de)serializes as a normal string. If it
/// isn't UTF-8, then it is still treated as "text," but is (de)serialized
/// at UTF-8 via an escaping mechanism. It is non-lossy.
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct TextBytes<'a>(pub Cow<'a, BStr>);

impl<'a> TextBytes<'a> {
    pub fn into_owned(self) -> TextBytes<'static> {
        match self.0 {
            Cow::Borrowed(bytes) => TextBytes(Cow::Owned(bytes.into())),
            Cow::Owned(bytes) => TextBytes(Cow::Owned(bytes)),
        }
    }
}

impl<'a> std::ops::Deref for TextBytes<'a> {
    type Target = BStr;

    fn deref(&self) -> &BStr {
        self.0.deref()
    }
}

impl<'a> From<TextBytes<'a>> for Cow<'a, BStr> {
    fn from(tb: TextBytes<'a>) -> Cow<'a, BStr> {
        tb.0
    }
}

impl<'a> From<&'a [u8]> for TextBytes<'a> {
    fn from(bytes: &'a [u8]) -> TextBytes<'a> {
        TextBytes(Cow::Borrowed(bytes.as_bstr()))
    }
}

impl From<Vec<u8>> for TextBytes<'static> {
    fn from(bytes: Vec<u8>) -> TextBytes<'static> {
        TextBytes(Cow::Owned(BString::from(bytes)))
    }
}

impl<'a> serde::Serialize for TextBytes<'a> {
    fn serialize<S: serde::Serializer>(
        &self,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;

        let mut state = s.serialize_struct("TextBytes", 1)?;
        match self.0.to_str() {
            Ok(text) => state.serialize_field("text", text)?,
            Err(_) => state.serialize_field(
                "bytes",
                &self.0.escape_bytes().to_string(),
            )?,
        }
        state.end()
    }
}

// This is painful, but these are the things we do to keep the dependency
// tree slim and compile times quick.
//
// Ref: https://serde.rs/deserialize-struct.html
impl<'a, 'de> serde::Deserialize<'de> for TextBytes<'a> {
    #[inline]
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        use serde::de;

        enum Field {
            Text,
            Bytes,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> serde::de::Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(
                        &self,
                        f: &mut std::fmt::Formatter,
                    ) -> std::fmt::Result {
                        f.write_str("`text` or `bytes`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "text" => Ok(Field::Text),
                            "bytes" => Ok(Field::Bytes),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TextBytes<'static>;

            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                formatter.write_str(
                    "a UTF-8 encoded string keyed by either `text` or `bytes`",
                )
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut decoded: Option<Vec<u8>> = None;
                let mut parsed: Option<&'static str> = None;
                while let Some(key) = map.next_key()? {
                    if let Some(parsed) = parsed {
                        return Err(de::Error::custom(format_args!(
                            "already parsed `{parsed}`, only one \
                             string permitted and it must be \
                             `text` or `bytes`",
                        )));
                    }
                    match key {
                        Field::Text => {
                            parsed = Some("text");
                            let text: String = map.next_value()?;
                            decoded = Some(text.into_bytes());
                        }
                        Field::Bytes => {
                            parsed = Some("bytes");
                            let encoded: String = map.next_value()?;
                            decoded = Some(Vec::unescape_bytes(encoded));
                        }
                    }
                }
                let bytes = decoded.ok_or_else(|| {
                    de::Error::missing_field("`text` or `bytes`")
                })?;
                Ok(TextBytes(Cow::Owned(bytes.into())))
            }
        }

        const FIELDS: &[&str] = &["text", "bytes"];
        deserializer.deserialize_struct("TextBytes", FIELDS, Visitor)
    }
}

fn split_line_terminator(line: &[u8]) -> (&[u8], &[u8]) {
    let mut terminator_at = line.len();
    if line.last_byte() == Some(b'\n') {
        terminator_at -= 1;
        if line.last_byte() == Some(b'\r') {
            terminator_at -= 1;
        }
    }
    (&line[..terminator_at], &line[terminator_at..])
}
