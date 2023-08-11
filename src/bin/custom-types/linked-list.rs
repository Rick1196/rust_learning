// a common way to implement a liked list is via enums
use crate::List::*;
enum List {
    // cons: Tuple strucut that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Null: A node that signifies the end of the end of the linked list
    Nil,
}

// Methods can e attached to an anum
impl List {
    // create an empty list
    fn new() -> List {
        // NIl has type `List`
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behaviour of this mehtod
        // depends on the variant of `self`
        // concete type `T` is preferred over a match on a reference `&T`
        // after Rust 2018 yuou can use self here and tail (with no ref)
        // bellow as well, rust will infer &s and ref tails
        match *self {
            // can't take ownership of the tails because `self` is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: And empty list has zero length
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `foramt!` is similar to `print!` but returns the heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    // create and empty lined list
    let mut list = List::new();
    // prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
