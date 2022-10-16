use rand::distributions::uniform::SampleUniform;
use rand::Rng;

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
    random_integer(min, max)
}
/// Generate a random u16;
/// 随机生成一个u16类型的值
///
/// # Example 示例
///
/// ```
/// use mock::base::random_u16_simple;
/// let u = random_u16_simple();
/// println!("u:{}",u);
/// ```
pub fn random_u16_simple() -> u16 {
    random_integer(u16::MIN, u16::MAX)
}

/// Generate a random u16 based on the given maximum value.
/// 根据指定的最大值随机生成一个u16值
///
/// # Example 示例
/// ```
/// use mock::base::random_u16_max;
/// let  u = random_u16_max(120);
/// println!("u:{}", u);
/// ```
pub fn random_u16_max(max: u16) -> u16 {
    random_u16(u16::MIN, max)
}

/// Generates a random u16 based on the given minimum value.
/// 根据指定的最小值随机生成一个u16值
///
/// # Example 示例
/// ```
/// use mock::base::random_u16_min;
/// let  u = random_u16_min(50);
/// println!("u:{}", u);
/// ```
pub fn random_u16_min(min: u16) -> u16 {
    random_integer(min, u16::MAX)
}

/// Generates a random u16 based on the given maximum and minimum value.
/// 根据指定的最大值随机生成一个u16值
///
/// # Example 示例
/// ```
/// use mock::base::random_u8;
/// let  u = random_u8(10,20);
/// println!("u:{}", u);
/// ```
pub fn random_u16(min: u16, max: u16) -> u16 {
    random_integer(min, max)
}

/// Generate a random u32;
/// 随机生成一个u32类型的值
///
/// # Example 示例
/// ```
/// use mock::base::random_u32_simple;
/// let u = random_u32_simple();
/// println!("u:{}",u);
/// ```
pub fn random_u32_simple() -> u32 {
    random_integer(u32::MIN, u32::MAX)
}

/// Generate a random u64;
/// 随机生成一个u64类型的值
///
/// # Example 示例
/// ```
/// use mock::base::random_u64_simple;
/// let u = random_u64_simple();
/// println!("u:{}",u);
/// ```
pub fn random_u64_simple() -> u64 {
    random_integer(u64::MIN, u64::MAX)
}

/// Generate a random u128;
/// 随机生成一个u128类型的值
///
/// # Example 示例
/// ```
/// use mock::base::random_u128_simple;
/// let u = random_u128_simple();
/// println!("u:{}",u);
/// ```
pub fn random_u128_simple() -> u128 {
    random_integer(u128::MIN, u128::MAX)
}

/// Generate a random i8;
/// 随机生成一个i8类型的值
///
/// # Example 示例
/// ```
/// use mock::base::random_i8_simple;
/// let  u = random_i8_simple();
/// println!("u:{}", u);
/// ```
pub fn random_i8_simple() -> i8 {
    random_integer(i8::MIN, i8::MAX)
}

/// Generate a random i16;
/// 随机生成一个i16类型的值
///
/// # Example 示例
/// ```
/// use mock::base::random_i16_simple;
/// let  i = random_i16_simple();
/// println!("i:{}", i);
/// ```
pub fn random_i16_simple() -> i16 {
    random_integer(i16::MIN, i16::MAX)
}

/// Generate a random i32;
/// 随机生成一个i32类型的值
///
/// ```
/// use mock::base::random_i32_simple;
/// let  i = random_i32_simple();
/// println!("i:{}", i);
/// ```
pub fn random_i32_simple() -> i32 {
    random_integer(i32::MIN, i32::MAX)
}

/// Generate a random i64;
/// 随机生成一个i64类型的值
///
/// # Example 示例
/// ```
/// use mock::base::random_i64_simple;
/// let  i = random_i64_simple();
/// println!("i:{}", i);
/// ```
pub fn random_i64_simple() -> i64 {
    random_integer(i64::MIN, i64::MAX)
}

/// Generate a random i128;
/// 随机生成一个i128类型的值
///
/// # Example 示例
/// ```
/// use mock::base::random_i128_simple;
/// let  i = random_i128_simple();
/// println!("{}", i);
/// ```
pub fn random_i128_simple() -> i128 {
    random_integer(i128::MIN, i128::MAX)
}

fn random_integer<T>(min: T, max: T) -> T
where
    T: PartialOrd + SampleUniform,
{
    let range = min..=max;
    let mut rng = rand::thread_rng();
    rng.gen_range(range)
}
