// Copyright 2018 Serde Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate serde;

// The serde_derive crate provides the macros for #[derive(Serialize)] and
// #[derive(Deserialize)]. You won't need these for implementing a data format
// but your unit tests will probably use them - hence #[cfg(test)].
#[cfg(test)]
#[macro_use]
extern crate serde_derive;

mod de;
mod error;
mod ser;

pub use de::{from_str, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_string, Serializer};
