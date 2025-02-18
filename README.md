# RustLabs

Repo for the lab work seen in parallel and concurrent programming Y3

# Lab A

I successfully set it up and got it running.

# Lab B

## Q1

Open the folder **first_thread**, in VS Code and run `cargo init` in the **terminal window**.

Open the main Rust file and edit the auto-generated Rust code. Move the print "hello world" to a new function called `my_function`, and then call `my_function` from the main program.

Look at the lecture material for guidance on function syntax.

Now replace the synchronous call to your function with an asynchronous call i.e. create a thread with your function set as its thread Function.
Use the following snippets of Rust as a guide:

```Rust
    std::thread::spawn(move || my_function() );
```

Now expand your code to include a second thread, which calls a second function. Add a print statement to this second function.
Notice how occasionally the output is jumbled. This is a race condition, caused by a thread unsafe print method. We'll address these topics in a future lecture.

Add a sleep method to one of your functions. Sleep is available in the thread library.

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

Alter the code to create N threads rather than two threads, where N is a arbitrary number. Use a for loop to achieve this.

Use the following snippets of Rust as a guide, where id is the loop counter and `num_of_threads` the loop limit:

```Rust
    for id in 0..num_of_threads {
        // Add code here
    }
```

Now that you can create an arbitrary number of threads, the next step is to ensure that all threads terminate before the application (the primary thread) terminates. To do this we can use a join method.
The join method blocks until the thread has terminated.

```Rust
t.join();
```

However, in order to use the join method you need to hold a reference to each thread. To do this, use a list to store the threads when they are created. Then iterate over the list to ensure they are all joined.
The following snippets of Rust show the basics of list manipulation.

```Rust
    let mut list_of_threads = vec!();

    list_of_threads.push(t);

    for t in list_of_threads {
        // Add code here
    }
```

Remember Rust variables are immutable (constant) by default. Use the `mut` keyword to make them mutable.

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

# Lab C

## Q1

Multiple Rust files can be linked together into a single application with the use of the `mod` statement

Placing `mod my_second_file` in one Rust file, enables that file to use the functions from `my_second_file.rs`

Make use of this functionality to move your thread main function, from the previous lab to its own Rust file. This method requires that you prefix any imported functions with `my_second_file`

```Rust
    my_second_file::run();
```

An alternative method is the `use` command which allows you to drop the requirement to prefix

```Rust
    mod my_second_file;
    use my_second_file::run;
```

## A1

I ended up with this code:

```Rust

mod my_second_file;


fn main() {
    println!("Hello, world!");
    my_second_file::my_function_too();
}

```

```Rust

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


```

I followed the brief, making the function public and the results were expected.

## Q2

Open up the folder **Ownership**.

Familiarise yourself with the `struct Person` and the `impl Person`. The `struct` defines the data members for a Person. The `impl` provides the behaviour for a Person.

```Rust
struct Person {
    name: String,
    age: u32
}
```

```Rust
impl Person {
    fn new_default() -> Person {
        Person {
            name: "Joe Bloggs".to_string(),
            age: 25
        }
    }

    fn new(name_param: &str, age_param: u32) -> Person {
        Person {
            name: name_param.to_string(),
            age: age_param
        }
    }
}
```

Note that Rust does not allow overriding of function names, so we have to define unique names for each "constructor".

Comment out `new_default` to avoid the warning as we'll not use this function in the remainder of the exercise.

A basic print function `print_person` has been provided to print out a person's details (this would ordinarily have been included in the `impl` section, but for simplicity its define outside as a basic function)

The main function, creates a `Person` struct and passes it to the `print_person`.

Add a second call to `print_person` within `main`, to print out the details a second time. Why does this not compile?

Hint: Rust has very struct ownership rules.

Now alter the code so that the `print_person` returns the Person object back to the `main`.

It would be very cumbersome to have to pass objects into functions and then to return them, so ownership could be retained. Rust has an alternative, the reference.

Modify `print_person` to use a reference

```Rust
    fn print_person(p: &Person)
```

Rust permits permits shared ownership on an immutable reference. This is called **borrowing**.

You should now be able to use multiple calls to `print_person` without any issue.

Add the function `increment_age` which takes a mutable reference as a parameter.

```Rust
fn increment_age(p: &mut Person) {
    p.age = p.age + 1;
}
```

Call this function once then twice within `main`. You'll also need to make the `Person` object mutable

```Rust
fn main() {
    let mut p1 = Person::new("Jane", 30);

    print_person(&p1);
    print_person(&p1);

    increment_age(&mut p1);
    increment_age(&mut p1);
}

fn print_person(p: & Person) {
    println!("{} is {} years old", p.name, p.age);
}

fn increment_age(p: &mut Person) {
    p.age = p.age + 1;
}
```

