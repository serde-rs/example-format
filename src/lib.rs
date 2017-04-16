extern crate serde;

// The serde_derive crate provides the macros for #[derive(Serialize)] and
// #[derive(Deserialize)]. You won't need these for implementing a data format
// but your unit tests will probably use them - hence #[cfg(test)].
#[cfg(test)]
#[macro_use]
extern crate serde_derive;

mod error;
mod ser;
mod de;

pub use error::{Error, Result};
pub use ser::{to_string, Serializer};
pub use de::{from_str, Deserializer};
