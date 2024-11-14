pub use crate::tool::{Datetime, TimeZone};

pub mod tool;

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {{
        $crate::log!("32m[ 正常 ]", $($arg)*);
    }};
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {{
        $crate::log!("33m[ 警告 ]", $($arg)*);
    }};
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {{
        $crate::log!("31m[ 错误 ]", $($arg)*);
    }};
}

#[macro_export]
macro_rules! log_link {
    ($($arg:tt)*) => {{
        $crate::log!("35m[ 通信 ]", $($arg)*);
    }};
}

#[macro_export]
macro_rules! log_msg {
    ($($arg:tt)*) => {{
        $crate::log!("95m[ 消息 ]", $($arg)*);
    }};
}

#[test]
fn test() {
    log_info!("{}", Datetime::local(TimeZone::E08));
    log_warn!("Warn");
    log_error!("Error");
    log_link!("Link");
    log_msg!("Message");
}