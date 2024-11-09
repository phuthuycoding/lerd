use std::process::Command;

pub fn start(service_name: &str) -> Result<String, String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("powershell")
            .args(&["-Command", &format!("Start-Service -Name {}", service_name)])
            .output()
    } else {
        Command::new("systemctl")
            .arg("start")
            .arg(service_name)
            .output()
    };

    match output {
        Ok(o) if o.status.success() => Ok(format!("Started service: {}", service_name)),
        Ok(o) => Err(format!(
            "Failed to start service: {} - {}",
            service_name,
            String::from_utf8_lossy(&o.stderr)
        )),
        Err(e) => Err(format!("Error: {}", e)),
    }
}

pub fn stop(service_name: &str) -> Result<String, String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("powershell")
            .args(&["-Command", &format!("Stop-Service -Name {}", service_name)])
            .output()
    } else {
        Command::new("systemctl")
            .arg("stop")
            .arg(service_name)
            .output()
    };

    match output {
        Ok(o) if o.status.success() => Ok(format!("Stopped service: {}", service_name)),
        Ok(o) => Err(format!(
            "Failed to stop service: {} - {}",
            service_name,
            String::from_utf8_lossy(&o.stderr)
        )),
        Err(e) => Err(format!("Error: {}", e)),
    }
}
