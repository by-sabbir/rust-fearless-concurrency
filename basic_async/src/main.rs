use futures::executor::block_on;
use futures::join;

async fn say_again() {
    println!("hello again")
}

async fn say_hello() {
    println!("hello");
    join!(say_again(), say_goodye());
}

async fn say_goodye() {
    println!("goodbye")
}

async fn say_with_await() {
    say_again().await
}

fn main() {
    let future = say_hello();
    block_on(future);

    println!("with `await`");
    block_on(say_with_await())
}
