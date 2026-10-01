use std::fs;

pub fn init() {
    println!("[+] Linux-specific monitoring hooks loaded.");
    check_enviornment_integrity();
}

fn check_enviornment_integrity() {
    print!("[+] Running Linux environment integrity scan...");

    // Check TracPid in /proc/self/status
    if let Ok(status_content) = fs::read_to_string("/proc/self/status") {
        for line in status_content.lines() {
            if line.starts_with("TracPid:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() > 1 {
                    if let Ok(pid) = parts[1].parse::<i32>() {
                        if pid > 0 {
                            trigger_sarcophagus(format!("debugger detected! TracPid: {}", pid));
                        }
                    }
                }
            }
        }
    }
    println!("Clean.");
}

fn trigger_sarcophagus(reason: String) {
    println!("[!] SECURITY ALERT: {}", reason);
    println!("[!]  Sarcophagus protocol engaged. Purging state...");
    // Future: Overwrite memory/files and exit
    std::process::exit(1);
}

