use std::sync::{Arc, Mutex, mpsc::{Sender, Receiver, channel, RecvTimeoutError}};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use crate::job::Job;

pub struct WorkerPool {
    pub workers: Vec<JoinHandle<()>>,
    pub sender: Sender<Job>
}

impl WorkerPool {
    pub fn new(num_workers: usize) -> Self {
        let (tx, rx) = channel::<Job>();
        let rx = Arc::new(Mutex::<Receiver<Job>>::new(rx));
        let mut threads: Vec<JoinHandle<()>> = Vec::new();
        for i in 0..num_workers {
            let rx_clone = Arc::clone(&rx);
            let new_thread = thread::spawn(move || {
                loop {
                    let check_job = {
                        let lock = rx_clone.lock().unwrap();
                        lock.recv_timeout(Duration::from_secs(20))
                    };
                    match check_job {
                        Ok(mut jb) => {
                            jb.run();
                            println!("Worker #{} completed job {}", i, jb.id);
                        },
                        Err(RecvTimeoutError::Timeout) => {
                            eprintln!("Worker timed out after 20 seconds -- shutting down.");
                            break;
                        }
                        Err(e) => {
                            eprintln!("Worker recevier error: {}", e);
                            break;
                        },
                    }
                }
            });
            threads.push(new_thread);
        }
        Self {
            workers: threads,
            sender: tx,
        }
    }

    pub fn submit(&self, job: Job) {
        self.sender.send(job).unwrap();
    }

    fn shutdown() {

    }
}