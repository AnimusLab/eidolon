// src/camouflage.rs
use std::thread;
use std::time::Duration;
use rand::Rng;

pub fn execute_with_jitter() {
    let mut rng = rand::thread_rng();

    // Generate a randomized operational delay between 3 and 10 seconds
    let jitter_seconds: u64 = rng.gen_range(3..10);
    
    println!("[*] Camouflage engaged. Entering dormant state for {} seconds (jitter masking active)...", jitter_seconds);
    
    // Sleep safely, masking execution timing from thread analysis
    thread::sleep(Duration::from_secs(jitter_seconds));
}