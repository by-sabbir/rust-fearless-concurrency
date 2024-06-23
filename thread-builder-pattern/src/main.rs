fn hello() {
    println!("hello from thread: {:?}", std::thread::current().name())
}

fn main() {
    let th = std::thread::Builder::new()
        .name("Named Thread".to_string())
        .stack_size(std::mem::size_of::<usize>() * 4)
        .spawn(hello)
        .unwrap();
    th.join().unwrap();
}
