use once_cell::sync::Lazy;
use std::sync::RwLock;

static USERS: Lazy<RwLock<Vec<String>>> = Lazy::new(|| RwLock::new(build_user()));

fn build_user() -> Vec<String> {
    vec!["Alice".to_string(), "Bob".to_string()]
}

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    std::thread::spawn(|| loop {
        let users = USERS.read().unwrap();
        println!("current user in thread: {users:?}");
        std::thread::sleep(std::time::Duration::from_secs(3));
    });

    loop {
        println!("input new user: ");
        let user = read_line();
        if user == "q" {
            break;
        }
        let mut lock = USERS.write().unwrap();
        lock.push(user);
    }
}
