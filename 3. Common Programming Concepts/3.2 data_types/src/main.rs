fn main() {
    // rust has four scalar types (basically primitives in java)
    // they are integers, characters (more than just ascii), floats, and booleans

    // there are a few types of integers
    // there are unsigned and signed integers from 8 to 128 bits
    
    // PREFIXING UNUSED VARIABLES WITH AN UNDERSCORE _

    // default integer is an unsigned 32 bit
    // aka i32
    let _x = 1;

    // floats are basically the same, and default to 64 bits 
    // y's type is f64
    let _y = 2.0;

    // you can manually set the variables type
    let _z: f32 = 3.2;

    // rust's math is much the same as java 
    // same integer rules and stuff

    // addition
    let sum = 5 + 10;
    println!("The sum of 5 and 10 is {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The difference of 95.5 and 4.3 is {difference}");

    // multiplication
    let product = 4 * 30;
    println!("The product of 4 and 30 is {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("The quotient of 56.7 and 32.2 is {quotient}");
    let floored = 2 / 3; // Results in 0
    println!("The quotient of 2 and 3 is {floored}");

    // remainder
    // two ints make an int with modulus
    let remainder = 43 % 5;
    println!("The remainder of 43 and 5 is {remainder}");

    // booleans are pretty ez
    let _bool = true;
    let _other_bool: bool = false;

    // rust has all sorts of characters you can use
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';


    // on to compound types 
    // rust has arrays (like java) and tuples (like python)
    // these are the two 'primitive' compound types

    // you can define different types for each value in the tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // you can define variables off of tuples
    let another_tuple = (500, 6.4, 1);
    let (x, y, z) = another_tuple;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    // get individual values from tuples
    let _five_hundred = tup.0;


    // arrays have a fixed length
    // more complicated arrays and stuff like arraylists are vectors in rust
    // they're defined by square brackets
    let array = [1, 2, 3, 4, 5];

    // define array type 
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    // same thing as let array = [3, 3, 3, 3, 3];
    let array = [3; 5];

    // access index of an array
    let index_zero = array[0];
    println!("first element in array is {index_zero}");


}