Rust has a rule that states you cannot have more than one mutable reference to the same object, neither can you have even a single mutable reference to an object that has one or more immutable references.

So why does the code you have created, work?

The answer is that Rust is able to determine if a reference (whether mutable or immutable) is still being used.

Now lets see if we can cause the borrowing to fail. We'll add some explicit immutable and mutable references to our code.

```Rust
fn main() {
    let mut p1 = Person::new("Jane", 30);

    let r1 = & p1;
    let r2 = & p1;

    print_person(r1);
    print_person(r2);

    let r3 = &mut p1;

    increment_age(r3);
}
```

`r1`, `r2` and `r3` are references

This should compile and execute just fine.

Now move one of the calls of `print_person` below the call to `increment_age`.

The code should no longer compile.

Rust has seen that we are trying to use a mutable reference to an object for which there is still an active immutable reference.

You can find more details here: <https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html>

## A2

This was my final code:

```Rust


struct Person {
    name: String,
    age: u32
}

impl Person {

    fn new(name_param: &str, age_param: u32) -> Person {
        Person {
            name: name_param.to_string(),
            age: age_param
        }
    }
}

fn main() {
    let mut p1 = Person::new("Jane", 30);

    let r1 = & p1;
    let r2 = & p1;

    print_person(r1);
    print_person(r2);

    let r3 = &mut p1;

    increment_age(r3);

    print_person(r3);

    increment_age(r3);

    print_person(&r3);
}

fn print_person(p: &Person) {
    println!("{} is {} years old", p.name, p.age);
}

fn increment_age(p: &mut Person) {
    p.age = p.age + 1;
}

```

I greatened my knowledge of the borrow checker in Rust and how it deals with references and where to use them. When I tried to iniutialy add a second call to print_person within main to print out the details a second time. This was the error I was getting in console:

```

PS D:\Files\Documents\AProjects\Rust\ParallelAndConcurrentProgrammingLabs\LabC\ownership> cargo run
   Compiling ownership v0.1.0 (D:\Files\Documents\AProjects\Rust\ParallelAndConcurrentProgrammingLabs\LabC\ownership)
error[E0382]: use of moved value: `p1`
  --> src\main.rs:21:18
   |
18 |     let p1 = Person::new("Jane", 30);
   |         -- move occurs because `p1` has type `Person`, which does not implement the `Copy` trait
19 |
20 |     print_person(p1);
   |                  -- value moved here
21 |     print_person(p1);
   |                  ^^ value used here after move
   |
note: consider changing this parameter type in function `print_person` to borrow instead if owning the value isn't necessary
  --> src\main.rs:24:20
   |
24 | fn print_person(p: Person) {
   |    ------------    ^^^^^^ this parameter takes ownership of the value
   |    |
   |    in this function
note: if `Person` implemented `Clone`, you could clone the value
  --> src\main.rs:2:1
   |
2  | struct Person {
   | ^^^^^^^^^^^^^ consider implementing `Clone` for this type
...
20 |     print_person(p1);
   |                  -- you could clone this value

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership` (bin "ownership") due to 1 previous error

```

I now know the cause of these errors through this lab, as well as how to solve them. I was able to use multiple calls to print_person without an issue.

## Q3

In the lecture you were introduced to a Rust class called `SharedData`. This class is used for sharing of data across two or more threads

```Rust
pub struct SharedData {
    value: u32
}

impl SharedData {
    pub fn new() -> SharedData {
        SharedData {
            value: 0
        }
    }

    pub fn update(&mut self) {
        let local_value = self.value;
        std::thread::sleep(std::time::Duration::new(1,0));
        self.value = local_value + 1;
    }

    pub fn print(&self) {
        println!("SharedData: value = {}", self.value)
    }
}
```

Familiarise yourself with how Rust implements:

- the constructor i.e. `new`
- methods
- data members
- the self object (The equivalent to C#'s `this` keyword)

The main program illustrates how to use the `SharedData` struct

Notice how placing `SharedData` in a separate Rust file, hides all the components and we have to use the `pub` keyword to make them visible to `main`

Create a new thread function which takes `SharedData` as a parameter and then calls the `update` and `print` functions.

Now run your new thread function in a thread, as per the previous Lab

Once this is working, try and move the `print` function from your thread function to the main program. You will notice that this causes ownership issues.

These cannot be resolved by the ownership techniques we learnt earlier. To solve this particular problem will require some new Rust techniques e.g. `arc` or the asynchronous reference counters. We'll investigate this in a future lab.

However, the reason why Rust does not allow the sharing of SharedData between main and your thread function is too eliminate the possibility of a race condition. The Rust language was created in response to a requirement for a very efficient language that was safe to use for multi-threaded problems.

## A3

This was the final code I ended up with:

```Rust

