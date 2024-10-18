fn main() {
    // Prints the output
    print!("Hello World\n");

    // Appends a new line after printing
    println!("Appending a new line");

    // Prints as an error
    eprint!("This is an error\n");

    // Prints as an error with new line
    eprintln!("This is an error with new line");

    // Initializing and declaring a variable
    let some_variable = "This_is_a_variable";
    println!("{}", some_variable);

    // Making a variable mutable
    let mutable_variable = "Mutable";
    // mutable_variable = "c";
    println!("{}", mutable_variable);

    // Assigning multiple variables
    let (_name, _age) = ("ElementalX", 20);

    // (Global) constant
    const SCREAMING_SNAKE_CASE: i64 = 9;

    println!("{}", SCREAMING_SNAKE_CASE);
}
