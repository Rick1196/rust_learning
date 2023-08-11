// the `for in` construct can be used to iterate through an `Iterator` One of the
// easiest ways to create an iterator is to use the range notation a..b. This yields
// values from a to b in steps of one

// the `for in` construct is able to interact wiht an `Interator` in several ways.
// As discussed in the section on the `Iterator` trait, by default the `for` loop
// will apply the `into_iter` function to the collection
// However this is not the only means of converting collestions into iterators

// `into_iter` `iter` and `iter_mut` all handle the conversion fo a collection
// into an iterator in different ways by providing different view on the data
// within
// -> `iter` - this vorrwos each element fo the collection through each iteration thus
// leaving the collection untouched and available for reuse after the loop

fn test_iter() {
    let names = vec!["bob", "frank", "ferris"];
    for name in names.iter() {
        match name {
            &"ferris" => println!("There is a rustacean among us"),
            _ => println!("Hello {}", name),
        }
    }
}

// -> `into_iter` this consumes the collection so that on each iteration
// the exact data is provided. Once the collection has been consumed it is no longer
// avaialble for reuse as it has been `moved` within the loop

fn test_into_iter() {
    let names = vec!["bob", "frank", "ferris"];
    for name in names.into_iter() {
        match name {
            "ferris" => println!("There is rustacean among us"),
            _ => println!("hello {}", name),
        }
    }
    // you can fix this changing the upper to into
    // for name in names.clone().into_iter() {
    // println!("names: {:?}", names);
}

// -> `ite_mut` this mutably borrow each element of the collection,
// allowing for the collection to be modified in place
fn test_iter_mut() {
    let mut names = vec!["bob", "frank", "ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "ferris" => "There is rustance among us",
            _ => "hellow",
        }
    }
    println!("names: {:?}", names);
}

fn main() {
    test_iter();
    test_into_iter();
    test_iter_mut();
    // `n` will take the values : 1,2...100
    // or we can use
    // for n in 1..=100
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 3 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
