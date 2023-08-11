// rust privuide pattern matching via the `match` keyword, which can be used like a C switch
// The first matching arm is evaluated and all possible values must be covered
fn main() {
    let number = 13;
    println!("Tell me about {}", number);
    match number {
        1 => println!("one"),
        2 | 3 | 5 | 7 | 11 => println!("this is a prime"),
        13..=19 => println!("a teen"),
        _ => println!("ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);
}
