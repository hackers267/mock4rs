use crate::base::integer;

/// Generate a random u8;
/// 随机生成一个u8类型的值
///
/// # Examples 示例
///
/// ```
/// use mock::base::random_u8_simple;
/// let u = random_u8_simple();
/// println!("u: {}",u);
/// ```
pub fn random_u8_simple() -> u8 {
    random_u8(u8::MIN, u8::MAX)
}

/// Generate a random be given the minimum value.
/// 根据指定的最小值随机生成一个u8值
///
/// # Example 示例
///
/// ```
/// use mock::base::random_u8_min;
/// let u = random_u8_min(0);
/// println!("u : {}",u);
/// ```
pub fn random_u8_min(min: u8) -> u8 {
    random_u8(min, u8::MAX)
}

/// Generate a random u8 based on the given maximum value.
/// 根据指定的最大值随机生成一个u8值
///
/// # Example 示例
///
/// ```
/// use mock::base::random_u8_max;
/// let u = random_u8_max(64);
/// println!("u:{}",u);
/// ```
pub fn random_u8_max(max: u8) -> u8 {
    random_u8(u8::MIN, max)
}

/// Generate a random u8 based on the given min and max values.
/// 根据指定的最小值和最大值生成一个u8值
pub fn random_u8(min: u8, max: u8) -> u8 {
    integer::random_integer(min, max)
}
