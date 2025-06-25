use std::sync::mpsc;
use std::thread::{JoinHandle};
use std::sync::{Arc, Mutex};
struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;


impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let (tx ,rx) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(rx));
        let mut workers = Vec::<Worker>::new();
        for id in 0..size {
            let rx_clone = receiver.clone();
            let thread = std::thread::spawn(move || {
                loop {
                    let job = rx_clone.lock().unwrap().recv();
                    match job {
                        Ok(job) => {
                            println!("Worker {} got a job; executing.", id);
                            job();
                        },
                        Err(_) => {
                            println!("Worker {} disconnected; shutting down.", id);
                            break;
                        }
                    }
                }
            });
            workers.push(Worker {
                id,
                thread: Some(thread),
            });
        }
        ThreadPool {
            workers,
            sender: tx,
        }
    }
    fn execute(&self, job: Job) {
        self.sender.send(job).expect("Failed to send job to thread pool");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(&self.sender);

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                println!("Shutting down worker {}", worker.id);
                thread.join().expect("Failed to join worker thread");
            }
        }
    }
}
fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let pool = ThreadPool::new(4);
    let chunk_size = (data.len() + pool.workers.len() - 1) / pool.workers.len();
    for chunk in data.chunks(chunk_size) {
        let chunk = chunk.to_vec(); // Clone the chunk to move into the thread
        pool.execute(Box::new(move || {
            let new_chunk: Vec<_> = chunk.iter().map(|&x| x * x).collect();
            println!("Processed chunk: {:?}", new_chunk);
        }));
    }
}
