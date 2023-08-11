// indirectly accessing a varialbe makes it impossible to branch and use that
// variable without re-binding `match` privides the `@` sigil for binding values to names

fn age() -> u32 {
    15
}
fn main() {
    println!("Tell me what type of person you are");
    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        // could `match` 1 ..=12 directly but then what age
        // would the child be? instead, bind to n for the
        // sequence of 1..=12 now the age can be reported
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }
}
