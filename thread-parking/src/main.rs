use std::thread;

fn parkable_thread(n: u32) {
    loop {
        // thread::park_timeout(std::time::Duration::from_secs(5));
        thread::park();
        println!("unparked {n}")
    }
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut threads = Vec::new();

    for i in 0..10 {
        let h = thread::spawn(move || {
            parkable_thread(i);
        });
        threads.push(h);
    }

    loop {
        println!("input thread number: ");
        let inp = read_line();

        if inp == "q" {
            break;
        }

        if let Ok(number) = inp.parse::<usize>() {
            if number < 10 {
                threads[number].thread().unpark();
            }
        } else {
            println!("input is not a number")
        }
    }
}
