// Lab H - Producer Consumer Problem
// Jayden Holdsworth - 2025
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;
use rand::Rng;

//* Data */
struct Data {
    is_empty: Mutex<bool>,
    cv: Condvar,
}

impl Data {
    fn new() -> Self {
        Data {
            is_empty: Mutex::new(true),
            cv: Condvar::new(),
        }
    }

    fn produce(&self) {
        let mut is_empty = self.cv.wait_while(
            self.is_empty.lock().unwrap(),
            |is_empty| !*is_empty
        ).unwrap();
        
        *is_empty = false;
        println!("Produced item");
        self.cv.notify_one();
    }

    fn consume(&self) {
        let mut is_empty = self.cv.wait_while(
            self.is_empty.lock().unwrap(),
            |is_empty| *is_empty
        ).unwrap();
        
        *is_empty = true;
        println!("Consumed item");
        self.cv.notify_one();
    }
}

//* Consumer */
fn consumer_main(data: Arc<Data>, id: usize) {
    let mut rng = rand::rng();
    
    for i in 0..10 {
        thread::sleep(Duration::from_millis(rng.random_range(50..200)));
        data.consume();
        println!("Consumer {}: Consumed {}", id, i);
    }
}

//* Producer */
fn producer_main(data: Arc<Data>, id: usize) {
    let mut rng = rand::rng();
    
    for i in 0..10 {
        thread::sleep(Duration::from_millis(rng.random_range(50..150)));
        data.produce();
        println!("Producer {}: Produced {}", id, i);
    }
}

//* Main */
fn main() {
    let data = Arc::new(Data::new());
    let mut handles = vec![];

    let num_consumers = 3;

    for id in 0..num_consumers {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            consumer_main(data_clone, id);
        });
        handles.push(handle);
    }

    let num_producers = 3;
    for id in 0..num_producers {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            producer_main(data_clone, id);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}