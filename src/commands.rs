// src/commands.rs
use crate::crypto_vault::CryptoVault;
use std::error::Error;

/// Defines the global behavioral states of the agent lifecycle
#[derive(Debug, Clone, PartialEq)]
pub enum AgentState {
    Dormant,          // Standard jitter polling state
    DiagnosticMode,   // Executing active health checks
    Lockdown,         // Emergency state: halts standard polling, purges volatile indicators
}

/// Trait that all decentralized commands must implement
pub trait Command {
    fn execute(&self, vault: &CryptoVault) -> Result<AgentState, Box<dyn Error>>;
}

// --- Concrete Command 1: Diagnostic ---
pub struct DiagnosticCommand;

impl Command for DiagnosticCommand {
    fn execute(&self, vault: &CryptoVault) -> Result<AgentState, Box<dyn Error>> {
        println!("[*] FSM: Transitioning to Diagnostic State.");
        vault.secure_write("eidolon_state.vault", b"EIDOLON_STATUS: DIAGNOSTIC_RUNNING")?;
        Ok(AgentState::DiagnosticMode) // Transition into DiagnosticMode
    }
}

// --- Concrete Command 2: Lockdown ---
pub struct LockdownCommand;

impl Command for LockdownCommand {
    fn execute(&self, vault: &CryptoVault) -> Result<AgentState, Box<dyn Error>> {
        println!("[!] FSM: EMERGENCY LOCKDOWN COMMAND RECEIVED.");
        vault.secure_write("eidolon_state.vault", b"EIDOLON_STATUS: SECURED_LOCKDOWN")?;
        
        // Transition agent into permanent lockdown state
        Ok(AgentState::Lockdown)
    }
}

/// Factory function to parse a raw string payload into a dynamic command object
pub fn parse_command(command_str: &str) -> Option<Box<dyn Command>> {
    match command_str.trim() {
        "EXEC_DIAGNOSTIC" => Some(Box::new(DiagnosticCommand)),
        "TRIGGER_LOCKDOWN" => Some(Box::new(LockdownCommand)),
        _ => {
            println!("[!] Unknown instruction payload: {}", command_str);
            None
        }
    }
}