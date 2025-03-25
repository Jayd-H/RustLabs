use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use rand::Rng;

struct Data {
    num_of_strips : usize,
    length_of_strip : usize,
    strips : Vec<Arc<Mutex<Vec<usize>>>>,
}

impl Data {
    fn new(num: usize, len: usize) -> Data {
        Data {
            num_of_strips: num,
            length_of_strip: len,
            strips: vec![Arc::new(Mutex::new(vec![0; len])); num],
        }
    }

    fn write(&self, index: usize, value: usize) {
        let strip_index = index / self.length_of_strip;

        if strip_index >= self.num_of_strips {
            panic!("Invalid index {}", strip_index);
        }

        let mut guard = self.strips[strip_index].lock().unwrap();
        guard[index % self.length_of_strip] = value;
    }

    fn _read(&self, index: usize) -> usize {
        let strip_index = index / self.length_of_strip;

        if strip_index >= self.num_of_strips {
            panic!("Invalid index {}", strip_index);
        }

        let guard = self.strips[strip_index].lock().unwrap();
        guard[index % self.length_of_strip]
    }    
}

fn main() {
    println!("Begin");
    let num_of_threads = 128;
    let mut list_of_threads = vec!();

    let shared_data = Arc::new(Data::new(num_of_threads, 256));
    
    for id in 0..num_of_threads {
        let data_clone = shared_data.clone();
        list_of_threads.push( std::thread::spawn( move || thread_main(id, data_clone) ) );
    }

    for t in list_of_threads {
        t.join().unwrap();
    }
    
    //* To avoid timing the prints, we have to seperate the printing and the reading */
    let start = SystemTime::now();

    for i in 0..shared_data.length_of_strip*shared_data.num_of_strips {
        let _ = shared_data._read(i);
    }

    let duration = start.elapsed().unwrap().as_micros();
    println!("Read time: {} microseconds", duration);
    

    //* This part is just for the printing, but it clogs up the output */
    //for i in 0..shared_data.length_of_strip*shared_data.num_of_strips {
    //    println! ("{} : {}", i, shared_data._read(i));
    //}
    
    println!("End");
}

fn thread_main(id: usize, data: Arc<Data>) {
    let mut rng = rand::rng();
    
    for _i in 0..10 {
        for _ in 0..data.length_of_strip*data.num_of_strips {
            let array_size = data.length_of_strip * data.num_of_strips;
            let index = rng.random_range(0..array_size);
            data.write(index, id);
        }
    }
}