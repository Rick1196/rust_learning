// rust provide type safety via statix typing. Variable bindings can by type annotated
// when declared. However, in most cases, the compiler will be able to infer the type of the
// variable from the context heavily reducing the annotation burden
// Values(like literals) can bound to variables, using the `let` binding

fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;
    println!("an integer: {:?}", copied_integer);
    println!("a boolean: {:?}", a_boolean);
    println!("meet the unit value {:?}", unit);

    // the compiler warns about unused varaible bindings;
    // this warnings can be silences by prefixing the variable with underscore
    let _unused_variable = 3u32;

    let _noisy_unusued_variable = 2u32;
}
