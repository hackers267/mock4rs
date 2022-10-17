/*!
 The `mock4rs` library is  a library that provides a set of methods that provides data.

 `mock4rs` 库提供了一系列的生成指定数据的方法。
*/
#![warn(missing_docs)]
pub mod base;
pub mod char;
pub mod date;
mod id_card;
mod phone;
mod pick_one;

use crate::pick_one::pick_one;

pub use crate::id_card::random_id_card;
pub use crate::phone::phone;
