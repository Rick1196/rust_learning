// for some use cases, when matching enums `match` is awkward. For example:
// make optional of type `Option<i32>`
// let optional = Some(7);
// match optional {
//     Some(i) =>{
//         println("This is a really long string and `{:?}`", i);
//     }
//     _=> {}
// }

// if let is cleaner for this use case and in addition allows various failure options to be specified

enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

fn enum_example() {
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // Variable a matches foo bar
    if let Foo::Bar = a {
        println!("a is foobar")
    }

    // variable b does not batch Foo::Bar
    // so this will print nothing
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    // variable c matched Foo::Quz which has a value
    // similar to Some() in the previous example
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // binding also works with `if let`
    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    }
}

fn main() {
    // all have type `Option<i32>`
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // the `if let` construct read "if `let` destructures `number` into"
    // `Some(i)` evaluate the block
    if let Some(i) = number {
        println!("Matched {:?}", i)
    }
    // if you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        // Destructured failed. Change to the failure case
        println!("Didn't match a number. Let's go with a letter");
    }
    // procide an altered failing condition
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}", i);
        // destructure failed. Evaluate and `else if` condition to see if the
        // alterate failure branch should be taken:
    } else if i_like_letters {
        println!("Didn't matcha number, let's go with a letter");
    } else {
        println!("I don't like letter. Let's go with an emoticon");
    }
    enum_example();
}

// in same way `if let` can be used to match any enum value
