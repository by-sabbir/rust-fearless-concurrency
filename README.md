# Rust-Fearless-Concurrency

=========================

## Introduction

This project, "Rust-Fearless-Concurrency", is a showcase of concurrent programming in Rust. It aims to provide a set of reusable and easy-to-use concurrency utilities for building scalable and performant systems.

## Features

- A set of abstractions for handling concurrency in Rust applications.
- Implementations of common concurrency patterns, such as:

* Channels: Send and receive data between threads using channels.
* Locks: Implement fine-grained locking mechanisms for shared resources.
* Semaphores: Manage access to critical sections with semaphores.

## Fearless System Thread Concurrency

System threads are OS level construct. They can run multiple process at the same time. If only one CPU is available, threads allow us to fake running multiple tasks.

- Threas live in the parent process and shares the memory with it.
- Each thread has its own `stack` so it can run whatever we put into it.
- `Heap` is shared among threads, so we can share heap shared data between threads.
- Threads are scheduled by the OS itself.
- System threads are limited
  - We can get the max thread count in linux by `cat /proc/sys/kernel/threads-max`
  - On my machine it's `126284`. Which is lot, but we should minimize the number of threads to optimize the thread schedule time. Or else our CPU will use a lot of time scheduling them.
- It's always a good idea to create a thread-pool before hand. As creating threads it time consuming task.

### Creating a Thread

1. Spawn a thread - `std::thread::spawn(fn)`
2. Join the thread with parent process.

## Sharing data from/with threads

[sharing-data-in-thread](./sharing-data-in-threads/src/main.rs)

To share data with thread we need to move the value with the help of closure ie.

```rust
let n = 1024;
std::thread::spawn(move || do_something(n));
```

Reading data from a spawned thread is farely simple. The data is returned as te output of `.join()` function. ie. if we modify the above script to do some calculation on `n` -

```rust
do_calc(n: u32) -> u32 {
    do_fake_work();
    n
}

let n = 1024;
let th = std::thread::spawn(move || do_something(n));

let calculated_value = th.join().unwrap();
println("{}", calculated_value);
```

### Thread Builder Pattern
[thread-builder-patther](./thread-builder-pattern/src/main.rs)
Thread builder is native to rust, we can name a thread to understand what's going on in the production. Usually used in a large program with a lot of threads spawned.

```rust
std::thread::Builder::new().
    .name("Named Thread".to_string())
    .stack_size(std::mem::size_of::<usize>() * 4)
    .spawn(fn)
    .unwrap();
```
Note: `::<>` this notation is called turbofish format.
So, when do we want to use `stack_size` when the threads are by default 2mb in size.
1. When we have to work with a lot of threads like 20K or 30K. (but in this case we should use async/await)
2. When we know the exact size of the stack.
3. Reducing the stack size also helps the thread to load faster.

### Scoped Thread
[scoped-thread](./scoped-thread/src/main.rs)

Scope thread is a conveniant way to spawn a group of thread without worrying about the `join()` function. For that we need to prove that the group of threads only belong to a scope.

First we create a scoped thread. The `scope()` function takes another fuction that defines the thread lifecycle.

```rust
std::thread::scope(|s| {
    // spawn and join here
});
```

### Sharing Data with `Atomics`
When we need to share a global state, ie. a counter, rust provides a bunch or Atomic premitives that are concurrency safe.We don't need to make a variable mutable for the atomics as the use a pattern called `interior mutability`. For example if we want to declare an i32 atomic counter, the assignment will be-

```rust
use std::sync::atomic::AtomicI32;
static COUNTER: AtomicI3232 = AtomicI32::new(0);

// to add a number to our atomic counter
COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

// to get the COUNTER value
COUNTER.load(std::sync::atomic::Ordering::Relaxed);
```

### Mutexes
Mutexes are mainly used for complex data synchronization which are not available in `sync::atomic::*` crate. Mutexes are a bit slower than the atomics. To initiate a Mutex we use the following syntax -
```rust
use std::sync::Mutex;
...
// Declaring a mutex with i32 vector array
let data Mutex<Vec<i32>> = Mutex::new(Vec::new());
```

to access the value we gotta acquire the lock
```rust
let mut lock = data.lock().unwrap();
// now we can perform any vector ops on lock
```
## How to Run the Project

To run this project, simply execute the following command:

```bash
cargo run
```
