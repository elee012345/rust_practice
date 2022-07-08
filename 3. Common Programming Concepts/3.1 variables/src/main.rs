// https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
fn main() {
    // make the variable mut so that you're able to redefine it as 6
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // defining a constant
    // constants are written in SCREAMING_SNAKE_CASE lmao
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // able to redefine y through the let keyword 
    // making it mut and changing the value just changes the value
    // re-using the let keyword redefines the variable entirely
    let y = 5;

    let y = y + 1;

    // using curly brackets changes the scope 
    // pretty cool though as of now I don't know when you would use this
    // possibly when you want to use a certain varible or something but don't 
    // want to use a new variable to use it for something
    {
        // different scope now
        let y = y * 2;
        // prints 12
        println!("The value of x in the inner scope is: {y}");
    }
    // prints 6
    println!("The value of x is: {y}");


    // difference between a mut variable and re-using the let keyword below
    // my main takeaway from this is that you make a variable mutable if you want to be able to 
    // change its value but not its type 
    // you redefine it with the let keyword when you want to change the type 
    // you use a constant rather than a normal variable with the let keyword and without the mut keyword 
    // when you don't want your variable to ever be redefined
    // you use just the let keyword when you want to be able to redefine the variable and as a different type 
    // idk when you would want that lol
     
    /*

    Shadowing is different from marking a variable as mut, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

    The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name. For example, say our program asks a user to show how many spaces they want between some text by inputting space characters, and then we want to store that input as a number:

    fn main() {
        let spaces = "   ";
        let spaces = spaces.len();
    }
    The first spaces variable is a string type and the second spaces variable is a number type. Shadowing thus spares us from having to come up with different names, such as spaces_str and spaces_num; instead, we can reuse the simpler spaces name. However, if we try to use mut for this, as shown here, we’ll get a compile-time error:

    fn main() {
        let mut spaces = "   ";
        spaces = spaces.len();
    }
    The error says we’re not allowed to mutate a variable’s type:

    $ cargo run
    Compiling variables v0.1.0 (file:///projects/variables)
    error[E0308]: mismatched types
    --> src/main.rs:3:14
      |
    2 |     let mut spaces = "   ";
      |                      ----- expected due to this value
    3 |     spaces = spaces.len();
      |              ^^^^^^^^^^^^ expected `&str`, found `usize`

    For more information about this error, try `rustc --explain E0308`.
    error: could not compile `variables` due to previous error
    Now that we’ve explored how variables work, let’s look at more data types they can have.

     */
}