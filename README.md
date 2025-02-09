# RustLabs
 Repo for the lab work seen in parallel and concurrent programming Y3

 # Lab A
 I successfully set it up and got it running.

 # Lab B 

 ## Q1

 Open the folder **first_thread**, in VS Code and run `cargo init` in the **terminal window**.

Open the main Rust file and edit the auto-generated Rust code.  Move the print "hello world" to a new function called `my_function`, and then call `my_function` from the main program.

Look at the lecture material for guidance on function syntax.

Now replace the synchronous call to your function with an asynchronous call i.e. create a thread with your function set as its thread Function.
Use the following snippets of Rust as a guide:

```Rust
    std::thread::spawn(move || my_function() );
```

Now expand your code to include a second thread, which calls a second function.  Add a print statement to this second function.
Notice how occasionally the output is jumbled.  This is a race condition, caused by a thread unsafe print method.  We'll address these topics in a future lecture.

Add a sleep method to one of your functions.  Sleep is available in the thread library.

Use the following snippet of Rust as a guide:

```Rust
    std::thread::sleep(std::time::Duration::new(5, 0));
```

## A1

I wrote this code:

```Rust

fn main() {
    std::thread::spawn(move || my_function() );
    std::thread::spawn(move || my_function_too() );
}

fn my_function(){
    println!("Hello, world!");
}

fn my_function_too(){
    println!("I'm asleep!");
    std::thread::sleep(std::time::Duration::new(5, 0));
    println!("I'm awake!");
}

```

Sometimes I get this result:

```
PS C:\users\725291\OneDrive - hull.ac.uk\gh\RustLabs\LabB\first_thread> cargo run 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target\debug\first_thread.exe`
Hello, world!
I'm asleep!
```

Other times I run it, I get this result:

```
PS C:\users\725291\OneDrive - hull.ac.uk\gh\RustLabs\LabB\first_thread> cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target\debug\first_thread.exe`
Hello, world!
```

Like the question says, this is an example of a race condition caused by a thread unsafe print method.

## Q2

Open the folder **joining_threads**, in VS Code and run `cargo init` in the **terminal window**.

Copy the code from the previous exercise into `main.rs`

Alter the code to create N threads rather than two threads, where N is a arbitrary number.  Use a for loop to achieve this.

Use the following snippets of Rust as a guide, where id is the loop counter and `num_of_threads` the loop limit:

```Rust
    for id in 0..num_of_threads {
        // Add code here
    }
```

Now that you can create an arbitrary number of threads, the next step is to ensure that all threads terminate before the application (the primary thread) terminates.  To do this we can use a join method.
The join method blocks until the thread has terminated.

```Rust
t.join();
```

However, in order to use the join method you need to hold a reference to each thread.  To do this, use a list to store the threads when they are created.  Then iterate over the list to ensure they are all joined.
The following snippets of Rust show the basics of list manipulation.

```Rust
    let mut list_of_threads = vec!();

    list_of_threads.push(t);

    for t in list_of_threads {
        // Add code here
    }
```

Remember Rust variables are immutable (constant) by default.  Use the `mut` keyword to make them mutable.

Add a print statement at the end of the main section of code and check that all threads have terminated prior to seeing this print message.

## A2

This is the code I wrote:

```Rust

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
    
    for t in list_of_threads {
        t.join().unwrap();
    }
    println!("All threads have terminated");
}

```

This code iterates through a for loop, creating a thread each pass-through and adding it to an array of threads. Then, when they are all created, it again iterates through the array, systematically joining them one by one, therefore terminating them.

## Q3

Now that you have the basic framework for creating and joining threads, experiment with giving the threads items of work, as well as altering the number of threads used.

Use **Windows Task Manager** to observe your program running on the CPU.

Info on each core is available on the **Performance** tab, then right click and change graph to **logical processors**

## A3

I altered the code to create 3000 threads:

```Rust

fn main() {
    println!("Hello, world!");
    let num_of_threads = 3000;
    let mut list_of_threads = vec!();
    for id in 0..num_of_threads {
        let t = std::thread::spawn(move || {
            println!("Thread {} is running", id);
        });
        list_of_threads.push(t);
    }
    
    for t in list_of_threads {
        t.join().unwrap();
    }
    println!("All threads have terminated");
}

```

And it added 3000 threads, although not entirely sequentially:

```
Thread 2963 is running
Thread 2989 is running
Thread 2998 is running
Thread 1673 is running
Thread 1186 is running
Thread 1585 is running
Thread 2977 is running
All threads have terminated
```

When I run this program, I notice some small spikes on the CPU's in task manager. Even on the CPU's that are marked as 'parked'. I assume this is due to the program, though I cannot completely confirm this. 

I do not know what it means if a CPU core is 'parked', or entirely why the threads are initialised out of order, or what exactly a 'thread unsafe print method' means, but I am excited to learn.





