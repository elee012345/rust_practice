// https://doc.rust-lang.org/book/ch03-03-how-functions-work.html

fn main() {
    println!("Hello, world!");
    another_function(3, 2.1);

    // a macro in rust is kind of weird
    // you can't do something like let y = ( let x = 3 ); because let x = 3; does not return a value,
    // unlike in some languages
    // instead you do:
    let z = {
        let x = 3;
        x + 1
    };
    // this is a macro and assigns y the value of 4
    // you define x, then add one to it. Note that the x + 1 does not have a semicolon;
    // this allows it to returns its value. With the semicolon the program would error because
    // it would not return a value
    println!("The value of z is {z}");

    let value = echo(10);
    println!("{value}");
}

// functions are defined by the keyword fn and are defined using snake case
// pretty self explanatory
// when you use parameters in a function you must define its type
// multiple parameters defined below
fn another_function(x: i32, y: f64) {
    println!("I'm another function :D");
    println!("The value of x is {x} and the value of y is {y}.");
}

// define return type by doing -> [type]
fn echo(x: i32) -> i32 {
    // automatically returns x because it is the last value and it also has no semicolon
    x
}

// you can also use the return keyword
fn amazing_function() -> i32 {
    return 3 + 2;
}