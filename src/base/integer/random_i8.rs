use crate::base::integer;

/// Generate a random i8;
/// 随机生成一个i8类型的值
///
/// # Example 示例
/// ```
/// use mock4rs::base::random_i8_simple;
/// let  u = random_i8_simple();
/// println!("u:{}", u);
/// ```
pub fn random_i8_simple() -> i8 {
    integer::random_integer(i8::MIN, i8::MAX)
}

/// Generate a random i8 based on the given maximum value.
/// 根据指定的最大值随机生成一个i8值
///
/// # Arguments
///
/// - `max`: 指定的最大值
///
/// returns 随机值
///
/// # Example 示例
/// ```
/// use mock4rs::base::random_i8_max;
/// let i = random_i8_max(100);
/// println!("i: {}", i);
/// ```
pub fn random_i8_max(max: i8) -> i8 {
    integer::random_integer(i8::MIN, max)
}

/// Generate a random i8 based on the given minimum value.
/// 根据指定的最小值随机生成一个i8值
///
/// # Arguments
///
/// - `min`: 指定的最小值
///
/// returns 随机值
///
/// # Example 示例
/// ```
/// use mock4rs::base::random_i8_min;
/// let  i = random_i8_min(100);
/// println!("i:{}",i)
/// ```
pub fn random_i8_min(min: i8) -> i8 {
    integer::random_integer(min, i8::MAX)
}

/// Generate a random i8 based on the given minimum and maximum values.
/// 根据指定的最小值和最大值随机生成一个i8值
///
/// # Arguments
///
/// - `min`: 指定的最小值
/// - `max`: 指定的最大值
///
/// returns 随机值
///
/// # Example 示例
/// ```
/// use mock4rs::base::random_i8;
/// let  i = random_i8(50,100);
/// println!("i:{}",i)
/// ```
pub fn random_i8(min: i8, max: i8) -> i8 {
    integer::random_integer(min, max)
}
