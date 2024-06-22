fn main() {
    const N_THREADS: usize = 8;

    let to_add: Vec<u32> = (0..5000).collect();

    let chunks = to_add.chunks(N_THREADS);

    let mut thread_handles = Vec::new();

    for chunk in chunks {
        let chunk = chunk.to_owned();
        let th = std::thread::spawn(move || calc_chunk_sum(chunk.to_vec()));
        thread_handles.push(th);
    }

    let mut sum: u32 = 0;
    thread_handles.into_iter().for_each(|h| {
        let chunk_sum = h.join().unwrap();
        sum += chunk_sum;
    });

    println!("Total Sum: {sum}");
}

fn calc_chunk_sum(chunk: Vec<u32>) -> u32 {
    let sum = chunk.iter().sum();
    sum
}
