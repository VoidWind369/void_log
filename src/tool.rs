use std::fmt::Display;
use std::ops::Add;
use std::time::{Duration, SystemTime};

#[macro_export]
macro_rules! log {
    ($state:expr, $($arg:tt)*) => {
        use std::fmt::Write;
        use std::thread::current;
        use $crate::tool::{Datetime, TimeZone};

        let th_id = format!("{:?}", current().id()).replace("ThreadId", "线程");
        let th_name = &(current().name().unwrap().to_string() + "0")[0..5];

        let now = Datetime::local(TimeZone::E08);

        let mut output = String::new();
        write!(&mut output, $($arg)*).unwrap();
        println!("\x1b[36m{}{}\x1b[0m {} \x1b[{}\x1b[0m {}", th_name, &th_id, now, $state, output);
    };
}

pub struct Datetime {
    year: u64,
    month: u64,
    day: u64,
    hour: u64,
    minute: u64,
    second: u64,
}

#[derive(Default)]
pub enum TimeZone {
    W12 = -12,
    W11 = -11,
    W10 = -10,
    W09 = -9,
    W08 = -8,
    W07 = -7,
    W06 = -6,
    W05 = -5,
    W04 = -4,
    W03 = -3,
    W02 = -2,
    W01 = -1,
    #[default]
    UTC = 0,
    E01 = 1,
    E02 = 2,
    E03 = 3,
    E04 = 4,
    E05 = 5,
    E06 = 6,
    E07 = 7,
    E08 = 8,
    E09 = 9,
    E10 = 10,
    E11 = 11,
    E12 = 12,
}

impl Datetime {
    pub fn local(time_zone: TimeZone) -> Self {
        let mut dt = Self::default();
        let system_now = SystemTime::now();
        if let Ok(mut duration) = system_now.duration_since(std::time::UNIX_EPOCH) {
            duration = duration.add(Duration::from_secs(
                3600 * <TimeZone as Into<u64>>::into(time_zone),
            ));
            dt.second = duration.as_secs();

            dt.day = dt.second / 86400 + 1; // 计算天数
            dt.second = dt.second % 86400; // 计算余下秒数

            dt.hour = dt.second / 3600; // 余下秒数计算小时数
            dt.second = dt.second % 3600; // 计算余下秒数

            dt.minute = dt.second / 60; // 余下秒数计算分钟数
            dt.second = dt.second % 60; //余下秒数

            dt.year = 1970;
            while dt.day >= days_in_year(dt.year) {
                dt.day -= days_in_year(dt.year); // 剩余天数大于一年，减去当年天数
                dt.year += 1; //减去天数后年份增加
            }

            dt.month = 1;
            while dt.day >= days_in_month(dt.year, dt.month) {
                dt.day -= days_in_month(dt.year, dt.month);
                dt.month += 1;
            }
        }
        dt
    }
}

impl Default for Datetime {
    fn default() -> Self {
        Self {
            year: 1970,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
        }
    }
}

impl Display for Datetime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

impl Into<u64> for TimeZone {
    fn into(self) -> u64 {
        self as u64
    }
}

// 计算给定年份的天数
fn days_in_year(year: u64) -> u64 {
    if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
        366
    } else {
        365
    }
}

// 计算给定年份和月份的天数
fn days_in_month(year: u64, month: u64) -> u64 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31, // 31天大月
        4 | 6 | 9 | 11 => 30,              // 30天小月
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                29
            } else {
                28
            }
        } // 2月闰年计算
        _ => 0,                            // 非法月份
    }
}
