use std::sync::Mutex;

pub type Task = Box<dyn FnOnce() -> () + Send>;

pub fn parallel_execute(tasks: impl Iterator<Item=Task> + Send, worker_count: usize) {
    
    let tasks = Mutex::new(tasks);
    
    std::thread::scope(|s| {
        for _ in 0..worker_count {
            s.spawn(|| {
                loop {
                    let Some(task) = tasks.lock().unwrap().next() else {break};
                    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(task));
                    
                    if let Err(_) = result {
                        println!("Warning: a task panicked during execution");
                    }
                }
            });
        }
    });
}