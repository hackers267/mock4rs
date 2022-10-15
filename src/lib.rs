/*!
 The `mock` library is  a library that provides a set of methods that provides data.

 `mock` 库提供了一系列的生成指定数据的方法。
*/
pub mod char;
pub mod date;
mod phone;
mod pick_one;

use crate::pick_one::pick_one;

pub use crate::phone::phone;
