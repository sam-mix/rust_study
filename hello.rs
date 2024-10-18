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

    println!("==================================================================================");

    // Single Placeholder
    println!("{}", 1);

    // Multiple Placeholder
    println!("{} {}", 1, 3);

    // Positional Arguments
    println!(
        "{0} is {1} {2}, also {0} is a {3} programming language",
        "Rust", "cool", "language", "safe"
    );

    // Named Arguments
    println!(
        "{country} is a diverse nation with unity.",
        country = "India"
    );

    // Placeholder traits :b for binary, :0x is for hex and :o is octal
    println!("Let us print 76 is binary which is {:b} , and hex equivalent is {:0x} and octal equivalent is {:o}", 76, 76, 76);

    // Debug Trait
    println!(
        "Print whatever we want to here using debug trait {:?}",
        (76, 'A', 90)
    );

    // New Format Strings in 1.58
    let x = "world";
    println!("Hello {x}!");

    println!("==================================================================================");

    for (v, c) in (0..10 + 1).enumerate() {
        println!("The {c} number loop");
        if v == 9 {
            println!("Here we go continue?");
            continue;
        }
        println! {"The value of v is : {v}"};
    }
    println!("==================================================================================");

    let mut array: [i32; 5] = [1, 2, 3, 4, 6];
    print_arrays(array);
    println!("The elements: {array:?}");
}

fn print_arrays(mut array: [i32; 5]) {
    array[0] = 89;
    array[1] = 90;
    array[2] = 91;
    array[3] = 92;
    array[4] = 93;
    println!("The elements: {array:?}");
}
