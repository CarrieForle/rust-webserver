use std::thread;

pub struct PoolCreationError;

pub struct ThreadPool {
    threads: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0
        {
            return Err(PoolCreationError);
        }
        
        let mut threads = Vec::with_capacity(size);
        
        for id in 0..size {
            threads.push(Worker::new(id));
        }
        
        Ok(ThreadPool { threads })
    }
    
    pub fn execute<F>(&self, f: F) 
    where F: FnOnce() + Send + 'static 
    {
        
    }
}

struct Worker {
    id: usize,
    job: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        Worker { 
            id, 
            job: thread::spawn(|| {}),
        }
    }
}