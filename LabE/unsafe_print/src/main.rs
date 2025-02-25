fn main() {
    let num_of_threads = 4;
    let mut array_of_threads = vec!();

    for id in 0..num_of_threads {
        array_of_threads.push(std::thread::spawn(move || print_lots(id)) );
    }

    for t in array_of_threads {
        t.join().expect("Thread join failure");
    }
}

fn print_lots(id: u32) {
    println!("Begin [{}]", id);
    for _i in 0..100 {
        print!("{} ", id);
    }
    println!("\nEnd [{}]", id);
}
