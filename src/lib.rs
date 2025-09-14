// Copyright 2018 Serde Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate serde_core as serde;

mod de;
mod error;
mod ser;

pub use crate::de::{from_str, Deserializer};
pub use crate::error::{Error, Result};
pub use crate::ser::{to_string, Serializer};
