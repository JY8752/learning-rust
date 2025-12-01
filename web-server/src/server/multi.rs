use std::io::prelude::*;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::{
    net::{TcpListener, TcpStream},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        Worker {
            id,
            thread: thread::spawn(move || {
                loop {
                    let job = receiver.lock().unwrap().recv().unwrap();
                    job();
                }
            }),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

pub fn listen(host: &str, port: u16, thread_pool_size: usize) -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(format!("{}:{}", host, port))?;
    let pool = ThreadPool::new(thread_pool_size);
    for stream in listener.incoming() {
        let stream = stream?;
        pool.execute(move || {
            handle_connection(stream).unwrap();
        });
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<(), std::io::Error> {
    let mut buf = [0; 1024];
    let _ = stream.read(&mut buf)?;

    let contents = include_str!("../../hello.html");

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    let _ = stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}
