pub extern crate time as void_time;
mod tool;

#[test]
fn test() {
    log_info!("Info");
    log_warn!("Warn");
    log_error!("Error");
    log_msg!("Message");
}