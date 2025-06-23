use std::sync::{Arc, mpsc};
use rand::Rng;
use std::fmt;
use std::time::Duration;
#[derive(Debug, PartialEq)]
enum Command {
    AdjustThrusters(f32),
    RotateTo(f32, f32, f32),
    SendTelemetry(String),
    Shutdown,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::AdjustThrusters(value) => write!(f, "Adjusting thrusters by {}", value),
            Command::RotateTo(x, y, z) => write!(f, "Rotating to angles x: {}, y: {}, z: {}", x, y, z),
            Command::SendTelemetry(data) => write!(f, "Sending telemetry data: {}", data),
            Command::Shutdown => write!(f, "Shutting down..."),
        }
    }
}
fn main() {
    let (tx, rx) = mpsc::channel();
    let tx = Arc::new(tx);
    for i in 0..4 {
        let tx_clone = tx.clone();
        std::thread::spawn(move || {
            let cmd = match i {
                0 => Command::AdjustThrusters(rand::thread_rng().gen_range(-1.0..=1.0)),
                1 => Command::RotateTo(
                    rand::thread_rng().gen_range(-180.0..=180.0), 
                    rand::thread_rng().gen_range(-180.0..=180.0), 
                    rand::thread_rng().gen_range(-180.0..=180.0)
            ),
                2 => Command::SendTelemetry("Telemetry data".to_string()),
                _ => {
                    std::thread::sleep(Duration::from_secs(1));
                    Command::Shutdown
                },
            };
            if let Err(e) = tx_clone.send(cmd) {
                eprintln!("Error sending command: {}", e);
            }
        });
    }
    loop {
        match rx.recv_timeout(Duration::from_secs(2)) {
            Ok(cmd) => {
                println!("Received command: {}", cmd);
                if cmd == Command::Shutdown {
                    println!("Shutting down the command receiver.");
                    break;
                }
            },
            Err(e) => {
                eprintln!("Error receiving command: {}", e);
                break;
            },
        }
    }
}
