// rust provides a `loop` keyword to indicate an infinite loop

// the break statement can be used to exit a loop at anytime, whereas the `continue`
// statement can be used to skip the rest fo the iteration and start a new one
fn main() {
    let mut count = 0u32;
    println!("lets cound until infinity");
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            // skipp the rest of this iteration
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("Ok, that's neought");
            break;
        }
    }
}
