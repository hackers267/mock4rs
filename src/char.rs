/*!
char crate is a set of methods that are used to generate a character string.

`char` 模块提供了一系列的方法来生成不同类型字符。
*/
use crate::pick_one;

/// Get the random number character.
/// 从0-9中随机获取一个数字
///
/// # Arguments
///
/// returns String number string representation 数字中的字符串表示
///
/// # Examples 示例
///
/// ```
/// use mock4rs::char::random_number;
/// let n = random_number();
/// let range = 0..=9;
/// let list:Vec<String> = range.map(|x|x.to_string()).collect();
/// assert!(list.contains(&n));
/// ```
///
pub fn random_number() -> String {
    random_char_bytes(CharType::Number).to_string()
}

/// Get the random lower letter character.
/// 随机生成一个小写字符
///
/// # Example 示例
/// ```
/// use mock4rs::char::random_lower_char;
/// let  lower_char = random_lower_char();
/// println!("lower_char: {}", lower_char)
/// ```
pub fn random_lower_char() -> String {
    random_char_bytes(CharType::Lower).to_string()
}

/// Get the random letter upper character.
/// 随机生成一个大写字符
///
/// # Example 示例
/// ```
/// use mock4rs::char::random_upper_char;
/// let  upper_char = random_upper_char();
/// println!("upper_char: {}", upper_char)
/// ```
pub fn random_upper_char() -> String {
    random_char_bytes(CharType::Upper).to_string()
}

/// Get the random alpha letter character.
/// 随机生成一个大小写字符
///
/// # Example 示例
/// ```
/// use mock4rs::char::random_alpha_char;
/// let  alpha_char = random_alpha_char();
/// println!("alpha_char: {}", alpha_char)
/// ```
pub fn random_alpha_char() -> String {
    random_char_bytes(CharType::Alpha).to_string()
}

/// Get the random alphanumeric character.
/// 随机生成一个大小写字符或数字字符
///
/// # Example 示例
/// ```
/// use mock4rs::char::random_alphanumeric_char;
/// let  alphanumeric_char = random_alphanumeric_char();
/// print!("alphanumeric_char: {}", alphanumeric_char);
/// ```
pub fn random_alphanumeric_char() -> String {
    random_char_bytes(CharType::Alphanumeric).to_string()
}

/// Get the random character.
/// 随机生成一个字符
///
/// # Example 示例
/// ```
/// use mock4rs::char::random_char;
/// let  string = random_char();
/// println!("string: {}", string);
/// ```
pub fn random_char() -> String {
    random_char_bytes(CharType::All).to_string()
}

/// Get the random character based on the given string.
/// 从指定的字符切片中随机取出一个字符
///
/// # Arguments
///
/// - `str`: 指定的字符切片
///
/// returns String
///
/// # Example 示例
/// ```
/// use mock4rs::char::random_char_with;
/// let  string = random_char_with("hello");
/// println!("string: {}", string);
/// ```
pub fn random_char_with(str: &str) -> String {
    let list = str.chars().collect::<Vec<char>>();
    pick_one(&list).to_string()
}

/// The type of Char.
///
/// - Lower 小写字符
/// - Upper 大写字符
/// - Number 数字字符
/// - Symbol 特殊符号
/// - Alpha 大小写字符
/// - Alphanumeric 大小写字符和数字
/// - All 大小写字符，数字和特殊字符
#[derive(Clone, Copy)]
pub enum CharType {
    /// 小写字符
    Lower,
    /// 大写字符
    Upper,
    /// 数字字符
    Number,
    /// 特殊符号
    Symbol,
    /// 大小写字符，数字和特殊字符
    All,
    /// 大小写字符
    Alpha,
    /// 大小写字符和数字
    Alphanumeric,
}

/// Get a random character by a given byte.
/// 根据指定的类型获取一个随机的字符
/// # Arguments
///
/// * `types`: 类型 `CharType`
///
/// returns: char 字符
pub fn random_char_bytes(types: CharType) -> char {
    let lower = "abcdefghijklmnopqrstuvwxyz";
    let upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let number = "0123456789";
    let symbols = "!@#$%^&*()_+[]{}";
    let alpha = format!("{}{}", lower, upper);
    let alpha_numbers = format!("{}{}", alpha, number);
    let alpha = alpha.as_str();
    let alpha_numbers = alpha_numbers.as_str();
    let all = format!("{}{}{}{}", lower, upper, number, symbols);
    let all = all.as_str();
    let ranges = match types {
        CharType::Lower => lower,
        CharType::Upper => upper,
        CharType::Number => number,
        CharType::Symbol => symbols,
        CharType::Alpha => alpha,
        CharType::Alphanumeric => alpha_numbers,
        CharType::All => all,
    };
    let ranges = ranges.chars().collect::<Vec<char>>();
    pick_one(&ranges)
}
