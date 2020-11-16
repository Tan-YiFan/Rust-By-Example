#[derive(Debug)]
struct Person<'a> {
    // The 'a defines a lifetime
    name: &'a str,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// structs used as fields of other structs
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

// Activity 1: add function rect_area
fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: a, y: b },
        bottom_right: Point { x: c, y: d }
    } = rect;
    (b - d) * (c - a)
}

// Activity 2: add function square
fn square(point: Point, f: f32) -> Rectangle {
    let top_left: Point = Point { x: point.x, y: point.y + f };
    let bottom_right: Point = Point { x: point.x + f, y: point.y };
    Rectangle { top_left: top_left, bottom_right: bottom_right }
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    let point = Point { x: 10.3, y: 0.4 };

    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };


    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: top_edge, y: left_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}