use std::{
    fmt,
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

pub struct ThreadPool {
    limit: usize,
    threads: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub enum ThreadPoolErr {
    InvalidPoolLimit,
}

impl ThreadPool {
    pub fn new(limit: usize) -> Result<ThreadPool, ThreadPoolErr> {
        if limit <= 0 {
            return Err(ThreadPoolErr::InvalidPoolLimit);
        }
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers: Vec<Worker> = Vec::with_capacity(limit);
        for id in 0..limit {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Ok(ThreadPool {
            limit,
            threads: workers,
            sender: Some(sender),
        })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.threads {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl fmt::Display for ThreadPoolErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            ThreadPoolErr::InvalidPoolLimit => String::from("Invalid pool limit"),
        };
        write!(f, "{}", error)
    }
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
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
