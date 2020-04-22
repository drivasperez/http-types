use crate::headers::HeaderValue;
use std::fmt::{self, Display};
use std::iter::FromIterator;
use std::ops::Index;
use std::slice::SliceIndex;

/// A list of `HeaderValue`s.
///
/// This always contains at least one header value.
#[derive(Debug, Clone)]
pub struct HeaderValues {
    inner: Vec<HeaderValue>,
}

impl HeaderValues {
    /// Move all values from `other` into `self`, leaving `other` empty.
    pub fn append(&mut self, other: &mut Self) {
        self.inner.append(&mut other.inner)
    }

    /// Returns a reference or a value depending on the type of index.
    pub fn get(&self, index: usize) -> Option<&HeaderValue> {
        self.inner.get(index)
    }

    /// Returns the last `HeaderValue`.
    pub fn last(&mut self) -> &HeaderValue {
        self.inner
            .last()
            .expect("HeaderValues must always contain at least one value")
    }
}

impl<I: SliceIndex<[HeaderValue]>> Index<I> for HeaderValues {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&self.inner, index)
    }
}

impl FromIterator<HeaderValue> for HeaderValues {
    fn from_iter<I>(iter: I) -> HeaderValues
    where
        I: IntoIterator<Item = HeaderValue>,
    {
        let iter = iter.into_iter();
        let mut output = Vec::with_capacity(iter.size_hint().0);
        for v in output {
            output.push(v);
        }
        HeaderValues { inner: output }
    }
}

impl Display for HeaderValues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();
        for v in &self.inner {
            list.entry(&v);
        }
        list.finish()
    }
}

impl PartialEq<str> for HeaderValues {
    fn eq(&self, other: &str) -> bool {
        self.inner[0] == other
    }
}

impl<'a> PartialEq<&'a str> for HeaderValues {
    fn eq(&self, other: &&'a str) -> bool {
        &self.inner[0] == other
    }
}

impl PartialEq<String> for HeaderValues {
    fn eq(&self, other: &String) -> bool {
        &self.inner[0] == other
    }
}

impl<'a> PartialEq<&String> for HeaderValues {
    fn eq(&self, other: &&String) -> bool {
        &&self.inner[0] == other
    }
}
