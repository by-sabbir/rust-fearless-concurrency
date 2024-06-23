use std::thread;

fn main() {
    const N_CHUNKS: usize = 8;

    let to_add: Vec<u32> = (0..5000).collect();

    let chunks = to_add.chunks(N_CHUNKS);

    let sum = thread::scope(|s| {
        let mut thread_handles = Vec::new();

        for chunk in chunks {
            let th = s.spawn(move || chunk.iter().sum::<u32>());
            thread_handles.push(th);
        }

        thread_handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .sum::<u32>()
    });

    println!("Sum: {sum}");
}
