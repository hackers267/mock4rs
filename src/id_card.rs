use crate::base::random_string_by_len;
use crate::char::CharType;
use crate::data::DICT;
use crate::date::{random_date_simple, DateType};
use crate::pick_one::pick_one;

///
///  A random string generator a id_card;
///  生成一个随机的身份证
///  
/// # Example 示例
///
/// ```
///  use mock4rs::random_id_card;
///  let id_card = random_id_card();
///  println!("id_card: {}", id_card);
///  ```
pub fn random_id_card() -> String {
    let prefix = get_prefix();
    let date = random_date_simple(DateType::Date);
    let third = random_string_by_len(CharType::Number, 3);
    let other = format!("{}{}{}", prefix, date, third);
    let last = calc_code(&other);
    format!("{}{}{}{}", prefix, date, third, last)
}

/// Random pick a code.
/// 随机获取一个行政代码
fn get_prefix() -> &'static str {
    let list = DICT
        .into_iter()
        .map(|(code, _address)| code)
        .collect::<Vec<&str>>();
    pick_one(&list)
}

/// 计算身份证的校验位
fn calc_code(str: &str) -> char {
    let left = str
        .split("")
        .filter(|x| x != &"")
        .map(|x| x.parse::<u16>().unwrap());
    let right = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2].iter();
    let sum = |a: u16, b: u16| a + b;
    let multiplier = |(a, b)| a * b;
    let total = left.zip(right).map(multiplier).reduce(sum);
    let x1 = total.unwrap() % 11;
    let list = ['1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2'];
    list[x1 as usize]
}
