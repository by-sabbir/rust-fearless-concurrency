use std::sync::mpsc;

enum Command {
    SayHello,
    Quit,
}

fn main() {
    let (tx, rx) = mpsc::channel::<Command>();

    let h = std::thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::SayHello => println!("Say Hello"),
                Command::Quit => {
                    println!("Quitting");
                    break;
                }
            }
        }
    });

    for _ in 1..10 {
        tx.send(Command::SayHello).unwrap();
    }

    tx.send(Command::Quit).unwrap();

    h.join().unwrap();
}
