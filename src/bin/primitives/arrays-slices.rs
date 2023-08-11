use std::mem;

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    // fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    // indexing starts at 0
    println!("First element of the array {}", xs[0]);

    // `len` returns the count of elements
    println!("Number of elements in array:{}", xs.len());

    // arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // arrays can be automatically borrowed as slices
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // arryas can be safely accessed using `.get` which returns
    // `Option` this can be matched as shown below, or sed with
    // `expect()` if you would like the program to exit with a nice
    // mesage insted of happily continue
    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{}:{}", i, xval),
            None => println!("Slow donw! {} is too far", i),
        }
    }
}
