use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;

pub fn log_service_action(service_name: &str, action: &str) -> std::io::Result<()> {
    let log_path = format!("{}/.lerd-logs/{}.log", env!("HOME"), service_name);
    let mut file = OpenOptions::new().create(true).append(true).open(log_path)?;
    let log_entry = format!(
        "{} - {}: {}\n",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        service_name,
        action
    );
    file.write_all(log_entry.as_bytes())?;
    Ok(())
}
