// varaible bindings are inmmutable by default, but this can be overriden
// using `mut` modifier
fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);
    // ok
    mutable_binding += 1;
    println!("after mutation: {}", mutable_binding);
    // error
    // _immutable_binding += 1;
}
