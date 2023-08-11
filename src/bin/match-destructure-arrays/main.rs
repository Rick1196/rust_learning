// like tuples, arrays and slices can be destructured this way
fn main() {
    let array = [1, -2, 6];
    match array {
        [0, second, third] => println!("array[0] = 0, 1 = {}, 2 = {}", second, third),
        [1, _, third] => println!("array[0] = 1, 2 = {} and 1 was ignored", third),
        [-1, second, ..] => println!(
            "array[0]=-1, 1 = {}, and the other one were ignored",
            second
        ),
        // or store them in another array/slice (the type depends on that of the values that is)
        // being matched against
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),
        // combining these patterns we can for example bind the first and last values
        // and store the rest of them in a single array
        [first, middle @ .., last] => {
            println!("array[0] = {}, middles = {:?} 2 = {}", first, middle, last)
        }
    }
}
