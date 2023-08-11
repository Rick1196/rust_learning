fn main() {
    // integer adition
    println!("1 + 2 = {}", 1u32 + 2);

    // integer substraction
    println!("1 - 2 = {}", 1i32 - 2);
    // Try to change i32 to u32 to see why type is important

    // Scientific notation
    println!("1e4 is {}, -2.5e32 is {}", 1e4, -2.5e-3);

    // short circuiting boolean logic
    println!("true and false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 and 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // use underscores to improve readability
    println!("One million is written as {}", 1_000_000u32);
}
