use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use log::{info, warn};

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<super::Job>>>) -> Worker {
        info!("spawned worker -> id: {id}");
        let thread = thread::spawn(move || loop {
            let message = receiver
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .recv();
            match message {
                Ok(job) => {
                    info!("worker `{id}` got a job; executing.");
                    job();
                }
                Err(_) => {
                    warn!("worker `{id}` disconnected; shutting down.");
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
