//! Generates a random color.
//! 随机生成一个颜色值
use crate::base::{random_f32, random_u16_max, random_u16_simple, random_u8_max};

/// Generates a random color.
/// 随机生成一个颜色值
///
/// # Example 示例
/// ```
/// use mock4rs::random_color;
/// let color = random_color();
/// println!("color: {}", color);
/// ```
pub fn random_color() -> String {
    let range = 0..3;
    range
        .map(|_i| random_u16_simple())
        .map(|s| format!("{:02x}", s))
        .fold("#".to_string(), |acc, cur| format!("{}{}", acc, cur))
}

/// Generates a random color which style is hex.
/// 随机生成一个hex形式的颜色值
///
/// # Example 示例
/// ```
/// use mock4rs::random_hex;
/// let color = random_hex();
/// println!("color: {}", color);
/// ```
pub fn random_hex() -> String {
    random_color()
}

/// Generates a random color which style is rgb.
/// 随机生成一个rgb形式的颜色值
///
/// # Example 示例
/// ```
/// use mock4rs::random_rgb;
/// let color = random_rgb();
/// println!("color: {}", color);
/// ```
pub fn random_rgb() -> String {
    let r = random_u16_simple();
    let g = random_u16_simple();
    let b = random_u16_simple();
    format!("rgb({},{},{})", r, g, b)
}

/// Generates a random color which style is rgba.
/// 随机生成一个rgba形式的颜色值
///
/// # Example 示例
/// ```
/// use mock4rs::random_rgba;
/// let color = random_rgba();
/// println!("color: {}", color);
/// ```
pub fn random_rgba() -> String {
    let r = random_u16_simple();
    let g = random_u16_simple();
    let b = random_u16_simple();
    let a = random_f32(0.0, 1.0);
    format!("rgba({},{},{},{:.2})", r, g, b, a)
}

/// Generates a random color which style is hsl.
/// 随机生成一个hsl形式的颜色值
///
/// # Example 示例
/// ```
/// use mock4rs::random_hsl;
/// let color = random_hsl();
/// println!("color: {}", color);
/// ```
pub fn random_hsl() -> String {
    let h = random_u16_max(360);
    let s = random_u8_max(100);
    let l = random_u8_max(100);
    format!("hsl({},{},{})", h, s, l)
}

/// Generates a random color which style is hsla.
/// 随机生成一个hsla形式的颜色值
///
/// # Example 示例
/// ```
/// use mock4rs::random_hsla;
/// let color = random_hsla();
/// println!("color: {}", color);
/// ```
pub fn random_hsla() -> String {
    let h = random_u16_max(360);
    let s = random_u8_max(100);
    let l = random_u8_max(100);
    let a = random_f32(0.0, 1.0);
    format!("hsla({},{},{},{:.2})", h, s, l, a)
}
