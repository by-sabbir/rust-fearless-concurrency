use std::{
    collections::VecDeque,
    sync::{mpsc, Mutex},
    thread,
    time::Duration,
};

use once_cell::sync::Lazy;

static WORK_QUEUE: Lazy<Mutex<VecDeque<String>>> = Lazy::new(|| Mutex::new(VecDeque::new()));

// work queue pattern
fn main() {
    let cpu_count = 2;
    let mut threads = Vec::with_capacity(cpu_count);
    let mut broadcast = Vec::with_capacity(cpu_count);

    for cpu in 0..cpu_count {
        let (tx, rx) = mpsc::channel::<()>();
        broadcast.push(tx);

        let th = thread::spawn(move || {
            while rx.recv().is_ok() {
                let mut lock = WORK_QUEUE.lock().unwrap();
                if let Some(work) = lock.pop_front() {
                    std::mem::drop(lock);
                    println!("CPU #{cpu} got work {work}");
                    std::thread::sleep(Duration::from_secs(2));
                    println!("#{cpu} task finished");
                } else {
                    println!("CPU #{cpu} found no work queue");
                }
            }
        });

        threads.push(th);
    }

    loop {
        let sent = {
            let mut lock = WORK_QUEUE.lock().unwrap();
            let len = lock.len();
            println!("Total Queue Item: #{len}");
            if len < 5 {
                lock.push_back("hello".to_string());
                true
            } else {
                false
            }
        };

        if sent {
            broadcast.iter().for_each(|tx| tx.send(()).unwrap());
        }
        thread::sleep(Duration::from_secs(1));
    }
}
