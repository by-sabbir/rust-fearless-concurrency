fn do_work(n: i32) {
    let mut a = n * n;

    for _ in 1..a {
        a += n;
    }

    println!("total: {a}");
}
fn main() {
    let mut threads = Vec::new();
    for x in 1..10 {
        let th = std::thread::spawn(move || {
            do_work(x);
        });
        threads.push(th)
    }

    threads.into_iter().for_each(|h| h.join().unwrap());
}
