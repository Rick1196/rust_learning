// an `enum` is destructured similarly
// `allow` required to silence warning because only one variant is used
#[allow(dead_code)]
enum Color {
    // these 3 are specified solely  by their name
    Red,
    Blue,
    Green,
    // These likewise tie `u32` tuples to different names: color modles
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    let color = Color::RGB(122, 17, 40);
    println!("What color is it?");
    // an `enum` can be structured using `match`
    match color {
        Color::Red => println!("The color is red"),
        Color::Blue => println!("The color is blue"),
        Color::Green => println!("The color is green"),
        Color::RGB(r, g, b) => println!("Red {} green:{} bluew:{}", r, g, b),
        Color::HSV(h, s, v) => println!("Hue {}, saturation: {}, value:{}", h, s, v),
        Color::HSL(h, s, l) => println!("hue: {}, saturation: {} lightnes:{}", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan:{},magenta:{}, yellow:{}", c, m, y),
        Color::CMYK(c, m, y, k) => println!("cyan: {}, magenta:{}, yellow:{}, key:{}", c, m, y, k),
    }
}
