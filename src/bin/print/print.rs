fn main() {
    // `{}` will be replaced automatically with any arguments
    println!("{} days", 31);

    // positional arguments, specifiyinf an integer inside `{}`
    // determines which additional argument will be replaced
    println!("{0}, this is {1}", "Hello", "Sparta!!");

    // as can named arguments
    println!("{name} {lastname}", name = "ricardo", lastname = "perez");

    // different formatting can be invoked by speciyinf the format character
    // after a `:`
    println!("Base 10: {}", 69420);
    println!("Base 2: {:b}", 69420);
    println!("Base 8: {:o}", 69420);
    println!("Base 16: {:x}", 69420);

    //  you can right-justify text with a specified width, this will
    //  output "    1"
    println!("{number:>5}", number = 1);

    // you can use named arguments in the format specifier by appending a `$`
    println!("{number:0>width$}", number = 1, width = 5);

    //  only types that implement fmt::Display can be formated with `{}`
    //  User defined types do not implement display fmt::Display by default
    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);
    // This will not compile because `Structure` does not implement
    // fmt::Display.
    //println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    // Fix the issue in the above code (see FIXME) so that it runs without error.
    // Try uncommenting the line that attempts to format the Structure struct (see TODO)
    // Add a println! macro call that prints: Pi is roughly 3.142 by controlling the number of decimal places shown. For the purposes of this exercise, use let pi = 3.141592 as an estimate for pi. (Hint: you may need to check the std::fmt documentation for setting the number of decimals to display)
}
