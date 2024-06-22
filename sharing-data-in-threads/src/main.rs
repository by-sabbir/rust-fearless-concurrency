fn fake_work(mut n: u32) {
    for _ in 0..10 {
        n = n * 2;
    }
}

fn do_calc(n: u32) -> u32 {
    println!("######### spawned: {n}");
    fake_work(n);
    n * 2
}

fn main() {
    println!("Hello from main thread");

    let mut thread_handles = Vec::new();

    for i in 0..10 {
        let thread_handle = std::thread::spawn(move || do_calc(i));
        thread_handles.push(thread_handle);
    }

    thread_handles
        .into_iter()
        .for_each(|h| println!("========= calculated: {}", h.join().unwrap()));
}
