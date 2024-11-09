use crate::infrastructure::logger;

pub fn log_start_action(service_name: &str) {
    logger::log_service_action(service_name, "started").unwrap();
}

pub fn log_stop_action(service_name: &str) {
    logger::log_service_action(service_name, "stopped").unwrap();
}