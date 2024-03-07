pub use chrono::Local;
pub use std::fmt::Write;
pub use std::thread::current;
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {{

        let th_id = format!("{:?}", current().id()).replace("ThreadId", "线程");
        let th_name = &(current().name().unwrap().to_string() + "0")[0..5];

        let mut output = String::new();
        write!(&mut output, $($arg)*).unwrap();
        println!("\x1b[36m{}{}\x1b[0m {} \x1b[32m[ 正常 ]\x1b[0m {}", th_name, &th_id, Local::now().format("%Y-%m-%d %H:%M:%S"), output);
    }};
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {{

        let th_id = format!("{:?}", current().id()).replace("ThreadId", "线程");
        let th_name = &(current().name().unwrap().to_string() + "0")[0..5];

        let mut output = String::new();
        write!(&mut output, $($arg)*).unwrap();
        println!("\x1b[36m{}{}\x1b[0m {} \x1b[33m[ 警告 ]\x1b[0m {}", th_name, &th_id, Local::now().format("%Y-%m-%d %H:%M:%S"), output);
    }};
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {{

        let th_id = format!("{:?}", current().id()).replace("ThreadId", "线程");
        let th_name = &(current().name().unwrap().to_string() + "0")[0..5];

        let mut output = String::new();
        write!(&mut output, $($arg)*).unwrap();
        println!("\x1b[36m{}{}\x1b[0m {} \x1b[31m[ 错误 ]\x1b[0m {}", th_name, &th_id, Local::now().format("%Y-%m-%d %H:%M:%S"), output);
    }};
}

#[macro_export]
macro_rules! log_link {
    ($($arg:tt)*) => {{

        let th_id = format!("{:?}", current().id()).replace("ThreadId", "线程");
        let th_name = &(current().name().unwrap().to_string() + "0")[0..5];

        let mut output = String::new();
        write!(&mut output, $($arg)*).unwrap();
        println!("\x1b[36m{}{}\x1b[0m {} \x1b[35m[ 通信 ]\x1b[0m {}", th_name, &th_id, Local::now().format("%Y-%m-%d %H:%M:%S"), output);
    }};
}