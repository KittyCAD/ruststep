use serde::{Deserialize, Serialize};

/// A derived value (`*`) is inferred from other properties.
///
/// It is up to the caller to implement instantiation of derived
/// members. The details can be found in the revelant schema.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Derived<T: ?Sized>(std::marker::PhantomData<fn() -> T>);

impl<T: ?Sized> Default for Derived<T> {
    fn default() -> Self {
        Derived(std::marker::PhantomData {})
    }
}

impl<T: ?Sized> std::fmt::Display for Derived<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.pad("*")
    }
}

impl<'de, T: ?Sized> serde::de::Visitor<'de> for Derived<T> {
    type Value = Self;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "'*'")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Default::default())
    }

    fn visit_some<D>(self, _deserializer: D) -> Result<Self::Value, D::Error>
    where
	D: serde::de::Deserializer<'de>,
    {
	// Ignore any provided value for derived attributes.
	Ok(Default::default())
    }
}

impl<'de, T: ?Sized> Deserialize<'de> for Derived<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_option(Self(std::marker::PhantomData {}))
    }
}
