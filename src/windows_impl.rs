// src/windows_impl.rs

use std::io::{self, Write};

#[link(name = "kernel32")]
unsafe extern "system" {
    fn IsDebuggerPresent() -> i32;
}

pub fn init() {
    println!("[+] Windows-specific monitoring hooks loaded.");
    check_environment_integrity();
}

fn check_environment_integrity() {
    print!("[*] Running Windows environment integrity scan... ");
    let _ = io::stdout().flush(); // Ensure the prompt renders before execution

    unsafe {
        if IsDebuggerPresent() != 0 {
            trigger_sarcophagus("Windows user-mode debugger detected!".to_string());
        }
    }

    println!("Clean.");
}

fn trigger_sarcophagus(reason: String) {
    eprintln!("\n[!] SECURITY ALERT: {}", reason);
    eprintln!("[!] Sarcophagus protocol engaged. Purging state...");
    std::process::exit(1);
}   