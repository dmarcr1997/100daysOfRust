use std::sync::mpsc;
use std::thread;

enum Message {
    Ping,
    Echo(String),
    Shutdown
}
struct Actor {
    name: String,
    receiver: mpsc::Receiver<Message>
}
impl Actor {
    fn run(&mut self) {
        loop {
            match self.receiver.recv() {
                Ok(msg) => match msg {
                    Message::Ping => println!("{} GOT PINGED!", self.name),
                    Message::Echo(text) => println!("ECHO: {}", text),
                    Message::Shutdown => {
                        println!("Shutting down: {}", self.name);
                        break;
                    }
                },
                Err(e) => {
                    eprintln!("ERROR: {}", e);
                    break;
                }
            }
        }
    }
}
fn main() {
    //create mailbox channel
    let (tx, rx) = mpsc::channel();
    let actor_thread = thread::spawn(move || {
        Actor {
            name: "0xDEADBEEF".to_string(),
            receiver: rx
        }.run();
    });
    tx.send(Message::Ping).unwrap();
    tx.send(Message::Echo("42?".to_string())).unwrap();
    tx.send(Message::Shutdown).unwrap();
    actor_thread.join().unwrap();
}
