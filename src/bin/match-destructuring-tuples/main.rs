// tuples can be destructured in a `match` as follows
fn main() {
    let triple = (0, -2, 3);
    println!("tell me about {:?}", triple);
    match triple {
        (0, y, z) => println!("First is 0, y is {:?} and z is {:?}", y, z),
        (1, ..) => println!("first is 1 and the rest doesn't matter"),
        (.., 2) => println!("last is 2 and the rest doesn't matter"),
        (3, .., 4) => println!("first is 3. last is 4 and the rest doesn't matter"),
        _ => println!("it doesn't matter what they are"),
    }
}
