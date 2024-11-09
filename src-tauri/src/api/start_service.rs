use crate::domain::service::start_service;
use tauri::command;

#[command]
pub fn start_service_command(service_name: String) -> Result<String, String> {
    start_service(&service_name)
}