// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(feature = "with-bench", feature(test))]

extern crate rand;
extern crate hex;

pub mod digest;
pub mod sha2;
pub mod cryptoutil;
pub mod buffer;
pub mod symmetriccipher;
pub mod simd;
