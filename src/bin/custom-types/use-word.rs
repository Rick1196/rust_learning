// the use declaration can be used so manual scoping isn't needed
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // explicitly `use` each name so the are available without
    // manual scoping
    use crate::Status::{Poor, Rich};
    // automatically `use` each name inside `Work`
    use crate::Work::*;

    // equivalent to `Status::Poor`
    let status = Poor;
    // equivalent to `Status::Civilian`
    let work = Civilian;
    match status {
        // Note the lack of scoping because of the explicit `use` above
        Rich => println!("Rich"),
        Poor => println!("Poor"),
    }

    match work {
        Civilian => println!("Civilian work"),
        Soldier => println!("Soldier"),
    }
}
