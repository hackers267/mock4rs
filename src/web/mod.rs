//! Generates a random web address
//! 随机生成一个web地址
use crate::base::{random_u8, random_u8_simple};
use crate::char::{random_char_bytes, CharType};
use crate::pick_one;

/// Generates a random url;
/// 随机生成一个url
///
/// # Example 示例
/// ```
/// use mock4rs::web::random_url;
/// let url = random_url();
/// println!("url: {:?}",url);
/// ```
pub fn random_url() -> String {
    let protocol = random_protocol();
    let domain = random_domain();
    let path = random_string_simple();
    format!("{}://{}/{}", protocol, domain, path)
}

fn random_string_simple() -> String {
    let len = random_u8(3, 8);
    let range = 1..=len;
    range
        .map(|_i| random_char_bytes(CharType::Alphanumeric))
        .collect()
}

/// Generate a random domain.
/// 随机生成一个domain
///
/// # Example 示例
/// ```
/// use mock4rs::web::random_domain;
/// let domain = random_domain();
/// println!("domain: {}", domain);
/// ```
pub fn random_domain() -> String {
    let path = random_string_simple();
    let tld = random_tld();
    format!("{}.{}", path, tld)
}

const PROTOCOLS: [&str; 26] = [
    "dhcp", "dns", "ftp", "gopher", "http", "imap4", "irc", "nntp", "xmpp", "pop3", "sip", "smtp",
    "snmp", "ssh", "telnet", "rpc", "rtcp", "rtp", "rtsp", "sdp", "soap", "gtp", "stun", "ntp",
    "ssdp", "bgp",
];

/// Generate a random protocol.
/// 随机生成一个protocol
///
/// # Example 示例
/// ```
/// use mock4rs::web::random_protocol;
/// let protocol = random_protocol();
/// println!("protocol: {}", protocol);
/// ```
pub fn random_protocol() -> String {
    pick_one(&PROTOCOLS).to_string()
}

const TLDS: [&str; 90] = [
    "com",
    "cn",
    "top",
    "xyz",
    "net",
    "ltd",
    "vip",
    "shop",
    "cc",
    "store",
    "online",
    "fun",
    "tech",
    "art",
    "site",
    "co",
    "icu",
    "club",
    "work",
    "xin",
    "wang",
    "space",
    "group",
    "ink",
    "pub",
    "info",
    "ren",
    "live",
    "link",
    "cloud",
    "com.cn",
    "我爱你",
    "中国",
    "网址",
    "website",
    "pro",
    "life",
    "asia",
    "biz",
    "cool",
    "mobi",
    "fit",
    "公司",
    "网络",
    "plus",
    "press",
    "wiki",
    "love",
    "red",
    "design",
    "video",
    "run",
    "show",
    "zone",
    "kim",
    "city",
    "gold",
    "today",
    "host",
    "team",
    "chat",
    "fund",
    "beer",
    "center",
    "company",
    "email",
    "yoga",
    "luxe",
    "net.cn",
    "org.cn",
    "world",
    "fans",
    "guru",
    "在线",
    "商店",
    "企业",
    "集团",
    "招聘",
    "网店",
    "商城",
    "中文网",
    "佛山",
    "广东",
    "商标",
    "游戏",
    "娱乐",
    "餐厅",
    "law",
    "social",
    "gov.cn",
];

/// Generate a random tld.
/// 随机生成一个tld
///
/// # Example 示例
/// ```
/// use mock4rs::web::random_tld;
/// let tld = random_tld();
/// println!("tld: {}", tld);
/// ```
pub fn random_tld() -> String {
    pick_one(TLDS.as_slice()).to_string()
}

/// Generate a random email.
/// 随机生成一个email
///
/// # Example 示例
/// ```
/// use mock4rs::web::random_email;
/// let email = random_email();
/// println!("email: {}", email);
/// ```
pub fn random_email() -> String {
    let name = random_string_simple();
    let domain = random_domain();
    format!("{}@{}", name, domain)
}

/// Returns a random ipv4
/// 随机生成一个ipv4地址
///
/// # Example 示例
/// ```
/// use mock4rs::web::random_ipv4;
/// let ipv4 = random_ipv4();
/// println!("ipv4: {}", ipv4);
/// ```
pub fn random_ipv4() -> String {
    let range = 0..=3;
    range
        .map(|_i| random_u8_simple())
        .map(|i| i.to_string())
        .reduce(|acc, cur| format!("{}.{}", acc, cur))
        .unwrap()
}
