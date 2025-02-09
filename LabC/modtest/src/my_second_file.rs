fn main() {
    std::thread::spawn(move || my_function() );
    std::thread::spawn(move || my_function_too() );
}

fn my_function(){
    println!("Hello, world!");
}

pub fn my_function_too(){
    println!("I'm asleep!");
    std::thread::sleep(std::time::Duration::new(5, 0));
    println!("I'm awake!");
}
