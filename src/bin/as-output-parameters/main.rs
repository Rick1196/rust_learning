// closures as input parameters are possible, so returning closures as output
// parameters should also be possible. However, anonymous closure types are, by definition
// unknown so we have to use `impl` trait to return them

// the valid traits for returning a closure are
// - Fn
// - FnMut
// - FnOnce

// beyond this, the move keyword must be used, which signals that all captures
// occur by value, This is required because any captures by reference would be dropped as
// soon as the function exited leaving invalid references in the closure

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMuyt".to_owned();
    move || println!("This is a {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();
    move || println!("This s a {}", text)
}

fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}