mod shared_data;
use shared_data::SharedData;

fn main() {

    let shared_data = SharedData::new();
    shared_data.print();
}

fn update_print(shared_data: &SharedData) {
    shared_data.update();
    shared_data.print();
}

```

```Rust


pub struct SharedData {
    value: u32
}

impl SharedData {
    pub fn new() -> SharedData {
        SharedData {
            value: 0
        }
    }

    pub fn update(&mut self) {
        let local_value = self.value;
        std::thread::sleep(std::time::Duration::new(1,0));
        self.value = local_value + 1;
    }

    pub fn print(&self) {
        println!("SharedData: value = {}", self.value)
    }

}

```

I have greatened my knowlege on how classes work, and how they correlate over to Rust from C++. I am excited to find out how to fix this error that was expected to happen from the brief:

```

error[E0596]: cannot borrow `*shared_data` as mutable, as it is behind a `&` reference
  --> src\main.rs:11:5
   |
11 |     shared_data.update();
   |     ^^^^^^^^^^^ `shared_data` is a `&` reference, so the data it refers to cannot be borrowed as mutable
   |
help: consider changing this to be a mutable reference
   |
10 | fn update_print(shared_data: &mut SharedData) {
   |                               +++

For more information about this error, try `rustc --explain E0596`.
error: could not compile `shared_data` (bin "shared_data") due to 1 previous error

```

## Lab C Reflection

Despite already having some knowledge in Rust before this due to personal projects over the years. I appreciate these labs for giving me time to reflect over what I know and even gain a greater understanding of them. I appreciate the parrallels that are drawn between C++ and Rust, highlighting where they differ from eachother in certain areas. I am excited to see how this knowledge will fit in when it comes to multi-threading. Despite still being a novice, I feel better about my Rust ability after the Lab.

# Lab D

## Q1

In the lecture and Lab C we looked at Rust class called `SharedData`. This class is used for sharing of data across two or more threads

```Rust
pub struct SharedData {
    value: u32
}

impl SharedData {
    pub fn new() -> SharedData {
        SharedData {
            value: 0
        }
    }

    pub fn update(&mut self) {
        let local_value = self.value;
        std::thread::sleep(std::time::Duration::new(1,0));
        self.value = local_value + 1;
    }

    pub fn print(&self) {
        println!("SharedData: value = {}", self.value)
    }
}
```

In Lab C you created a new thread function which takes `SharedData` as a parameter and calls the `update` and `print` functions.

When you tried to move the `print` function from your thread function to the main program. You would have experienced ownership issues.

These cannot be resolved by the ownership techniques we learnt earlier. To solve this particular problem will require some new Rust techniques. This lab begins by exploring one of these, namely: reference counters.

Create a new Rust folder and add the following code to main.rs

```Rust
struct Aircraft<'a> {
    name: String,
    engines: Vec<&'a Engine>,
}

impl Aircraft<'_> {
    pub fn new(name_param: &str) -> Aircraft {
        Aircraft {
            name: name_param.to_string(),
            engines: Vec::new()
        }
    }
}

struct Engine {
    name: String,
}

impl Engine {
    pub fn new(name_param: &str) -> Engine {
        Engine {
            name: name_param.to_string(),
        }
    }
}

fn main() {
    let engine1 = Engine::new( "General Electric F404" );
    let engine2 = Engine::new( "General Electric F404" );
    let mut f18 = Aircraft::new( "F-18" );

    f18.engines.push (&engine1);
    f18.engines.push (&engine2);

    println! ("Aircraft: {} has a {} and {} ", f18.name, f18.engines[0].name, f18.engines[1].name );
}
```

This code creates two structs `Aircraft` and `Engine`. `Aircraft` contains a vector of references that can hold a number of engines.

In `main` we create two `Engine` objects and an `Aircraft` object and the then link them together.

Examine the code and make sure you understand the syntax.

Note: The strange `'a` notation attached to the reference is called a lifetime parameter. It allows the compiler to determine whether all references are going to stay "alive" at least as long as the "parent". In our example, it ensure that the engines will exist at least as long as the aircraft. What would happen if this was not the case?

Try to expand the code to include a data member in `Engine` that links to the `Aircraft`.

The limitation with the current code is that due to ownership restrictions it is not possible to link the `Aircraft` to an `Engine`

## A1

