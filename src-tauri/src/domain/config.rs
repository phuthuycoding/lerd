use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// Cấu trúc cho cấu hình dịch vụ
#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceConfig {
    pub version: String,
    pub status: String,
}

// Đường dẫn mặc định cho thư mục cấu hình
fn get_config_path(service_name: &str) -> String {
    format!("{}/.lerd-config/{}.yaml", env!("HOME"), service_name)
}

// Hàm đọc file cấu hình cho một dịch vụ
pub fn read_service_config(service_name: &str) -> Result<ServiceConfig, Box<dyn std::error::Error>> {
    let file_path = get_config_path(service_name);

    // Kiểm tra nếu file không tồn tại thì trả về lỗi
    if !Path::new(&file_path).exists() {
        return Err(format!("Configuration file for {} does not exist", service_name).into());
    }

    let content = fs::read_to_string(file_path)?;
    let config: ServiceConfig = serde_yaml::from_str(&content)?;
    Ok(config)
}

// Hàm ghi file cấu hình cho một dịch vụ
pub fn write_service_config(service_name: &str, config: &ServiceConfig) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = get_config_path(service_name);
    
    // Tạo thư mục nếu chưa tồn tại
    if let Some(parent_dir) = Path::new(&file_path).parent() {
        fs::create_dir_all(parent_dir)?;
    }

    let content = serde_yaml::to_string(config)?;
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
