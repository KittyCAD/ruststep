use serde::{Deserialize, Serialize};

/// Binary literal.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Binary {
    /// Four bit sequence.
    pub nibbles: Vec<u8>,
}

impl Binary {
    /// Encode as per §6.4.6.
    pub fn encode(&self) -> String {
        let prefix = 4 - (self.nibbles.len() % 4);
        let mut encoding = format!("{prefix}");
        for nibble in &self.nibbles {
            encoding += &format!("{nibble}");
        }
        format!("\"{encoding}\"")
    }
}

impl std::fmt::Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.pad(&self.encode())
    }
}

impl<'de> Deserialize<'de> for Binary {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Binary;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "BINARY literal")
            }

            fn visit_str<E>(self, _: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                unimplemented!()
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}
