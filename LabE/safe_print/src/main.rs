use std::sync::{Arc, Mutex};
use std::thread;
use std::io::{self, Write};

//* THERE ARE TWO METHODS TO SOLVE THIS */
//* 1. use mutex with arc for thread-safe sharing using standard print! macros */
//* 2. use stdout().lock() for thread-safe sharing using write! and writeln! macros */

//* the mutex approach is better because its general-purpose and therefore the point of this lab */
//* the stdout().lock() approach is more memory efficient (no need for extra allocations) but less general-purpose */
//* realistically you are using mutex */

fn main() {
    println!("=== Approach 1: Using Arc<Mutex<()>> ===");
    
    let num_of_threads = 4;
    let mut array_of_threads = vec!();
    
    let mutex = Mutex::new(());
    let arc_mutex = Arc::new(mutex);

    for id in 0..num_of_threads {
        let thread_mutex = Arc::clone(&arc_mutex);
        
        array_of_threads.push(thread::spawn(move || {
            print_lots(id, thread_mutex)
        }));
    }
    
    for t in array_of_threads {
        t.join().expect("Thread join failure");
    }
    
    println!("\n=== Approach 2: Using stdout().lock() ===");
    
    let mut array_of_threads = vec!();
    
    for id in 0..num_of_threads {
        array_of_threads.push(thread::spawn(move || {
            print_lots_alt(id)
        }));
    }
    
    for t in array_of_threads {
        t.join().expect("Thread join failure");
    }
}

//* mutex approach */
fn print_lots(id: u32, lock: Arc<Mutex<()>>) {
    // acquire the lock - this blocks until the lock is available
    let _guard = lock.lock().unwrap();
    
    // while the _guard exists, we have exclusive access to stdout
    println!("Begin [{}]", id);
    for _i in 0..100 {
        print!("{} ", id);
    }
    println!("\nEnd [{}]", id);
    
    // _guard automatically drops here when it goes out of scope
    // which releases the lock for other threads
}

//* stdout().lock() approach */
fn print_lots_alt(id: u32) {
    // get a handle to stdout
    let stdout = io::stdout();
    // lock stdout for exclusive access
    let mut handle = stdout.lock();
    
    // use write! and writeln! with the locked handle
    writeln!(handle, "Begin [{}]", id).unwrap();
    for _i in 0..100 {
        write!(handle, "{} ", id).unwrap();
    }
    writeln!(handle, "\nEnd [{}]", id).unwrap();
    
    // lock is automatically released when handle goes out of scope
}