// closures are function that can capture the enclosing environemt
// for example, a closure that captures the `x` varialbe
// |val| val + x

// the syntax and capabilities of closures make them very convenient
// for the fly usage. Calling a closure is exactly like callung a function
// however both input and return types can be inferred and input varialbe names
// must be specified
//

// Other characteristics of closures include:
// - Using || istead of () around input varialbes
// - optional body delimination ({}) for a single expression
// - the ability to capture the outer environment variables

fn main() {
    let outer_var = 42;
    // a regular function can't refer to variables in the enclosing environment
    // fn function(i:32) -> {i +_ outer_var}
    // Closures are anonymous, here we are binding them to references
    // annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named varialbes
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred = |i| i + outer_var;

    // call the closures
    println!("closure annotated: {}", closure_annotated(1));
    println!("closure inferred: {}", closure_inferred(1));

    // ounce closure's type has been inferred, it cannot be inferred agait with
    // another type
    //println!("cannot reuse closure_inferred with another type: {}", closure_inferred(42i64));
    // TODO: uncomment the line above and see the compiler error.

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());
}
