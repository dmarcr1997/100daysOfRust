mod job;
mod worker;

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;
    use crate::job::{JobQueue, JobStatus, Job};
    use crate::worker::WorkerPool;
    use std::io::{self, Write};

    #[test]
    fn test_job_creation() {
        let task = || println!("Task executed");
        let job = Job::new(1, 5, Box::new(task));

        assert_eq!(job.id, 1);
        assert_eq!(job.priority, 5);
        assert_eq!(job.status, JobStatus::Pending);
    }

    #[test]
    fn test_job_queue_and_pop() {
        let mut queue = JobQueue::new();
        let task1 = || println!("Task 1 executed");
        let task2 = || println!("Task 2 executed");
        let job1 = Job::new(1, 10, Box::new(task1));
        let job2 = Job::new(2, 5, Box::new(task2));
        
        queue.push(job1);
        queue.push(job2);
        let popped_job = queue.pop().unwrap();
        assert_eq!(popped_job.id, 1);
        assert_eq!(popped_job.priority, 10);
    }

    #[test]
    fn test_job_exec_changes_status() {
        let ran = Arc::new(Mutex::new(false));
        let ran_clone = Arc::clone(&ran);

        let mut job = Job::new(42, 3, Box::new(move || {
            *ran_clone.lock().unwrap() = true;
        }));

        assert_eq!(job.status, JobStatus::Pending);
        job.run();
        assert_eq!(job.status, JobStatus::Done);
        assert_eq!(*ran.lock().unwrap(), true);
    }

    #[test]
    fn test_job_executes_in_worker() {
        let pool = WorkerPool::new(2);
        let result = Arc::new(Mutex::new(0));
        let result_clone = Arc::clone(&result);

        let job = Job::new(1, 5, Box::new(move || {
            *result_clone.lock().unwrap() = 42;
        }));

        pool.submit(job);

        thread::sleep(Duration::from_millis(500));
        assert_eq!(*result.lock().unwrap(), 42);
    }
}
