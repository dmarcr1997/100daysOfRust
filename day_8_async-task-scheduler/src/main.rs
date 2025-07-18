use std::{
    collections::VecDeque, future::Future, pin::Pin, sync::{Arc, Mutex}, task::{Context, Poll, RawWaker, RawWakerVTable, Waker}, thread, time::Duration
};

struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    queue: Arc<Mutex<VecDeque<Arc<Task>>>>
}

struct Executor {
    queue: Arc<Mutex<VecDeque<Arc<Task>>>>,
}

impl Executor {
    fn new() -> Self {
        Executor { queue: Arc::new(Mutex::new(VecDeque::new())) }
    }

    fn spawn(&self, fut: impl Future<Output = ()> + Send + 'static) {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(fut)),
            queue: Arc::clone(&self.queue)
        });
        self.queue.lock().unwrap().push_back(task);
    }

    fn run(&self) {
        loop {
            let maybe_task = self.queue.lock().unwrap().pop_front();
            match maybe_task {
                Some(task) => {
                    let waker = waker_from_task(Arc::clone(&task));
                    let mut context = Context::from_waker(&waker);

                    let poll_result = {
                    let mut future = task.future.lock().unwrap();
                        future.as_mut().poll(&mut context)
                    };

                    if let Poll::Pending = poll_result {
                        // Reschedule the task for another poll
                        self.queue.lock().unwrap().push_back(task);
                    }
                }
                None => {
                    // No tasks left right now — check if any were requeued
                    thread::sleep(Duration::from_millis(1)); // simulate reactor waiting
                    if self.queue.lock().unwrap().is_empty() {
                        break;
                    }
                }
            }
        }
    }

}

fn dummy_raw_waker() -> RawWaker {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        dummy_raw_waker()
    }

    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
    RawWaker::new(std::ptr::null(), &VTABLE)
}

fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

fn waker_from_task(task: Arc<Task>) -> Waker {
    unsafe fn clone(data: *const ()) -> RawWaker {
        let arc = Arc::<Task>::from_raw(data as *const Task);
        let _ = arc.clone();
        std::mem::forget(arc);
        RawWaker::new(data, &VTABLE)
    }

    unsafe fn wake(data: *const ()) {
        let arc = Arc::<Task>::from_raw(data as *const Task);
        arc.queue.lock().unwrap().push_back(arc.clone());
    }

    unsafe fn wake_by_ref(data: *const ()) {
        let arc = Arc::<Task>::from_raw(data as *const Task);
        arc.queue.lock().unwrap().push_back(arc.clone());
        std::mem::forget(arc);
    }

    unsafe fn drop(_: *const ()) {}

    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
    let data = Arc::into_raw(task) as *const ();
    unsafe { Waker::from_raw(RawWaker::new(data, &VTABLE))}
}

struct DummyFuture {
    count: usize,
}
impl Future for DummyFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.count < 2 {
            println!("Not ready yet...");
            self.count += 1;
            Poll::Pending
        } else {
            println!("Done!");
            Poll::Ready(())
        }
    }
}

fn main() {
    let exec = Executor::new();

    exec.spawn(DummyFuture { count: 0 }); // spawn directly
    exec.run(); // only run once
}