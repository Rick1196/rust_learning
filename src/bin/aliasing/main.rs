// the `type` statement can be used to give a new name to a
// existing type
// Types must have `UpperCamelCase` names, or the compiler will rise a warning
// The exception to this rule are the primitive types:
// `usize`, `f32` etc.

// `Nanosecond` `Inch` and `u64` are names for `u64`;

type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn main() {
    let nanoseconds: NanoSecond = 5 as u64;
    let inches: Inch = 2 as U64;

    // note that type aliases dont privide any extra type safety
    // because aliases are not new type
    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}
