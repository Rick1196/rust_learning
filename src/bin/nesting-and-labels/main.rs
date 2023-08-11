// it's possible to break or continue outer loops when
// dealing with nested loops. in this cases, the loops must be
// anotated with some `label` and the labels must be passed to the
// `break/continue` statement
#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("entered the outer loop");
        'inner: loop {
            println!("entered the inner loop");
            // this would break only the inner loop
            // break;
            // this breaks the outer loop
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");
}
