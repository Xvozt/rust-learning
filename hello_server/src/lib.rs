use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}
#[derive(Debug, Clone)]
pub struct ThreadPoolCreationError;

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let mut threads = Vec::with_capacity(size);
        for _ in 0..size {
            todo!()
        }
        ThreadPool { threads }
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
    }
}
