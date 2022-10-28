use crate::data::DICT;
use crate::pick_one;

fn get_provinces() -> Vec<(&'static str, &'static str)> {
    DICT.into_iter()
        .filter(|(_address, code)| code.ends_with("0000"))
        .collect::<Vec<(&str, &str)>>()
}

fn get_cities() -> Vec<(&'static str, &'static str)> {
    DICT.into_iter()
        .filter(|(_address, code)| !code.ends_with("0000") && code.ends_with("00"))
        .collect::<Vec<_>>()
}

fn get_counties() -> Vec<(&'static str, &'static str)> {
    DICT.into_iter()
        .filter(|(_address, code)| !code.ends_with("00"))
        .collect::<_>()
}

/// Generate a random province.
/// 随机生成一个省。
///
/// # Example 示例
/// ```
/// use mock4rs::address::random_province;
/// let  province = random_province();
/// println!("province: {}", province);
/// ```
pub fn random_province() -> &'static str {
    let provinces = get_provinces();
    pick_one(&provinces).1
}

/// Generate a random city.
/// 随机生成一个市。
///
/// # Example 示例
/// ```
/// use mock4rs::address::random_city;
/// let city = random_city();
/// println!("city: {}", city);
/// ```
pub fn random_city() -> &'static str {
    let cities = get_cities();
    pick_one(&cities).1
}

/// Generate a random county.
/// 随机生成一个县。
///
/// # Example 示例
/// ```
/// use mock4rs::address::random_county;
/// let county = random_county();
/// println!("county: {}", county);
/// ```
pub fn random_county() -> &'static str {
    let counties = get_counties();
    pick_one(&counties).1
}
