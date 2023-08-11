// rust provides no implicit type conversion between types. But explicit type
// conversion can be performed using the `as` word
// rules for converting between integral types follow C conventions generally
// except in cases where C has undefined behavior. The behavior of all casts
// between integral types is well defined in Rust

// suppress all warnings from casts which overflow
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321f32;
    // Error, no implicit conversion
    // let integer: u8 = decimal;

    // explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    // Error, there are limitations in conversion rules
    // a float cannot be directly converted to a char
    // let character = decimal as char;
    println!("casting: {} -> {} -> {}", decimal, integer, character);

    // when casting any valueto an unsigned type, T,
    // T::MAX + 1 is added or substracted until the value
    // fits into the new type

    // 1000 already fits in a u16
    println!("1000 as u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256-256 = 232
    // under the hood, the first 8 least significant but(LSB) are kept,
    // while the rest towards the most significant bit(MSB) get truncated
    println!("1000 as u8 is: {}", 1000 as u8);

    // -1 + 256 = 255
    println!("-1 as u8 is: {}", (-1i8) as u8);
    // for positive numbers this is the same as modulues
    println!("1000 mod 256 is: {}", 1000 % 256);

    // when casting to as signed type, the (bitwise) result is the smae as
    // first casting to the corresponding unsigned type. If the most isgnificat
    // bit of that value is 1, then the value is negative
    // unless it already fits of course
    println!("128 as i16 is {}", 128 as i16);

    // 128 as u8 -> 128 whoe value in 8bit two's complement representation is
    println!("128 as i16 is {}", 128 as i8);

    // repeating the example avobe
    // 1000 as u8 -> 232
    println!("1000 as u8 is:{}", 1000 as u8);
    // and the value of 232 in 8but two's complement representation is -24
    println!("232 as i8: {}", 232 as i8);

    // since rust 1.45 the `as` keyword performs a *saturating cast*
    // when casting from float to int. if the floating point value exceeds
    // the upper bound or is less than the lower bound, the returned value
    // will be equal to the bound crossed

    // 300.0 as u8 is 255
    println!("300.0 as u8 is :{}", 300.0_f32 as u8);
    // -100.0 as u8 is 0
    println!("-100.0 as u8 is: {}", -100.0_f32 as u8);

    // nan as u8 is 0
    println!("nan as u8 is :{}", f32::NAN as u8);

    // this behavior incurs a small runtime cost and can be avoided
    // with the unsafe methods, however the results might overflow and
    // return *unsoved values* use these methods wisely:
    unsafe {
        // 300.0 as u8 is 44
        println!("300.0 as u8 is: {}", 300.0f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!(" nan as u8 is :{}", f32::NAN.to_int_unchecked::<u8>());
    }
}
