use crate::time::{Datetime, TimeZone};

mod time;

pub fn get_local() -> Datetime {
    Datetime::local(TimeZone::E08)
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {{
        use std::fmt::Write;
        use std::thread::current;
        use $crate::time::{Datetime, TimeZone};

        let th_id = format!("{:?}", current().id()).replace("ThreadId", "线程");
        let th_name = &(current().name().unwrap().to_string() + "0")[0..5];

        let now = Datetime::local(TimeZone::E08).to_string();

        let mut output = String::new();
        write!(&mut output, $($arg)*).unwrap();
        println!("\x1b[36m{}{}\x1b[0m {} \x1b[32m[ 正常 ]\x1b[0m {}", th_name, &th_id, now, output);
    }};
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {{
        use std::fmt::Write;
        use std::thread::current;
        use $crate::time::{Datetime, TimeZone};

        let th_id = format!("{:?}", current().id()).replace("ThreadId", "线程");
        let th_name = &(current().name().unwrap().to_string() + "0")[0..5];

        let now = Datetime::local(TimeZone::E08).to_string();

        let mut output = String::new();
        write!(&mut output, $($arg)*).unwrap();
        println!("\x1b[36m{}{}\x1b[0m {} \x1b[33m[ 警告 ]\x1b[0m {}", th_name, &th_id, now, output);
    }};
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {{
        use std::fmt::Write;
        use std::thread::current;
        use $crate::time::{Datetime, TimeZone};

        let th_id = format!("{:?}", current().id()).replace("ThreadId", "线程");
        let th_name = &(current().name().unwrap().to_string() + "0")[0..5];

        let now = Datetime::local(TimeZone::E08).to_string();

        let mut output = String::new();
        write!(&mut output, $($arg)*).unwrap();
        println!("\x1b[36m{}{}\x1b[0m {} \x1b[31m[ 错误 ]\x1b[0m {}", th_name, &th_id, now, output);
    }};
}

#[macro_export]
macro_rules! log_link {
    ($($arg:tt)*) => {{
        use std::fmt::Write;
        use std::thread::current;
        use $crate::time::{Datetime, TimeZone};

        let th_id = format!("{:?}", current().id()).replace("ThreadId", "线程");
        let th_name = &(current().name().unwrap().to_string() + "0")[0..5];

        let now = Datetime::local(TimeZone::E08).to_string();

        let mut output = String::new();
        write!(&mut output, $($arg)*).unwrap();
        println!("\x1b[36m{}{}\x1b[0m {} \x1b[35m[ 通信 ]\x1b[0m {}", th_name, &th_id, now, output);
    }};
}

#[macro_export]
macro_rules! log_msg {
    ($($arg:tt)*) => {{
        use std::fmt::Write;
        use std::thread::current;
        use $crate::time::{Datetime, TimeZone};

        let th_id = format!("{:?}", current().id()).replace("ThreadId", "线程");
        let th_name = &(current().name().unwrap().to_string() + "0")[0..5];

        let now = Datetime::local(TimeZone::E08).to_string();

        let mut output = String::new();
        write!(&mut output, $($arg)*).unwrap();
        println!("\x1b[36m{}{}\x1b[0m {} \x1b[95m[ 消息 ]\x1b[0m {}", th_name, &th_id, now, output);
    }};
}

#[test]
fn test() {
    log_info!("{}", Datetime::local(TimeZone::E08).to_string());
    log_info!("Info");
    log_warn!("Warn");
    log_error!("Error");
    log_link!("Link");
    log_msg!("Message");
}