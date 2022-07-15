fn main() {
    let num = 5;

    // rust doesn't like having unnecessary parentheses around simple statements like this one
    // simple little if, else if, and else statements
    if num == 5 {
        println!("num was equal to five");
    } else if num > 5 {
        println!("num was greater than five");
    } else {
        println!("num was less than five");
    }
    // the not equal to operator is the standard !=

    // similar to the java ternary operator, rust has the ability to bind the result of an 
    // if else statement to a variable
    let boolean = true;
    // boolean is true, one gets assigned the value 1
    let one = if boolean {1} else {2};

    // you can't do:
    // let error = if boolean {1} else {"2"};
    // rust is a compiled language and must know the type at compile time

    // loops
    let mut counter = 0;

    // loop {} is an infinite loop
    let result = loop {
        counter += 1;

        // breaks after 10 loops, returns the value of counter * 2
        // in this case 20 and assigns this value to the variable result
        if counter == 10 {
            break counter * 2;
        }
    };
    // prints 20
    println!("The result is {result}");


    // while loops are pretty much the same
    let mut another_counter = 0;
    while another_counter < 10 {
        println!("another_counter = {another_counter}");
        // rust does not support ++
        // so can't do another_counter++;
        another_counter += 1;
    }

    let array = [1, 2, 3, 4, 5];
    // looping through an array with a for loop similar to python
    for value in array {
        println!("value = {value}");
    }

    // a normal for loop
    // similar to for i in range(num) in python
    // the .rev() reverses the range so that we count down instead
    for num in (1..10).rev() {
        println!("num = {num}");
    }

}
