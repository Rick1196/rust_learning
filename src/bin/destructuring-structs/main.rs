// similarly a `strucut` can be destructured as shown
fn main() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // try chancging the values in the struct to see what happens
    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {}, y = {}", b, y),
        // you can destructure strucuts and rename the variables,
        // the order is not importatn
        Foo { y: 2, x: i } => println!("y is 2, i ={:?}", i),
        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we dont care about x", y),
        // this will give an error: pattern does not mentions field `x`
        // Foo {y} => println("y = {}",y)
    }
}
