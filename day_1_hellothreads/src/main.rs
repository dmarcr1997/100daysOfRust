use std::thread;
fn thread_print(id: i32) {
    println!("Hello from thread {}", id);
} 
fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        let handle = thread::spawn(move || {
            thread_print(i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
