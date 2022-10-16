use crate::base::integer;

/// Generate a random u32;
/// 随机生成一个u32类型的值
///
/// # Example 示例
/// ```
/// use mock4rs::base::random_u32_simple;
/// let u = random_u32_simple();
/// println!("u:{}",u);
/// ```
pub fn random_u32_simple() -> u32 {
    integer::random_integer(u32::MIN, u32::MAX)
}

/// Generate a random u32 based on the given maximum value.
/// 根据指定的最大值随机生成一个u32值
/// # Arguments
///
/// * `max`: 指定最大值
///
/// returns: u32 随机值
///
/// # Examples
///
/// ```
/// use mock4rs::base::random_u32_max;
/// let u = random_u32_max(30);
/// println!("u:{}",u);
/// ```
pub fn random_u32_max(max: u32) -> u32 {
    integer::random_integer(u32::MIN, max)
}

/// Generates a random u32 based on the given minimum value.
/// 根据指定的最小值随机生成一个u32值
/// # Arguments
///
/// * `min`: 指定的最小值
///
/// returns: u32 随机值
///
/// # Examples
///
/// ```
/// use mock4rs::base::random_u32_min;
/// let u = random_u32_min(5);
/// println!("u:{}",u);
/// ```
pub fn random_u32_min(min: u32) -> u32 {
    integer::random_integer(min, u32::MAX)
}

/// Generate a random u32 based on the given maximum and minimum value.
/// 根据指定的最大值和最小值随机生成一个u32值
///
/// # Arguments
///
/// * `min`: 指定的最小值
/// * `max`: 指定的最大值
///
/// returns: u32
///
/// # Examples
///
/// ```
/// use mock4rs::base::random_u32;
/// let u = random_u32(10,50);
/// println!("u:{}", u);
/// ```
pub fn random_u32(min: u32, max: u32) -> u32 {
    integer::random_integer(min, max)
}
