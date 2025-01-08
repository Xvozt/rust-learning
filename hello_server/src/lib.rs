use std::{
    error::Error,
    fmt,
    fs::read,
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
    thread,
};
// Stopped at A Worker Struct Responsible for Sending Code from the ThreadPool to a Thread
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

#[derive(Debug, Clone)]
pub struct ThreadPoolCreationError;

impl fmt::Display for ThreadPoolCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Thread pool size must be greater then zero")
    }
}

impl Error for ThreadPoolCreationError {}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let (sender, reciever) = mpsc::channel();
        let reciever = Arc::new(Mutex::new(reciever));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&reciever)));
        }

        ThreadPool { workers, sender }
    }
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Error
    ///
    /// The `build` function will return `ThreadPoolCreationError if the size is zero.
    pub fn build(size: usize) -> Result<ThreadPool, ThreadPoolCreationError> {
        match size {
            0 => Err(ThreadPoolCreationError),
            _ => Ok(ThreadPool::new(size)),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

impl Worker {
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = reciever.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing.");
            job();
        });
        Worker { id, thread }
    }
}
