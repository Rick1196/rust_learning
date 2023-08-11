// closures succintly capture variables from enclosing scopes. Does this have any consequeces?
// it does. Observe how suing a closure a a function parameter requires generics, which is
// necessary because of how they are defined
fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

// when c closure is defined, the compiler implictly creates a new anonymous structure
// to store the captures varaibles inside, meanwhile implmenting the functionality via one of the
// `traits: Fn, FnMut, FnOnce for this unknown type` This type is assigned to the variable
// which is stores until calling
// since this new type is of unknown type, any usage in a fuinction will requires generics
// however an unbounded type parameter <T> would still be ambiguous and not bellowed.
// Thus bounding by one tof the `traits Fn, FnMut or FnOnce` is sufficient to specify its type

// `F` must implement `Fn` for a closure which takes not
// inputs and returns nothing - exactly what is required
// for `print`
// same as above function
fn main() {
    let x = 7;
    // capture `x` into an anonymous type and implement
    // `Fn` for it, Store it in `print`
    let print = || println!("{}", x);
    apply(print);
}
