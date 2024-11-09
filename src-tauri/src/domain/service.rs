use crate::infrastructure::service_controller;

pub fn start_service(service_name: &str) -> Result<String, String> {
    service_controller::start(service_name)
}

pub fn stop_service(service_name: &str) -> Result<String, String> {
    service_controller::stop(service_name)
}