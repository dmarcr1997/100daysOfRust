use std::collections::BinaryHeap;

#[derive(Debug, PartialEq)]
pub enum JobStatus {
    Pending,
    Running,
    Done,
    Cancelled
}
pub struct Job {
    pub id: u32,
    pub priority: u8,
    pub status: JobStatus,
    pub task: Box<dyn FnOnce() + Send>,
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

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.id == other.id
    }
}
impl Eq for Job {}
impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Job {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
            .then_with(|| self.id.cmp(&other.id))
    }
}

pub struct JobQueue {
    pub queue: BinaryHeap<Job>,
}

impl JobQueue {
    pub fn new() -> Self {
        JobQueue {queue: BinaryHeap::new()}
    }

    pub fn push(&mut self, job: Job) {
        self.queue.push(job);
    }

    pub fn pop(&mut self) -> Option<Job> {
        self.queue.pop()
    }
}