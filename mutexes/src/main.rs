use std::sync::Mutex;

static DATA: Mutex<Vec<i32>> = Mutex::new(Vec::new());

fn main() {
    let mut handlers = Vec::new();
    for n in 0..50 {
        let th = std::thread::spawn(move || {
            let mut lock = DATA.lock().unwrap();
            lock.push(n);
        });
        handlers.push(th)
    }
    handlers.into_iter().for_each(|h| h.join().unwrap());

    let lock = DATA.lock().unwrap();
    println!("{:?}", lock);
}
