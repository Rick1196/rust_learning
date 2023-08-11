// closure are inherently flexible and will do what the functionality requires
// to make the closure work without annotation. This allows capturing to flexibly adapt to the
// use case, sometimes moving and sometimes borrowing. Closures can capture variables
// - by reference: &T
// - by mutable reference: &mute T
// - by value: T

// they preferentially capture varialbes by reference and only go lower when required

fn main() {
    use std::mem;
    let color = String::from("green");

    // a closure to print `color` which immediately borrows (`&`) `color` and
    // stores the borrow and closure in the `print` variable. It will remain
    // borrowed until `print` is used the last time

    // `println!` onlyu requires arguments by ummutable reference so it doesnt
    // impose anything more restrictive
    let print = || println!("`color: {}`", color);
    // call the closure using the borrow
    print();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`

    let _reborrow = &color;
    print();

    // a move or borrow is allowed after the final use of `print`
    let _color_moved = color;

    let mut count = 0;
    // a closure to increment `count` could take eihter `&mut count` or `coutn`
    // but `&mut count` is less restrictive so it takes that. Immediately borrows
    // `count`
    //
    //  A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
    // calling the closure mutates the closure which requires a `mut`
    let mut inc = || {
        count += 1;
        println!("count: {}", count);
    };

    // call the closure using a mutable borrow
    inc();

    // the closure still mutably borrows `count` because it is called later
    //  an attempt to borrow will lead to an error
    // let _reborrow = &count;
    inc();

    // the closure no longer need to borrow `&mut count` therefore, it is
    // bossible to reborrow without an error
    let _count_reborrow = &mut count;

    // a non-copu type
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must be take by value.  A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure
    let consume = || {
        println!("`movable`:{:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the varialbe so this can only be called once
    consume();
    move_examples();
}
// using `move` before vertical pipes forces closure to take ownership of captured variables:

fn move_examples() {
    // `Vec` has non-copy semantics
    let heystack = vec![1, 2, 3];
    let contains = move |needle| heystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));
    // println!("There are {} elements in vec", heystack.len());
    // Uncommenting above line will result in compile time error
    // because borrow check doesnt allow re-using variable after it
    // has been moved
    // removing `move` from closure's signature will cause closure
    // to borrow _haystack_ variable immutably, hence _heystack_ is still
    // available and uncommenting above line will not cause an error
}
