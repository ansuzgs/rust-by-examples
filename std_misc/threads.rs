use std::thread;

const NTHREADS: u32 = 10;

// This is the main thread
fn main() {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];

    for i in 0..NTHREADS {
        // Spin another thread
        children.push(thread::spawn(move || {
            println!("this is a thread number {}", i);
        }));
    }

    for child in children {
        // Wait for the thread to finish. Return a result.
        let _ = child.join();
    }
}
