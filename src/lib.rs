use std::{
    sync::{mpsc, Arc, Mutex}, 
    thread
};

type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug)]
/// An error associated to `ThreadPool`.
pub struct PoolCreationError;

/// A data structure to manage multiple existing threads.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
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

        let (sender, receiver) = mpsc::channel();
        let sender = Some(sender);
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        Ok(ThreadPool { workers, sender })
    }
    
    /// Send a job to the workers.
    ///
    /// `f` is the job to be executed in the form of closure.
    ///
    /// # Panics
    ///
    /// The method panics if it failed to send the job.
    pub fn execute<F>(&self, f: F) 
        where F: FnOnce() + Send + 'static 
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).expect("Failed to send job down the queue");
    }
}

impl Drop for ThreadPool {
    /// Drop the sender and wait the workers to finish their jobs.
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.job.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    job: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().expect("Failed to acquire resource").recv();
                
                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    },
                    Err(_) => {
                        println!("Worker {id} disconnected.");
                        break;
                    }
                }
            }
        });

        Worker { 
            id, 
            job: Some(thread),
        }
    }
}