use std::thread;

fn main() {
    // Start a new thread and run the code inside the closure
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("Spawned thread testing...{}", i);
        }
    });

    // This loop runs in the main thread
    for i in 0..10 {
        println!("Main thread testing...{}", i);
    }

    // Wait for the spawned thread to finish before exiting
    handle.join().unwrap();
}