# 纯Rust实现的Log日志库

* 输出语言均为中文

* Cargo.toml
```
[dependencies]
void_log = { git = "https://github.com/VoidWind369/void_log.git" }
```

* Code
```
#[test]
fn test() {
    log_warn!("Warn");
    log_error!("Error");
    log_link!("Link");
    log_msg!("Message");
    log_debug!("Debug");
}
```
