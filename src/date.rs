/*!
To generate a random date.
随机生成日期和时间
 */
use rand::Rng;

fn random_year() -> String {
    let mut rng = rand::thread_rng();
    let range = 1970..2099;
    rng.gen_range(range).to_string()
}

fn random_month() -> String {
    let range = 1..=12;
    let mut rng = rand::thread_rng();
    rng.gen_range(range).to_string()
}

fn is_leap_year(year: u16) -> bool {
    if year % 400 == 0 {
        return true;
    }
    if year % 4 == 0 {
        return true;
    }
    false
}

fn random_day(date: Option<(u16, u8)>) -> String {
    let end = match date {
        Some(date) => {
            let (year, month) = date;
            let day31 = [1, 3, 5, 7, 8, 10, 12];
            let day30 = [2, 4, 6, 11];
            if day31.contains(&month) {
                31
            } else if day30.contains(&month) {
                30
            } else if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        None => 31,
    };
    let range = 1..=end;
    let mut rng = rand::thread_rng();
    rng.gen_range(range).to_string()
}

/// DateType
/// 日期类型
pub enum DateType {
    /// 年
    Year,
    /// 月
    Month,
    /// 日
    Day,
    /// 年月
    YearMonth,
    /// 月日
    MonthDay,
    /// 年月日
    Date,
}
pub fn random_date_simple(date_type: DateType) -> String {
    random_date_sep(date_type, None)
}

fn random_date_sep(date_type: DateType, sep: Option<&str>) -> String {
    let sep = sep.unwrap_or("");
    match date_type {
        DateType::Year => random_year(),
        DateType::Month => random_month(),
        DateType::Day => random_day(None),
        DateType::YearMonth => {
            let year = random_year();
            let month = random_month();
            format!("{}{sep}{:0>2}", year, month)
        }
        DateType::MonthDay => {
            let year = random_year().parse().unwrap();
            let month = random_month().parse().unwrap();
            let day = random_day(Some((year, month)));
            format!("{:0>2}{sep}{:0>2}", month, day)
        }
        DateType::Date => {
            let year = random_year().parse().unwrap();
            let month = random_month().parse().unwrap();
            let day = random_day(Some((year, month)));
            format!("{}{sep}{:0>2}{sep}{:0>2}", year, month, day)
        }
    }
}

/// Generate a random date by type.
/// 根据类型生成一个随机日期
///
/// # Examples 示例
/// ```
/// use mock4rs::date::{DateType, random_date};
/// let d = random_date(DateType::Date);
/// println!("{}", d);
/// ```
pub fn random_date(date_type: DateType) -> String {
    random_date_sep(date_type, Some("-"))
}

/**
 * TimeTye 时间类型
 *
 * - Hour 时
 * - Minute 分
 * - Second 秒
 * - HourMinute 时分
 * - MinuteSecond 分秒
 * - Time 时分秒
 */
pub enum TimeType {
    /// 时
    Hour,
    /// 分
    Minute,
    /// 秒
    Second,
    /// 时分
    HourMinute,
    /// 分秒
    MinuteSecond,
    /// 时分秒
    Time,
}

/// Generate a random time by type.
/// 根据类型生成一个随机时间
///
/// # Examples 示例
///
/// ```
/// use mock4rs::date::{random_time, TimeType};
/// let t = random_time(TimeType::Time);
/// println!("{}",t);
/// ```
pub fn random_time(time_type: TimeType) -> String {
    match time_type {
        TimeType::Hour => random_hour().to_string(),
        TimeType::Minute => random_minute().to_string(),
        TimeType::Second => random_second().to_string(),
        TimeType::HourMinute => {
            format!("{:0>2}:{:0>2}", random_hour(), random_minute())
        }
        TimeType::MinuteSecond => {
            format!("{:0>2}:{:0>2}", random_minute(), random_second())
        }
        TimeType::Time => {
            format!(
                "{:0>2}:{:0>2}:{:0>2}",
                random_hour(),
                random_minute(),
                random_second()
            )
        }
    }
}

fn random_integer_range<T>(min: T, max: T) -> T
where
    T: PartialOrd + rand::distributions::uniform::SampleUniform,
{
    rand::thread_rng().gen_range(min..=max)
}

fn random_hour() -> u8 {
    random_integer_range(0, 24)
}

fn random_minute() -> u8 {
    random_integer_range(0, 60)
}

fn random_second() -> u8 {
    random_integer_range(0, 60)
}
