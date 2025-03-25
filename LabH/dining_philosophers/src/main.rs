use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;
use rand::prelude::*;

struct Fork {
    id: usize,
    available: Mutex<bool>,
}

impl Fork {
    fn new(id: usize) -> Fork {
        Fork {
            id,
            available: Mutex::new(true),
        }
    }
}

struct Philosopher {
    name: String,
    left_fork: Arc<Fork>,
    right_fork: Arc<Fork>,
}

impl Philosopher {
    fn new(name: &str, left_fork: Arc<Fork>, right_fork: Arc<Fork>) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left_fork,
            right_fork,
        }
    }

    fn run(&self) {
        let mut rng = rand::rng();
        loop {
            let sleep_time = rng.random_range(1000..5000);
            println!("{} is thinking", self.name);
            thread::sleep(Duration::from_millis(sleep_time));
            
            println!("{} is hungry", self.name);
            self.dine();
            
            println!("{} is full", self.name);
        }
    }

    fn dine(&self) {
        let mut rng = rand::rng();
        
        println!("{} is trying to take fork {}", self.name, self.left_fork.id);
        let left_guard = self.left_fork.available.lock().unwrap();
        if !*left_guard {
            println!("{} couldn't get fork {}", self.name, self.left_fork.id);
            return;
        }
        let mut left_guard = left_guard;
        *left_guard = false;
        println!("{} acquired fork {}", self.name, self.left_fork.id);
        
        // Try to acquire the right fork
        println!("{} is trying to take fork {}", self.name, self.right_fork.id);
        let right_guard = self.right_fork.available.lock().unwrap();
        if !*right_guard {
            *left_guard = true;
            println!("{} couldn't get fork {}, released fork {}", 
                     self.name, self.right_fork.id, self.left_fork.id);
            return;
        }
        let mut right_guard = right_guard;
        *right_guard = false;
        println!("{} acquired fork {}", self.name, self.right_fork.id);
        
        // Eat
        println!("{} is eating", self.name);
        let eat_time = rng.random_range(1000..5000);
        thread::sleep(Duration::from_millis(eat_time));
        println!("{} is done eating", self.name);
        
        // Release both forks
        *left_guard = true;
        *right_guard = true;
        println!("{} released forks {} and {}", self.name, self.left_fork.id, self.right_fork.id);
    }
}

fn main() {
    // Create 5 forks
    let forks: Vec<Arc<Fork>> = (0..5).map(|i| Arc::new(Fork::new(i))).collect();
    
    // Create philosophers
    let philosophers = vec![
        Philosopher::new("Socrates", Arc::clone(&forks[0]), Arc::clone(&forks[1])),
        Philosopher::new("Plato", Arc::clone(&forks[1]), Arc::clone(&forks[2])),
        Philosopher::new("Aristotle", Arc::clone(&forks[2]), Arc::clone(&forks[3])),
        Philosopher::new("Descartes", Arc::clone(&forks[3]), Arc::clone(&forks[4])),
        Philosopher::new("Confucius", Arc::clone(&forks[4]), Arc::clone(&forks[0])),
    ];
    
    // Create a thread for each philosopher
    let mut handles = vec![];
    
    for philosopher in philosophers {
        let handle = thread::spawn(move || {
            philosopher.run();
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}