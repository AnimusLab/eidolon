

#[cfg(target_os = "windows")]
use windows_sys::Win32::System::Diagnostics::Debug::IsDebuggerPresent;

/// Scans the host environment for analysis hooks. 
/// Returns `true` if clean, `false` if an intrusion/debugger is detected.
pub fn verify_environment() -> bool {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            if IsDebuggerPresent() != 0 {
                println!("[!] SARCOPHAGUS ALERT: Active user-mode debugger detected via PEB.");
                return false;
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        // Parse /proc/self/status for TracerPid
        if let Ok(status_content) = std::fs::read_to_string("/proc/self/status") {
            for line in status_content.lines() {
                if line.starts_with("TracerPid:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() > 1 {
                        if let Ok(tracer_pid) = parts[1].parse::<u32>() {
                            if tracer_pid != 0 {
                                println!("[!] SARCOPHAGUS ALERT: Process trace active (TracerPid: {}).", tracer_pid);
                                return false;
                            }
                        }
                    }
                }
            }
        }
    }

    true
}