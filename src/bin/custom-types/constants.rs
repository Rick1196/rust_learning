// rust has two different type of constatns which can be declared in any scrope
// including global. Both require explicit type annotation:

// `const`: An unchangeable value(common case)
// `static`  a possibly `mut`able variable with `static` lifetime.
// the static lifetime is inferred and does not have to be specified
// Accessing or modifying a mutable static variable is `unsafe`

// globals are declared outside other scopes
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    let n = 16;
    // access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
    // THRESHOLD = 5;
}
