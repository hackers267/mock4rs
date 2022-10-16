use crate::base::integer;
use crate::base::integer::random_integer;

/// Generate a random u16;
/// 随机生成一个u16类型的值
///
/// # Example 示例
///
/// ```
/// use mock4rs::base::random_u16_simple;
/// let u = random_u16_simple();
/// println!("u:{}",u);
/// ```
pub fn random_u16_simple() -> u16 {
    integer::random_integer(u16::MIN, u16::MAX)
}

/// Generate a random u16 based on the given maximum value.
/// 根据指定的最大值随机生成一个u16值
///
/// # Example 示例
/// ```
/// use mock4rs::base::random_u16_max;
/// let  u = random_u16_max(120);
/// println!("u:{}", u);
/// ```
pub fn random_u16_max(max: u16) -> u16 {
    random_integer(u16::MIN, max)
}

/// Generates a random u16 based on the given minimum value.
/// 根据指定的最小值随机生成一个u16值
///
/// # Example 示例
/// ```
/// use mock4rs::base::random_u16_min;
/// let  u = random_u16_min(50);
/// println!("u:{}", u);
/// ```
pub fn random_u16_min(min: u16) -> u16 {
    integer::random_integer(min, u16::MAX)
}

/// Generates a random u16 based on the given maximum and minimum value.
/// 根据指定的最大值随机生成一个u16值
///
/// # Example 示例
/// ```
/// use mock4rs::base::random_u8;
/// let  u = random_u8(10,20);
/// println!("u:{}", u);
/// ```
pub fn random_u16(min: u16, max: u16) -> u16 {
    integer::random_integer(min, max)
}
