// since closures may be used as arguments, you might wonder if the same can be said about
// functions. And indeed they can declare a function that takes a closure as parameter
// then any function that satisfies the trait bound of that closure can be passed as parameter
// define a function which takes a generic `F` argument
// bound by `Fn` and calls it
fn call_me<F: Fn()>(f: F) {
    f();
}

// define a wrapper function satisfying the `Fn` bound
fn function() {
    println!("Im a function");
}

fn main() {
    // define a closure satisfying the `Fn` bound
    let closure = || println!("I'm a closure");
    call_me(closure);
    call_me(function);
}
