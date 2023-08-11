// the type inference enigine is pretty smart. it does more than looking at the type
// of the value expression during initialization. It also looks at how the variable
//  is used afterwards to infer its type. Here's an advanced example of typ einference

fn main() {
    // because of the annotation, the compiler knows that `elem` has type u8
    let elem = 5u8;

    // create an empty vector (a growable array)
    let mut vec = Vec::new();
    // at this point the compiler doesn't know the exact type of `vec`,
    // it just knows that it's a vector of something (`Vec<_>`)

    // insert `elem` in vector
    vec.push(elem);
    // aha! now the compiler knwos that `vec` is vector of `u8`s
    // TODO try commentinf out the `vec.push(elem)` line
    println!("{:?}", vec);
}
