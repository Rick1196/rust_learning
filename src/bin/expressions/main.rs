// a rust programm is mostly made up of a series of statements
// There are a few kinds of statements in Rust. The must common two are
// declaring a variable binding and using a `;` with an expression

// Blocks are expressiont too, so they can be used as values ins assignments
// The last expression in the block will be assigned to the place expression such a local
// variable. However if the last expression of the block ends with a semicolon,
// the return value will be ()

fn main() {
    let x = 5u32;
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        x_cube + x_squared + x
    };

    let z = {
        // the semicolon supresses this expression and `()` is assigned to `z`
        2 * x;
    };
    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
