// and attribute to hide warnings for unused code
#![allow(dead_code)]
use std::fmt;
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// a Tutple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x:{}, y:{})", self.x, self.y)
    }
}

struct Rectangle {
    // A rectangle can be specified by where the top left
    // and bottom right corners are in space
    top_left: Point,
    bottom_right: Point,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(top_left:{}, bottom_right:{})",
            self.top_left, self.bottom_right
        )
    }
}

fn rectangle_area(rectangle: &Rectangle) -> f32 {
    let missing_point: Point = Point {
        x: rectangle.top_left.x,
        y: rectangle.bottom_right.y,
    };
    println!("top_left: {}", rectangle.top_left);
    println!("bottom_right: {}", rectangle.bottom_right);

    println!("missing point: {}", missing_point);
    let l: f32 = missing_point.x - rectangle.bottom_right.x;
    let w: f32 = rectangle.top_left.y - missing_point.y;
    return l * w;
}

fn square(point: Point, complement: f32) -> Rectangle {
    let rectangle = Rectangle {
        // structu instantiation is ans expression too
        top_left: Point {
            x: point.x,
            y: point.y,
        },
        bottom_right: Point {
            x: point.x - complement,
            y: point.y + complement,
        },
    };
    return rectangle;
}

fn main() {
    // create strucut with field init shorthand
    let name = String::from("Peter");
    let age = 27;

    // print debu struct
    println!("{:?}", name);

    // Instantiate a `point`
    let point: Point = Point { x: 10.3, y: 0.4 };
    // access the fields of the point
    println!("point coordinates: ({},{})", point.x, point.y);
    // make a new point by using struct update syntax to use the
    // fields of our other one
    let bottom_right = Point { x: 5.2, y: 0.0 };
    // bottom_right.y willbe the same as `point.y` because we used that
    // field from `point`
    println!("second point: ({},{})", bottom_right.x, bottom_right.y);

    // destructure the point using a 'let' binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let rectangle = Rectangle {
        // structu instantiation is ans expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    let _unit = Unit;
    let pair = Pair(1, 0.1);
    // access the fields of a tuple strucut
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // destructure a tuple strucut
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
    println!("rectangle area: {}", rectangle_area(&rectangle));
    let new_rectangle: Rectangle = square(Point { x: 10.0, y: 0.0 }, 5.5);
    println!("new rectangle: {}", new_rectangle);
    println!("rectangle area: {}", rectangle_area(&new_rectangle));
}
