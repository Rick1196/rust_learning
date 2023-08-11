// while rust chooses how to capture variables on the fly mostly without type annotation,
// this ambiguity is no allowed when writing functions. When taking a closure as an input parameter
//  the closures complete type must be annotated using one of a few traits, and the're determined
// by what the closure does with the captured value. In order of decreasing restriction, they are:
// -> Fn: the closure uses the captured value by reference &T
// -> FnMut: the closure uses the captured value by mutable reference &mut T
// -> FnOnce: the closure uses the captured value by value T

// on a variable-by-varialbe basis, the compiler will capture variables in the least restrictive manner possible

// For instance consider a parameter annotated as FnOnce. This specifies that the closure
// may capture by &T, &mut T or T, but the compiler will ultimately choose based on how
// the captured variables are used in the closure
// This is because if a move is possible, then any type of borrow should also be possible
// Not that the reverse is not true. Ifthe parameter is annotated as Fn, the capturing
// variables by &mut T or T are not allowed. However  &T is allowed
// In the following example, try swapping the usage of Fn, FnMut and FnOnce to see what happens

// A function which takes a closure as an argument and calls it
// <T> denotes that F is a "Generic type parameter"

fn apply<F>(f: F)
where
    // the closure takes no input and returns nothing
    F: FnOnce(),
{
    f();
}

// a function which takes a closure and returns an `i32`
fn apply_to_3<F>(f: F) -> i32
where
    // the closure takes an `i32` and returns an `i32`
    F: Fn(i32) -> i32,
{
    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // a non copy type
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodby".to_owned();

    // capture 2 variables `greeting` by reference and
    // farewell by value
    let diary = || {
        // greeting is by reference: requires `Fn`
        println!("I said {}", greeting);
        // mutation forces `farewell` to be captured by
        // mutable reference, Now requires `FnMut`
        farewell.push_str("!!!");
        println!("Then i screamed {}", farewell);
        println!("now I can sleep. zzzz");

        // manually calling drop forces `farewell ` to
        // be captured by value. Now requires `FnOnce`
        mem::drop(farewell);
    };
    //call the function which applies the closure
    apply(diary);
    let double = |x| 2 * x;
    // double satisfies `pply_to_3` trait bound
    println!("3 doubled {}", apply_to_3(double))
}
