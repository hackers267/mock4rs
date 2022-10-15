/*!
To generate a random date.
随机生成日期和时间
 */
use rand::Rng;

fn year() -> String {
    let mut rng = rand::thread_rng();
    let range = 1970..2099;
    rng.gen_range(range).to_string()
}

fn month() -> String {
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

fn day(date: Option<(u16, u8)>) -> String {
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
/// Generate a random date by type.
/// 根据类型生成一个随机日期
///
/// # Examples 示例
/// ```
/// use mock::date::{date, DateType};
/// let d = date(DateType::Date);
/// println!("{}", d);
/// ```
pub fn date(date_type: DateType) -> String {
    match date_type {
        DateType::Year => year(),
        DateType::Month => month(),
        DateType::Day => day(None),
        DateType::YearMonth => {
            let year = year();
            let month = month();
            format!("{}-{:0>2}", year, month)
        }
        DateType::MonthDay => {
            let year = year().parse().unwrap();
            let month = month().parse().unwrap();
            let day = day(Some((year, month)));
            format!("{:0>2}-{:0>2}", month, day)
        }
        DateType::Date => {
            let year = year().parse().unwrap();
            let month = month().parse().unwrap();
            let day = day(Some((year, month)));
            format!("{}-{:0>2}-{:0>2}", year, month, day)
        }
    }
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
/// use mock::date::{time, TimeType};
/// let t = time(TimeType::Time);
/// println!("{}",t);
/// ```
pub fn time(time_type: TimeType) -> String {
    match time_type {
        TimeType::Hour => hour().to_string(),
        TimeType::Minute => minute().to_string(),
        TimeType::Second => second().to_string(),
        TimeType::HourMinute => {
            format!("{:0>2}:{:0>2}", hour(), minute())
        }
        TimeType::MinuteSecond => {
            format!("{:0>2}:{:0>2}", minute(), second())
        }
        TimeType::Time => {
            format!("{:0>2}:{:0>2}:{:0>2}", hour(), minute(), second())
        }
    }
}

fn random_integer_range<T>(min: T, max: T) -> T
where
    T: PartialOrd + rand::distributions::uniform::SampleUniform,
{
    rand::thread_rng().gen_range(min..=max)
}

fn hour() -> u8 {
    random_integer_range(0, 24)
}

fn minute() -> u8 {
    random_integer_range(0, 60)
}

fn second() -> u8 {
    random_integer_range(0, 60)
}
