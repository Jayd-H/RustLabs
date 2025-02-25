# 600086-Lab-E

## Q1. Thread safe printing

Open the **unsafe_print** folder in Visual Studio Code.

Build and run the code, and ensure that it works.
Examine the code to become familiar with the syntax.

The output should appear jumbled, as multiple threads compete for the single print function.  This is called a race condition.  You might have to run it several times to see the jumble.

Make a copy of the **unsafe_print** folder and name it **safe_print**.

The remainder of **Q1** will work in the **safe_print** folder.

Implement the thread safe printing as described during the lectures.

You will need to use both `Mutex` and `Arc` to create a critical section within the `print_lots()`

Create the `Mutex` and `Arc` within `main()`, to enable them to be shared with all the threads.

Ensure that the result of `print_lots()` no longer demonstrates a race condition.

What happens to your code if you fail to release the mutex?

Are you able to verify this in your code?

What happens if you raise an exception within the critical section?

Extend your code to verify your answer

## Q2. Triangles using OpenGL

Open the **triangles** folder in Visual Studio Code.

This is a Rust program that uses OpenGL to render some simple triangles to the scene.

Build and run the code to ensure it works.

Although this is a relatively small piece of code by OpenGL standards, it does use some advanced Rust language constructs.  Don't panic, you are not expected to understand all of this code, even if you have previously studied graphics.

We'll look at the code in a little more detail in future lectures.

For this lab, focus on the code between the comments:

```Rust
// Begin render loop
```

and

```Rust
// End render loop
```

The code clears the screen, and then individually positions ten triangles on the display.  It uses the counter `delta_t` to animate the triangles.

Update the code to move the triangles around the screen in a more chaotic pattern.

The following code can be used to generate a floating point random number.

```Rust
let x = rand::random::<f32>();
```

Use this code to help create a more chaotic movement of triangles.

### Note

As part of the final large lab exercise, you will be required to create a large parallel simulation to be run on both the CPU and GPU.  You will be using this OpenGL framework to help visualize the results.  It is therefore important that you complete this lab exercise so that you start becoming familiar with the OpenGL / Rust interface.
