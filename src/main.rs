use std::{thread, time::Duration};
use speck_thread_pool::ThreadPool;

fn main() {
    let pool = ThreadPool::new(4).unwrap();

    pool.execute(|| {
        thread::sleep(Duration::new(1, 0));
        println!("execute");
    });

    pool.execute(|| {
        thread::sleep(Duration::new(2, 0));
        println!("execute 0");
        
    });

    pool.execute(|| {
        thread::sleep(Duration::new(2, 0));
        println!("execute 1");
        
    });

    pool.execute(|| {
        thread::sleep(Duration::new(2, 0));
        println!("execute 2");
    });

    pool.execute(|| {
        println!("execute last");
    });

    thread::sleep(Duration::new(3, 0));
}
