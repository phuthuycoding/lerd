use crate::domain::service::stop_service;
use tauri::command;

#[command]
pub fn stop_service_command(service_name: String) -> Result<String, String> {
    stop_service(&service_name)
}