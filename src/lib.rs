/*!
 The `mock4rs` library is  a library that provides a set of methods that provides data.

 `mock4rs` 库提供了一系列的生成指定数据的方法。
*/
#![deny(missing_docs)]
#![deny(rustdoc::missing_doc_code_examples)]
extern crate core;

pub mod base;
pub mod char;
mod color;
pub mod date;
mod id_card;
mod phone;
mod pick_one;
mod web;

use crate::pick_one::pick_one;

pub use crate::id_card::random_id_card;
pub use crate::phone::random_phone;
pub use color::{random_color, random_hex, random_hsl, random_hsla, random_rgb, random_rgba};
pub use web::{random_domain, random_email, random_ipv4, random_protocol, random_tld, random_url};
