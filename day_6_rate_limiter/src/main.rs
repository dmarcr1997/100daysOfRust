use std::{thread, time::{Duration, SystemTime}};
use std::sync::{Arc, Mutex, mpsc};
use rand::{prelude::IndexedRandom, distr::Alphanumeric, Rng};


#[derive(Debug)]
struct EncryptedMessage {
    id: u32,
    origin: String,
    payload: String,
    timestamp: SystemTime
}

fn generate_encrypted_message() -> String {
    let payload: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    let tag = ["##CMD", "##SIG", "##DATA", "##AUTH"]
        .choose(&mut rand::rng())
        .unwrap_or(&"##GEN");
    format!("{}:0x{}", tag, payload)
}

fn main() {
    //create mpsc channel 
    let (tx, rx) = mpsc::channel();
    let tx = Arc::new(tx);

    let capacity: usize = 5;
    let token_bucket = Arc::new(Mutex::new(capacity));
    let refill_interval = Duration::from_millis(500);
    let tb1 = Arc::clone(&token_bucket);
    thread::spawn(move || {
        loop {
            thread::sleep(refill_interval);
            let mut tokens = tb1.lock().unwrap();
            if *tokens < capacity {
                *tokens += 1;
                println!("[GhostShell] +1 token ({} total)", *tokens);
            }
        }
    });
    for id in 0..20 {
        let tx_clone = Arc::clone(&tx);
        let tb2 = Arc::clone(&token_bucket);
        thread::spawn(move || {
            loop {
                let recv_token = {
                    let mut tkn = tb2.lock().unwrap();
                    if *tkn > 0 {
                        *tkn -= 1;
                        true
                    } else {
                        false
                    }
                };
                if recv_token {
                    let msg = EncryptedMessage {
                        id,
                        origin: format!("Agent-{}", id),
                        payload: generate_encrypted_message(),
                        timestamp: SystemTime::now()
                    };
                    tx_clone.send(msg).unwrap();
                    break;
                } else {
                    let queued = tb2.lock().unwrap();
                    println!(
                        "[GhostShell] ⚠ Agent-{} rate limited. Tokens left: {}. Retrying...",
                        id, *queued
                    );
                    thread::sleep(Duration::from_millis(500));
                }

            }
        });
    }
    loop {
        match rx.recv() {
            Ok(msg) => {
                println!("[🛰️ GHOST NODE] 📦 {} | from {} @ {:?}", 
                    msg.payload, msg.origin, msg.timestamp);
                thread::sleep(Duration::from_millis(500)); // simulate decrypt
            }
            Err(e) => {
                println!("[GhostShell] No messages received: {}", e);
                break;
            }
        }
    }
}