Adding an aircraft data member to our engine struct like this throws an error:

```Rust

struct Engine {
    name: String,
    aircraft: &Aircraft,
}

```

```

PS D:\Files\Documents\AProjects\Rust\ParallelAndConcurrentProgrammingLabs\LabD\Aircraft> cargo run
   Compiling Aircraft v0.1.0 (D:\Files\Documents\AProjects\Rust\ParallelAndConcurrentProgrammingLabs\LabD\Aircraft)
error[E0106]: missing lifetime specifier
  --> src\main.rs:17:15
   |
17 |     aircraft: &Aircraft,
   |               ^ expected named lifetime parameter
   |
help: consider introducing a named lifetime parameter
   |
15 ~ struct Engine<'a> {
16 |     name: String,
17 ~     aircraft: &'a Aircraft,
   |

error[E0106]: missing lifetime specifier
  --> src\main.rs:17:16
   |
17 |     aircraft: &Aircraft,
   |                ^^^^^^^^ expected named lifetime parameter
   |
help: consider introducing a named lifetime parameter
   |
15 ~ struct Engine<'a> {
16 |     name: String,
17 ~     aircraft: &Aircraft<'a>,
   |

error[E0063]: missing field `aircraft` in initializer of `Engine`
  --> src\main.rs:22:9
   |
22 |         Engine {
   |         ^^^^^^ missing `aircraft`

Some errors have detailed explanations: E0063, E0106.
For more information about an error, try `rustc --explain E0063`.
error: could not compile `Aircraft` (bin "Aircraft") due to 3 previous errors

```

This is because we get into a circular reference situation, wherein Aircraft refers to Engine which refers back to Aircraft. Because of Rust's ownership rules, this does not compile. To solve this, we need to implement reference counting.

## Q2

Let's rewrite the code and use reference counters.

Create a new Rust folder and add the following code to main.rs

```Rust
use std::rc::Rc;

struct Aircraft {
    name: String,
    engines: Vec<Rc<Engine>>,
}

impl Aircraft {
    pub fn new(name_param: &str) -> Aircraft {
        Aircraft {
            name: name_param.to_string(),
            engines: Vec::new()
        }
    }
}

struct Engine {
    name: String,
}

impl Engine {
    pub fn new(name_param: &str) -> Engine {
        Engine {
            name: name_param.to_string(),
        }
    }
}

fn main() {
    let engine1 = Rc::new(Engine::new( "General Electric F404" ));
    let engine2 = Rc::new(Engine::new( "General Electric F404" ));

    let mut f18 = Aircraft::new( "F-18" );

    f18.engines.push (engine1.clone());
    f18.engines.push (engine2.clone());

    println! ("Aircraft: {} has a {} and {}", f18.name, f18.engines[0].name , f18.engines[1].name );
    println! ("Engine: {} ", engine1.name );
    println! ("Engine: {} ", engine2.name );
}
```

This code uses `RC` (or reference counters) to act as smart pointers to the objects. We have removed the need to use references and lifetime parameters. Arguable this code is also now easier to understand.

Examine the code.

Use your knowledge of the Rust ownership model to explain what is happening with the reference counters and why we do not need to pass them as references.

Remove the `clone()` method from this line

```Rust
f18.engines.push (engine1.clone());
```

Can you explain why this program now fails to build?

Add a new boolean data member `requires_service` to `Engine`.

Then add a new method `service(&mut self)` to `Engine`. This method will just set the `requires_service` data member to `false`.

Now test your code with by adding the following to `main()`

```Rust
    let mut engine3 = Engine::new( "General Electric F404" );
    engine3.service();
```

You should be able to service engine3.

You will get a build error if you try and service engine2, which is accessed through an `rc`.

Again, using your knowledge of the Rust ownership module can you explain why the error is occurring?

This is a limitation of reference counters. We'll look to overcome this limitation in future labs.

## A2

Through using RC, a kind of smart pointer, it enables us to have the circular connection we wanted in the previous question. RC keeps track of how many references exist to a value, and when the last reference is dropped, the value is cleaned up.

If we remove the `clone` function from both of these lines, leaving us with this:

```Rust

f18.engines.push (engine1);
f18.engines.push (engine2);

```

We get this compiler error:

