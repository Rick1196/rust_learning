// the from and into traits are inherintly linked, and this is actually part
// of its implementations. if you are able to convert type A from type B, then
// it should be easy to believe that we should be able to convert type B to type A

// FROM
// the from trait allows for a type to define how to create itself from another type
// hence providing a very simple mechanism for converting between several types.
// There are numerous implementations of this trait within the standard library for
// conversion of prmitive and common type

// For example we can easily convert a `str` into a `string`

// let my_str = "hello";
// let my_string = String::from(my_str);

// the `Into` trait is simply the reciprocal of the From trait. That is, if you have
// implemented `From` trait for your type, Into will call it when necesary.

// Using the into trait will tipically require specificationof the type to convert into
// as the compiler is unable to determine this most of the time. However
// thsi is a small trade-off considering we get the functionality for free

// we can do similar for defininf a conversion for our own type
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    let int_num = 5;
    let num_from_int: Number = int_num.into();
    println!("my number is {:?}", num);
    println!("my number from int is {:?}", num_from_int);
}
