use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    ops::Range,
};

use bstr::BStr;

use crate::parse::{BytesExt, FromBytes, TextBytes};

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum MaybeTagged<'a, T> {
    Untagged(T),
    Tagged(Tagged<'a, T>),
}

impl<'a, T> MaybeTagged<'a, T> {
    #[expect(dead_code)]
    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> MaybeTagged<'a, U> {
        match self {
            MaybeTagged::Untagged(t) => MaybeTagged::Untagged(f(t)),
            MaybeTagged::Tagged(t) => MaybeTagged::Tagged(t.map(f)),
        }
    }

    pub fn try_map<U>(
        self,
        mut f: impl FnMut(T) -> anyhow::Result<U>,
    ) -> anyhow::Result<MaybeTagged<'a, U>> {
        match self {
            MaybeTagged::Untagged(t) => Ok(MaybeTagged::Untagged(f(t)?)),
            MaybeTagged::Tagged(t) => Ok(MaybeTagged::Tagged(t.try_map(f)?)),
        }
    }
}

impl<'a, T: Display + serde::Serialize> MaybeTagged<'a, T> {
    pub fn write(&self, mut wtr: impl std::io::Write) -> anyhow::Result<()> {
        match *self {
            MaybeTagged::Untagged(ref t) => write!(wtr, "{t}")?,
            MaybeTagged::Tagged(ref tagged) => {
                tagged.write(wtr)?;
            }
        }
        Ok(())
    }
}

impl<E, T> FromBytes for MaybeTagged<'static, T>
where
    T: FromBytes<Err = E> + serde::de::DeserializeOwned,
    E: Display + Debug + Send + Sync + 'static,
{
    type Err = anyhow::Error;

    fn from_bytes(s: &[u8]) -> anyhow::Result<MaybeTagged<'static, T>> {
        let probably_json = s.first().map_or(false, |&byte| byte == b'{');
        let mut json_decoding_err = None;
        if probably_json {
            json_decoding_err = Some(match s.parse() {
                Err(err) => err,
                Ok(tagged) => return Ok(MaybeTagged::Tagged(tagged)),
            });
        }
        let raw_err = match s.parse::<T>() {
            Err(err) => err,
            Ok(untagged) => return Ok(MaybeTagged::Untagged(untagged)),
        };

        // We're only here if either JSON decoding failed or parsing as an
        // untagged value failed. Both could have failed. But if we don't
        // show the JSON decoding error and it was actually a JSON tagged
        // value, then the error message resulting from trying to parse an
        // untagged value would be very confusing.
        //
        // In theory this heuristic could be wrong some times. For example,
        // if a raw untagged value begins with a `{` and is invalid. But that
        // should be very rare.
        if probably_json {
            if let Some(err) = json_decoding_err {
                return Err(err);
            }
        }
        Err(anyhow::Error::msg(raw_err))
    }
}

#[derive(Clone, Debug)]
pub struct Tagged<'a, T> {
    tags: Tags<T>,
    data: TextBytes<'a>,
}

impl<'a, T> Tagged<'a, T> {
    pub fn new(data: impl Into<Cow<'a, BStr>>) -> Tagged<'a, T> {
        Tagged { tags: Tags(vec![]), data: TextBytes(data.into()) }
    }

    pub fn tag(mut self, tag: Tag<T>) -> Tagged<'a, T> {
        self.tags.0.push(tag);
        self
    }

    /// Return the actual tags.
    ///
    /// Callers generally shouldn't use this unless they need to examine the
    /// tag's metadata itself. To do operations on tag values, you should use
    /// `Tagged::map` or `Tagged::try_map`.
    pub fn tags(&self) -> &[Tag<T>] {
        &self.tags.0
    }

    pub fn data(&self) -> &BStr {
        &*self.data
    }

    pub fn into_owned(self) -> Tagged<'static, T> {
        Tagged { tags: self.tags, data: self.data.into_owned() }
    }

    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Tagged<'a, U> {
        let Tagged { tags, data } = self;
        let mut tagged = Tagged::new(data);
        for tag in tags.0 {
            tagged = tagged.tag(tag.map(&mut f));
        }
        tagged
    }

    pub fn try_map<U>(
        self,
        mut f: impl FnMut(T) -> anyhow::Result<U>,
    ) -> anyhow::Result<Tagged<'a, U>> {
        let Tagged { tags, data } = self;
        let mut tagged = Tagged::new(data);
        for tag in tags.0 {
            tagged = tagged.tag(tag.try_map(&mut f)?);
        }
        Ok(tagged)
    }

    /// Retain only the tags for which the given predicate returns `true`.
    pub fn retain(&mut self, mut predicate: impl FnMut(&mut T) -> bool) {
        self.tags.0.retain_mut(|tag| predicate(tag.value_mut()));
    }
}

