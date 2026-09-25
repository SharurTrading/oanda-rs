//! Distinguishes omitted fields from explicit JSON null in OANDA mutations.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A mutation field that may be left unchanged, cleared, or set.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Patch<T> {
    /// Omit the field from the request.
    #[default]
    Unchanged,
    /// Send JSON null to clear the existing value.
    Clear,
    /// Send a new value.
    Set(T),
}

impl<T> Patch<T> {
    /// Whether the field should be omitted when serialized as a struct field.
    pub fn is_unchanged(&self) -> bool {
        matches!(self, Self::Unchanged)
    }
}

impl<T: Serialize> Serialize for Patch<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Unchanged | Self::Clear => serializer.serialize_none(),
            Self::Set(value) => value.serialize(serializer),
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Patch<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Option::<T>::deserialize(deserializer).map(|value| match value {
            Some(value) => Self::Set(value),
            None => Self::Clear,
        })
    }
}
