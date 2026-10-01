// src/crypto_vault.rs
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce
};
use aes_gcm::aead::rand_core::RngCore;
use std::fs;
use std::path::Path;

pub struct CryptoVault {
    key: [u8; 32],
}

impl CryptoVault {
    /// Initialize the vault with a generated or provided key
    pub fn new() -> Self {
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        Self { key }
    }

    /// Encrypt plaintext bytes and write them to disk securely
    pub fn secure_write(&self, path: &str, plaintext: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let cipher = Aes256Gcm::new((&self.key).into());
        
        // Generate a unique 96-bit nonce for this specific write operation
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt the payload
        let ciphertext = cipher.encrypt(nonce, plaintext)
            .map_err(|e| format!("Encryption failure: {:?}", e))?;

        // Pack nonce + ciphertext together for storage
        let mut stored_data = nonce_bytes.to_vec();
        stored_data.extend(ciphertext);

        // Write artifact to disk
        fs::write(path, stored_data)?;
        println!("[+] Local state successfully encrypted and stored at: {}", path);

        Ok(())
    }

    /// Read and decrypt an encrypted artifact from disk
    pub fn secure_read(&self, path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if !Path::new(path).exists() {
            return Err("Vault artifact not found.".into());
        }

        let stored_data = fs::read(path)?;
        if stored_data.len() < 12 {
            return Err("Corrupted vault artifact: Insufficient data for nonce.".into());
        }

        // Split nonce and ciphertext
        let (nonce_bytes, ciphertext) = stored_data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let cipher = Aes256Gcm::new((&self.key).into());
        
        // Decrypt and authenticate payload
        let plaintext = cipher.decrypt(nonce, ciphertext)
            .map_err(|e| format!("Decryption or authentication failure: {:?}", e))?;

        println!("[+] Vault artifact decrypted and verified clean.");
        Ok(plaintext)
    }
}

// Add this implementation block to src/crypto_vault.rs

impl Drop for CryptoVault {
    fn drop(&mut self) {
        println!("[*] Purging Cryptographic Vault from memory...");
        // Volatile write to ensure compiler doesn't optimize away the zeroization
        for byte in self.key.iter_mut() {
            unsafe {
                std::ptr::write_volatile(byte, 0);
            }
        }
        println!("[+] Vault cryptographic material securely zeroed from RAM.");
    }
}