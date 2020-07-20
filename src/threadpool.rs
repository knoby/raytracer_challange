//! Implementation of a simple threadpool according to the Rust Book
use std::sync::{Arc, Mutex};

pub struct ThreadPool {
    /// The Vector with all worker threads
    workers: Vec<Worker>,
    /// The sender to send a command to the threads
    sender: std::sync::mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Creates a new threadpool with the desired number of threads.
    ///
    /// # Panics
    /// Functions panics if num_of_threads is 0.
    pub fn new(num_of_threads: usize) -> Self {
        // Check Valid thread size
        assert!(num_of_threads > 0);

        // Create the Sender Reciver for sending work to the threads
        let (sender, reciver) = std::sync::mpsc::channel::<Job>();
        // Wrap reciver in arc/mutex to send safely to threads
        let reciver = Arc::new(Mutex::new(reciver));

        // Create a Vector for the threadworker
        let mut workers = std::vec::Vec::with_capacity(num_of_threads);

        // Create threads
        for _ in 0..num_of_threads {
            workers.push(Worker::new(reciver.clone()));
        }

        Self { workers, sender }
    }

    /// Execute a function in the threadpool
    pub fn execute<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // Encueue the function to the worker
        self.sender.send(Job::ExecuteJob(Box::new(f))).unwrap();
    }
}

impl Drop for ThreadPool {
    /// Join all threads to be able to finish the program
    fn drop(&mut self) {
        // Send Join Signal to all threads
        for _ in &self.workers {
            self.sender.send(Job::JoinThread).unwrap();
        }
        // At this point every thread should have recived the join signal so we can end them
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

/// Struct that does the work in the threadpool
struct Worker {
    /// Some unique identifyer. Has to be implemented in the generating unit.
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Worker {
    // Creates a new Worker
    pub fn new(reciver: Arc<Mutex<std::sync::mpsc::Receiver<Job>>>) -> Self {
        let thread = std::thread::spawn(move || loop {
            let job = reciver.lock().unwrap().recv().unwrap();
            match job {
                // Execute the job
                Job::ExecuteJob(job) => {
                    job();
                }
                Job::JoinThread => {
                    // End the loop
                    break;
                }
            }
        });
        Self {
            thread: Some(thread),
        }
    }
}

/// Job for the threads
pub enum Job {
    /// Job for the task to execute some function
    ExecuteJob(Box<dyn FnOnce() + Send + 'static>),
    /// End the thread
    JoinThread,
}