```

PS D:\Files\Documents\AProjects\Rust\ParallelAndConcurrentProgrammingLabs\LabD\Aircraft> cargo run
   Compiling Aircraft v0.1.0 (D:\Files\Documents\AProjects\Rust\ParallelAndConcurrentProgrammingLabs\LabD\Aircraft)
error[E0382]: borrow of moved value: `engine1`
    --> src\main.rs:39:30
     |
30   |     let engine1 = Rc::new(Engine::new( "General Electric F404" ));
     |         ------- move occurs because `engine1` has type `Rc<Engine>`, which does not implement the `Copy` trait
...
35   |     f18.engines.push (engine1);
     |                       ------- value moved here
...
39   |     println! ("Engine: {} ", engine1.name );
     |                              ^^^^^^^^^^^^ value borrowed here after move
     |
     = note: borrow occurs due to deref coercion to `Engine`
note: deref defined here
    --> C:\Users\Jayd\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\alloc\src\rc.rs:2224:5
     |
2224 |     type Target = T;
     |     ^^^^^^^^^^^
help: clone the value to increment its reference count
     |
35   |     f18.engines.push (engine1.clone());
     |                              ++++++++

error[E0382]: borrow of moved value: `engine2`
    --> src\main.rs:40:30
     |
31   |     let engine2 = Rc::new(Engine::new( "General Electric F404" ));
     |         ------- move occurs because `engine2` has type `Rc<Engine>`, which does not implement the `Copy` trait
...
36   |     f18.engines.push (engine2);
     |                       ------- value moved here
...
40   |     println! ("Engine: {} ", engine2.name );
     |                              ^^^^^^^^^^^^ value borrowed here after move
     |
     = note: borrow occurs due to deref coercion to `Engine`
note: deref defined here
    --> C:\Users\Jayd\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\alloc\src\rc.rs:2224:5
     |
2224 |     type Target = T;
     |     ^^^^^^^^^^^
help: clone the value to increment its reference count
     |
36   |     f18.engines.push (engine2.clone());
     |                              ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `Aircraft` (bin "Aircraft") due to 2 previous errors

```

The purpose of the `clone()` function here is not to clone the particular Engine, but rather to create a new reference to the same engine. When we remove it, `push` takes ownership of the Engine, therefore not allowing us to print the Engine's name. That's why we get the error `` error[E0382]: borrow of moved value: `engine2` ``.

If we add in the functionality of Engines having a 'service', we end up with this code.

```Rust

struct Engine {
    name: String,
    requires_service: bool,
}

impl Engine {
    pub fn new(name_param: &str) -> Engine {
        Engine {
            name: name_param.to_string(),
            requires_service: true,
        }
    }
    pub fn service(&mut self) {
        self.requires_service = false;
    }
}

fn main() {
    let engine1 = Rc::new(Engine::new( "General Electric F404" ));
    let engine2 = Rc::new(Engine::new( "General Electric F404" ));

    let mut engine3 = Engine::new( "General Electric F404" );
    engine3.service();

    let mut f18 = Aircraft::new( "F-18" );

    f18.engines.push (engine1.clone());
    f18.engines.push (engine2.clone());

    println! ("Aircraft: {} has a {} and {}", f18.name, f18.engines[0].name , f18.engines[1].name );
    println! ("Engine: {} ", engine1.name );
    println! ("Engine: {} ", engine2.name );

    println! ("Engine: {} requires service: {}", engine3.name, engine3.requires_service );

    engine2.service();
}

```

However, this throws an error.

```

PS D:\Files\Documents\AProjects\Rust\ParallelAndConcurrentProgrammingLabs\LabD\Aircraft> cargo run
   Compiling Aircraft v0.1.0 (D:\Files\Documents\AProjects\Rust\ParallelAndConcurrentProgrammingLabs\LabD\Aircraft)
error[E0596]: cannot borrow data in an `Rc` as mutable
  --> src\main.rs:52:5
   |
52 |     engine2.service();
   |     ^^^^^^^ cannot borrow as mutable
   |
   = help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Rc<Engine>`

For more information about this error, try `rustc --explain E0596`.
error: could not compile `Aircraft` (bin "Aircraft") due to 1 previous error

```

'engine3' is able to be serviced fine, but 'engine2' cannot. This is because RC only provides immutable access to its contents, it is designed for cases where you need shared ownership but not shared mutability.

## Lab D Reflection

Through this lab I have become more familiar with the 'RustTM' way of doing things. Despite having some previous Rust experience, the lab provided me with a nicely formatted way of understanding the more complex way data structures can interact in Rust. I understand that at this early stage of learning, the main focus here is not necessarily to give big coding tasks, but more bitesized examples on the 'why' of Rust programming, something that I do appreciate. Although I am still largely inexperienced, I am excited to have gotten more in-tune with Rust's infamous ownership and borrowing system. I am hyped to see where Refcel or Mutex fit into this. I assume we will be using Mutex in the next lab because this is all building up to being able to make multi-threaded programs.
