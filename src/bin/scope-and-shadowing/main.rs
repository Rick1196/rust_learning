// varialbe bindings have a scope, and are constrained to live in a block
// a block is a collection of statements enclosed by braces `{}`

fn declare_first() {
    let a_bidning;
    {
        let x = 2;
        // initialize the binding
        a_bidning = x * 2
    }

    println!("a binding{}", a_bidning);

    let another_binding;
    // error
    println!("another bidning:{}", another_binding);
    another_binding = 1;
    println!("another_binding: {}", another_binding);
}

fn freezing() {
    let mut _mutable_integer = 7i32;
    {
        // shadowing by ummutable `mutable_integer`
        let _mutable_integer = _mutable_integer;
        // Error `_mutable_integer` is frozen in this scope
        _mutable_integer = 50;
    }

    // ok `mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
}

fn main() {
    // this binding lives in the main function
    let long_lived_binding = 1;
    let shadowed_binding = 1;

    // this is a block, and has a smalled scope than the main function
    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
        let shadowed_binding = "abc";
        println!("inner shadowd {}", shadowed_binding);
    }

    // Error `short_lived_binding` doesn't exist in this scope
    // println!("outer short: {}", short_lived_binding);
    println!("outer long: {}", long_lived_binding);
    println!("outer shadowed {}", shadowed_binding);
    declare_first();
}