impl<'a, T: Eq> Eq for Tagged<'a, T> {}

impl<'a, T: PartialEq> PartialEq for Tagged<'a, T> {
    fn eq(&self, rhs: &Tagged<'a, T>) -> bool {
        self.tags.eq(&rhs.tags)
    }
}

impl<'a, T: Ord> Ord for Tagged<'a, T> {
    fn cmp(&self, rhs: &Tagged<'a, T>) -> std::cmp::Ordering {
        self.tags.cmp(&rhs.tags)
    }
}

impl<'a, T: PartialOrd> PartialOrd for Tagged<'a, T> {
    fn partial_cmp(&self, rhs: &Tagged<'a, T>) -> Option<std::cmp::Ordering> {
        self.tags.partial_cmp(&rhs.tags)
    }
}

impl<'a, T: std::hash::Hash> std::hash::Hash for Tagged<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tags.hash(state);
    }
}

impl<'a, T: serde::Serialize> Tagged<'a, T> {
    pub fn write(&self, wtr: impl std::io::Write) -> anyhow::Result<()> {
        serde_json::to_writer(wtr, self)?;
        Ok(())
    }
}

impl<T: serde::de::DeserializeOwned> FromBytes for Tagged<'static, T> {
    type Err = anyhow::Error;

    fn from_bytes(s: &[u8]) -> anyhow::Result<Tagged<'static, T>> {
        Ok(serde_json::from_slice(s)?)
    }
}

impl<'a, T: serde::Serialize> serde::Serialize for Tagged<'a, T> {
    fn serialize<S: serde::Serializer>(
        &self,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;

        let len = 1 + if self.tags.0.is_empty() { 0 } else { 1 };
        let mut state = s.serialize_struct("Tagged", len)?;
        if !self.tags.0.is_empty() {
            state.serialize_field("tags", &self.tags)?;
        }
        state.serialize_field("data", &self.data)?;
        state.end()
    }
}

impl<'a, 'de, T: serde::Deserialize<'de>> serde::Deserialize<'de>
    for Tagged<'a, T>
{
    #[inline]
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        use serde::de;

        enum Field {
            Tags,
            Data,
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
                        f.write_str("`tags` or `data`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tags" => Ok(Field::Tags),
                            "data" => Ok(Field::Data),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct Visitor<T>(std::marker::PhantomData<T>);

        impl<'de, T: serde::Deserialize<'de>> serde::de::Visitor<'de> for Visitor<T> {
            type Value = Tagged<'static, T>;

            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                formatter.write_str(
                    "a map with a `data` key and an optional `tags` key",
                )
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut tags = None;
                let mut data = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Tags => {
                            if tags.is_some() {
                                return Err(de::Error::duplicate_field(
                                    "tags",
                                ));
                            }
                            tags = Some(map.next_value()?);
                        }
                        Field::Data => {
                            if data.is_some() {
                                return Err(de::Error::duplicate_field(
                                    "data",
                                ));
                            }
                            data = Some(map.next_value()?);
                        }
                    }
                }
                let tags = tags.map(Tags).unwrap_or_else(|| Tags(vec![]));
                let data =
                    data.ok_or_else(|| de::Error::missing_field("data"))?;
                Ok(Tagged { tags, data })
            }
        }

        const FIELDS: &[&str] = &["tags", "data"];
        deserializer.deserialize_struct(
            "Tagged",
            FIELDS,
            Visitor(std::marker::PhantomData),
        )
    }
}

/// A sequence of tags.
///
/// This is a wrapper type to make the by-hand Serde trait implementations a
/// little more manageable.
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Tags<T>(Vec<Tag<T>>);

impl<T: serde::Serialize> serde::Serialize for Tags<T> {
    fn serialize<S: serde::Serializer>(
        &self,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        s.collect_seq(&self.0)
    }
}

impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for Tags<T> {
    #[inline]
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        struct Visitor<T>(std::marker::PhantomData<T>);

        impl<'de, T: serde::Deserialize<'de>> serde::de::Visitor<'de> for Visitor<T> {
            type Value = Tags<T>;

            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                formatter.write_str("an array of tags")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let mut tags =
                    Vec::with_capacity(seq.size_hint().unwrap_or(0));
                while let Some(tag) = seq.next_element()? {
                    tags.push(tag);
                }
                Ok(Tags(tags))
            }
        }

        deserializer.deserialize_seq(Visitor(std::marker::PhantomData))
    }
}

