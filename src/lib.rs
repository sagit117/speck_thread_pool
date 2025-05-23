#![crate_type = "lib"]
#![crate_name = "speck_thread_pool"]
use std::{error::Error, sync::{mpsc, Arc, Mutex}, thread::{self, JoinHandle}};

/// Пул потоков.
/// 
/// #Example
/// 
/// ```
/// use std::{thread, time::Duration};
/// use speck_thread_pool::ThreadPool;
/// let pool = ThreadPool::new(4).unwrap();
/// pool.execute(|| {
///     thread::sleep(Duration::new(1, 0));
/// println!("execute");
/// });
/// pool.execute(|| {
///     thread::sleep(Duration::new(2, 0));
///     println!("execute 0");   
/// });
/// pool.execute(|| {
///     thread::sleep(Duration::new(2, 0));
///     println!("execute 1");    
/// });
/// pool.execute(|| {
///     thread::sleep(Duration::new(2, 0));
///     println!("execute 2");
/// });
/// pool.execute(|| {
///     println!("execute last");
/// });
/// thread::sleep(Duration::new(3, 0));
///```
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl ThreadPool {
    /// Создайте новый ThreadPool.
    ///
    /// Размер - это количество потоков в пуле.
    ///
    /// ## Panics
    ///
    /// Функция `new` завершится с ошибкой, если размер равен нулю.
    pub fn new(size: usize) -> Result<ThreadPool, Box<dyn Error>> {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            let w = Worker::new(id, Arc::clone(&receiver))?;
            workers.push(w);
        }

        Ok(
            ThreadPool { 
                workers, 
                sender: Some(sender) 
            }
        )
    }

    /// Метод обработки запроса.
    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

struct Worker {
    #[allow(dead_code)]
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Result<Worker, Box<dyn Error>> {
        let builder = thread::Builder::new();
        let thread = builder.spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        })?;

        Ok(Worker { id, thread: Some(thread) })
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;