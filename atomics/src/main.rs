use std::sync::atomic::Ordering::Relaxed;
use std::{sync::atomic::AtomicI32, thread};

static COUNTER: AtomicI32 = AtomicI32::new(0);

fn main() {
    let mut handles = Vec::new();
    for _ in 0..1000 {
        let th = thread::spawn(move || {
            for _ in 0..1_100 {
                COUNTER.fetch_add(1, Relaxed);
            }
        });
        handles.push(th);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());

    println!("Atomic Counter: {}", COUNTER.load(Relaxed));
}