#[derive(Clone, Debug)]
pub struct Tag<T> {
    value: T,
    range: Option<TagRange>,
}

impl<T> Tag<T> {
    pub fn new(value: T) -> Tag<T> {
        Tag { value, range: None }
    }

    pub fn with_range(self, range: impl Into<TagRange>) -> Tag<T> {
        Tag { range: Some(range.into()), ..self }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn range(&self) -> Option<TagRange> {
        self.range.as_ref().copied()
    }

    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Tag<U> {
        Tag { value: f(self.value), range: self.range }
    }

    pub fn try_map<U>(
        self,
        mut f: impl FnMut(T) -> anyhow::Result<U>,
    ) -> anyhow::Result<Tag<U>> {
        Ok(Tag { value: f(self.value)?, range: self.range })
    }
}

impl<T: Eq> Eq for Tag<T> {}

impl<T: PartialEq> PartialEq for Tag<T> {
    fn eq(&self, rhs: &Tag<T>) -> bool {
        self.value.eq(&rhs.value)
    }
}

impl<T: Ord> Ord for Tag<T> {
    fn cmp(&self, rhs: &Tag<T>) -> std::cmp::Ordering {
        self.value.cmp(&rhs.value)
    }
}

impl<T: PartialOrd> PartialOrd for Tag<T> {
    fn partial_cmp(&self, rhs: &Tag<T>) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&rhs.value)
    }
}

impl<T: std::hash::Hash> std::hash::Hash for Tag<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T: serde::Serialize> serde::Serialize for Tag<T> {
    fn serialize<S: serde::Serializer>(
        &self,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;

        let len = 1 + if self.range.is_some() { 1 } else { 0 };
        let mut state = s.serialize_struct("Tag", len)?;
        state.serialize_field("value", &self.value)?;
        if let Some(ref range) = self.range {
            state.serialize_field("range", range)?;
        } else {
            state.skip_field("range")?;
        }
        state.end()
    }
}

impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for Tag<T> {
    #[inline]
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        use serde::de;

        enum Field {
            Value,
            Range,
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
                        f.write_str("`value` or `range`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value" => Ok(Field::Value),
                            "range" => Ok(Field::Range),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct Visitor<T>(std::marker::PhantomData<T>);

        impl<'de, T: serde::Deserialize<'de>> serde::de::Visitor<'de> for Visitor<T> {
            type Value = Tag<T>;

            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                formatter.write_str(
                    "a map with a `value` key and an optional `range` key",
                )
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                let mut range = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Value => {
                            if value.is_some() {
                                return Err(de::Error::duplicate_field(
                                    "value",
                                ));
                            }
                            value = Some(map.next_value()?);
                        }
                        Field::Range => {
                            if range.is_some() {
                                return Err(de::Error::duplicate_field(
                                    "range",
                                ));
                            }
                            range = Some(map.next_value()?);
                        }
                    }
                }
                let value =
                    value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(Tag { value, range })
            }
        }

        const FIELDS: &[&str] = &["text", "bytes"];
        deserializer.deserialize_struct(
            "Tag",
            FIELDS,
            Visitor(std::marker::PhantomData),
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct TagRange {
    start: usize,
    end: usize,
}

impl TagRange {
    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    pub fn diff(&self, len: usize) -> isize {
        len as isize - (self.end.saturating_sub(self.start) as isize)
    }

    pub fn offset(self, offset: isize) -> TagRange {
        let start = (self.start as isize) + offset;
        let end = (self.end as isize) + offset;
        TagRange { start: start as usize, end: end as usize }
    }
}

impl From<Range<usize>> for TagRange {
    fn from(range: Range<usize>) -> TagRange {
        TagRange { start: range.start, end: range.end }
    }
}

impl serde::Serialize for TagRange {
    fn serialize<S: serde::Serializer>(
        &self,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        s.collect_seq([self.start, self.end])
    }
}

impl<'de> serde::Deserialize<'de> for TagRange {
    #[inline]
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        use serde::de;

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TagRange;

            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                formatter.write_str(
                    "an array of length two containing a byte range",
                )
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let start = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let end = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(TagRange { start, end })
            }
        }

        deserializer.deserialize_seq(Visitor)
    }
}
