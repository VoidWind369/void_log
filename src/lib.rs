mod tool;

extern crate time;

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {{
        use std::fmt::Write;
        use std::thread::current;
        pub use time::{format_description, OffsetDateTime};

        let th_id = format!("{:?}", current().id()).replace("ThreadId", "线程");
        let th_name = &(current().name().unwrap().to_string() + "0")[0..5];

        let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
        let now = OffsetDateTime::now_local().unwrap().format(&format).unwrap();

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
        pub use time::{format_description, OffsetDateTime};

        let th_id = format!("{:?}", current().id()).replace("ThreadId", "线程");
        let th_name = &(current().name().unwrap().to_string() + "0")[0..5];

        let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
        let now = OffsetDateTime::now_local().unwrap().format(&format).unwrap();

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
        pub use time::{format_description, OffsetDateTime};

        let th_id = format!("{:?}", current().id()).replace("ThreadId", "线程");
        let th_name = &(current().name().unwrap().to_string() + "0")[0..5];

        let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
        let now = OffsetDateTime::now_local().unwrap().format(&format).unwrap();

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
        pub use time::{format_description, OffsetDateTime};

        let th_id = format!("{:?}", current().id()).replace("ThreadId", "线程");
        let th_name = &(current().name().unwrap().to_string() + "0")[0..5];

        let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
        let now = OffsetDateTime::now_local().unwrap().format(&format).unwrap();

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
        pub use time::{format_description, OffsetDateTime};

        let th_id = format!("{:?}", current().id()).replace("ThreadId", "线程");
        let th_name = &(current().name().unwrap().to_string() + "0")[0..5];

        let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
        let now = OffsetDateTime::now_local().unwrap().format(&format).unwrap();

        let mut output = String::new();
        write!(&mut output, $($arg)*).unwrap();
        println!("\x1b[36m{}{}\x1b[0m {} \x1b[95m[ 消息 ]\x1b[0m {}", th_name, &th_id, now, output);
    }};
}

#[test]
fn test() {
    log_info!("Info");
    log_warn!("Warn");
    log_error!("Error");
    log_msg!("Message");
}