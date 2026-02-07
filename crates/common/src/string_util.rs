use lasso::{Spur, ThreadedRodeo};
use serde::de::{self, Deserialize, Deserializer, Visitor};
use std::{
    fmt,
    sync::{LazyLock, OnceLock},
};

static STRING_POOL: LazyLock<ThreadedRodeo> = LazyLock::new(|| ThreadedRodeo::new());
static EMPTY_SPUR: OnceLock<Spur> = OnceLock::new();

pub fn empty_spur() -> Spur {
    *EMPTY_SPUR.get_or_init(|| STRING_POOL.get_or_intern(""))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InternString(pub Spur);

impl<'de> Deserialize<'de> for InternString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StrVisitor;
        impl<'de> Visitor<'de> for StrVisitor {
            type Value = InternString;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a string to intern")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(InternString(STRING_POOL.get_or_intern(v)))
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(InternString(STRING_POOL.get_or_intern(v)))
            }
        }
        deserializer.deserialize_str(StrVisitor)
    }
}

impl Default for InternString {
    fn default() -> Self {
        InternString(empty_spur())
    }
}

impl InternString {
    pub fn is_empty(&self) -> bool {
        self.as_str().is_empty()
    }
}
impl InternString {
    pub fn as_str(&self) -> &str {
        STRING_POOL.resolve(&self.0)
    }
}

impl PartialEq<str> for InternString {
    fn eq(&self, other: &str) -> bool {
        STRING_POOL.resolve(&self.0) == other
    }
}

impl PartialEq<&str> for InternString {
    fn eq(&self, other: &&str) -> bool {
        STRING_POOL.resolve(&self.0) == *other
    }
}

impl PartialEq<String> for InternString {
    fn eq(&self, other: &String) -> bool {
        STRING_POOL.resolve(&self.0) == other.as_str()
    }
}

impl From<&str> for InternString {
    fn from(s: &str) -> Self {
        InternString(STRING_POOL.get_or_intern(s))
    }
}

impl From<String> for InternString {
    fn from(s: String) -> Self {
        InternString(STRING_POOL.get_or_intern(s))
    }
}

impl fmt::Display for InternString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", STRING_POOL.resolve(&self.0))
    }
}

pub trait InternCheck {
    fn is_interned(&self) -> bool;
}

impl InternCheck for &str {
    fn is_interned(&self) -> bool {
        STRING_POOL.get(self).is_some()
    }
}

impl InternCheck for String {
    fn is_interned(&self) -> bool {
        STRING_POOL.get(self.as_str()).is_some()
    }
}

pub fn get_string_hash(s: &str) -> u32 {
    s.chars().fold(0, |hash, c| {
        (((c as u64) + 131 * hash as u64) & 0xFFFFFFFF) as u32
    })
}
