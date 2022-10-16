use crate::base::integer;

/// Generate a random i32;
/// 随机生成一个i32类型的值
///
/// ```
/// use mock4rs::base::random_i32_simple;
/// let  i = random_i32_simple();
/// println!("i:{}", i);
/// ```
pub fn random_i32_simple() -> i32 {
    integer::random_integer(i32::MIN, i32::MAX)
}

/// Generate a random i32 based on the given maximum value.
/// 根据指定的最大值随机生成一个i32值
///
/// # Arguments
///
/// - `max`: 指定的最大值
///
/// returns 随机值
///
/// # Example 示例
/// ```
/// use mock4rs::base::random_i32_max;
/// let  i = random_i32_max(100);
/// println!("i: {}", i);
/// ```
pub fn random_i32_max(max: i32) -> i32 {
    integer::random_integer(i32::MIN, max)
}

/// Generate a random i32 based on the given minimum value.
/// 根据指定的最小值随机生成一个i32值
///
/// # Arguments
///
/// - `min`: 指定的最小值
///
/// returns 随机值
///
/// # Example 示例
/// ```
/// use mock4rs::base::random_i32_min;
/// let  i = random_i32_min(100);
/// println!("i: {}", i);
/// ```
pub fn random_i32_min(min: i32) -> i32 {
    integer::random_integer(min, i32::MAX)
}

/// Generate a random i32 based on the given minimum and maximum value.
/// 根据指定的最小值和最大值随机生成一个i32值
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
/// use mock4rs::base::random_i32;
/// let  i = random_i32(100,200);
/// println!("i: {}", i);
/// ```

pub fn random_i32(min: i32, max: i32) -> i32 {
    integer::random_integer(min, max)
}
