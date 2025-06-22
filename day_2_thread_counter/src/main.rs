use std::{thread, time::Duration};
use std::sync::{Arc, Mutex};

fn thread_counter(counter: Arc<Mutex<i32>>, id: i32) {
    let mut num = counter.lock().unwrap();
    *num += 1;
    println!("Thread {} has incremented the counter to {}", id, *num);
    thread::sleep(Duration::from_millis(10));
}

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for i in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let thread = thread::spawn(move || thread_counter(counter_clone, i));
        handles.push(thread);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Final count: {}", *counter.lock().unwrap());
}
