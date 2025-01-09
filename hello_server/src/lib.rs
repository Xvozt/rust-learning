use std::{
    error::Error,
    fmt,
    sync::{
        mpsc::{self},
        Arc, Mutex,
    },
    thread,
};
// Stopped at A Worker Struct Responsible for Sending Code from the ThreadPool to a Thread
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
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

        ThreadPool {
            workers,
            sender: Some(sender),
        }
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
        self.sender.as_ref().unwrap().send(job).unwrap()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Worker {
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = reciever.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    job();
                    println!("Worker {id} got a job; executing.");
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
