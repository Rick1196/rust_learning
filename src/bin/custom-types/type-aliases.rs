enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Substract,
}

// create a type alias
type Oprations = VeryVerboseEnumOfThingsToDoWithNumbers;
// the most common place you'll see this is in `impl` block using the self alias

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Substract => x - y,
        }
    }
}

fn main() {
    // we can refer to each variant via its alias, not its long and inconvenient
    // name
    let x = Oprations::Add;
}
