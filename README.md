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

[sharing-data-in-thread](./sharing-data-in-threads/src)

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

## How to Contribute

If you'd like to contribute to this project, please follow these steps:

1. Fork the repository on GitHub.
2. Clone your forked copy locally.
3. Create a new branch for your changes (e.g., `feature/new-concurrency-pattern`).
4. Make your changes and commit them with descriptive commit messages.
5. Push your changes to your forked repository.

## How to Run the Project

To run this project, simply execute the following command:

```bash
cargo run
```
