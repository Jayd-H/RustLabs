fn main() {
    println!("Hello, world!");
    let num_of_threads = 5;
    let mut list_of_threads = vec!();
    for id in 0..num_of_threads {
        let t = std::thread::spawn(move || {
            println!("Thread {} is running", id);
        });
        list_of_threads.push(t);
    }

    println!("All threads are created!");
    
    for t in list_of_threads {
        t.join().unwrap();
    }
    println!("All threads have terminated");
}
