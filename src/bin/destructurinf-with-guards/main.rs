// a match guard can be added to filter the arm
#[allow(dead_code)]
enum Temperature {
    Celcius(i32),
    Fehrenheit(i32),
}

fn main() {
    let temperature = Temperature::Celcius(35);
    match temperature {
        Temperature::Celcius(t) if t > 30 => println!("{}c is above 30 celsius", t),
        Temperature::Celcius(t) => println!("{}c is below 30 celsius", t),
        Temperature::Fehrenheit(t) if t > 86 => println!("{}F is above 86 farenheit", t),
        Temperature::Fehrenheit(t) => println!("{}F is bellow 86", t),
    }
}
