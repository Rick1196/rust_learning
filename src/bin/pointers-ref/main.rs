// for pointers, a distinction needs to be made between destructuring and deferencing as they
// are different concepts which are used differently from languages like C/C++
// -> Deferencing uses `*`
// -> Destructuring uses `&`, `ref` and `ref mut`

fn main() {
    // assign a reference of type `i32` the `&` signifies there is a
    // reference being assigned
    let reference = &4;
    match reference {
        // if `reference` is pattern matched against `&val`, it results
        // in a comparision like:
        // `&i32`
        // `&val`
        // ^ we see that if the matching `&`s are dropped, the the `i32`
        // should be assigned to `val`
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // to avoid the `&` you deference before matching
    match *reference {
        val => println!("Got a value via referencing: {:?}", val),
    }

    // what if you don't start with a reference? `reference` as a `&`
    // because the right side was already a reference, This is not a
    // reference because the right side is not one

    let _not_a_reference = 3;
    // rust provide `ref` for axactly this purpose, it modifies the
    // assigment so that a reference is created for the element;l this
    // references is assigned
    let ref _is_a_references = 3;
    // accordingly by defining 2 values without references, references
    // can be retrived via `ref` and `ref mut
    let value = 5;
    let mut mut_value = 6;
    // use `ref` keyword to crteate a reference
    match value {
        ref r => println!("Got a reference to a value {:?}", r),
    }

    // use `ref mut` similarly
    match mut_value {
        ref mut m => {
            // got a reference. Gotta dereference it before we can
            // add anything to it
            *m += 10;
            println!("We added 10. `mut_value` {:?}", m);
        }
    }
}
