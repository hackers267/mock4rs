/*!
 The `mock4rs` library is  a library that provides a set of methods that provides data.

 `mock4rs` 库提供了一系列的生成指定数据的方法。
*/
#![deny(missing_docs)]
#![deny(rustdoc::missing_doc_code_examples)]
#![doc(html_no_source)]
extern crate core;

pub mod address;
pub mod base;
pub mod char;
pub mod color;
mod data;
pub mod date;
mod id_card;
pub mod name;
mod phone;
mod pick_one;
pub mod web;

use crate::pick_one::pick_one;

pub use crate::id_card::random_id_card;
pub use crate::phone::random_phone;
