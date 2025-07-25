use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
enum JobStatus {
    Pending,
    Running,
    Done,
    Cancelled
}
struct Job {
    id: u32,
    priority: u8,
    status: JobStatus,
    task: Box<dyn FnOnce() + Send>,
}

impl Job {
    pub fn new(id: u32, priority: u8, task: Box<dyn FnOnce() + Send>) -> Self {
        Job {
            id,
            priority,
            status: JobStatus::Pending,
            task,
        }
    }
    pub fn run(&mut self) {
        if self.status == JobStatus::Pending {
            self.status = JobStatus::Running;
            let task = std::mem::replace(&mut self.task, Box::new(|| {}));
            task();
            self.status = JobStatus::Done;
        }
    }
}

struct JobQueue {
    queue: VecDeque<Job>,
}

impl JobQueue {
    pub fn new() -> Self {
        JobQueue {queue: VecDeque::new()}
    }

    pub fn push(&mut self, job: Job) {
        self.queue.push_back(job);
    }

    pub fn pop(&mut self) -> Option<Job> {
        if self.queue.is_empty() {
            return None;
        }
        
        let idx = self.queue
            .iter()
            .enumerate()
            .max_by_key(|(_, job)| job.priority)
            .map(|(i, _)| i)?;
        Some(self.queue.remove(idx).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

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
}
