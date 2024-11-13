mod tool;

pub use time::{format_description, OffsetDateTime};

#[test]
fn test() {
    log_info!("Info");
    log_warn!("Warn");
    log_error!("Error");
    log_msg!("Message");
}