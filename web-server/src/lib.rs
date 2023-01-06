use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>; // type alias !!

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0); // no sense in having a thread pool of zero size yet zero is a valid usize
        
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        // more efficient than `Vec::new` which resizes itself as elements are inserted
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {workers, sender}
    }
    pub fn execute<F>(&self, f: F)
    where
        // using `FnOnce` because thats what `spawn` uses
        // theres an `()` after it because it takes no parameters and returns the unit type `()`
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing.");
            job(); //execute
        });
        Worker {id, thread}
    }
}
