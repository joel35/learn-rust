#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
#[derive(Clone)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

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

    let new_rect = Rectangle {
        top_left: Point { x: 10., y: 0. },
        bottom_right: Point { x: 0., y: 5. },
    };

    let area = rect_area(&new_rect);
    println!("rect area: {}", area);


    let my_square = square(Point { x: 3.432, y: 9. }, 10.);
    dbg!(&my_square);

    dbg!(rect_area(&my_square));
}

// 1. Add a function rect_area which calculates the area of a Rectangle (try using nested destructuring).

fn rect_area(rect: &Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { y: start_0, x: finish_0 },
        bottom_right: Point { y: start_1, x: finish_1 }
    } = rect;

    // (finish_0.abs() - start_0.abs()) * (finish_1.abs() - start_1.abs()).abs()
    (start_0 - start_1).abs() * (finish_0 - finish_1).abs()
}

// 2. Add a function square which takes a Point and a f32 as arguments, and returns a Rectangle with
// its top left corner on the point, and a width and height corresponding to the f32.

fn square(top_left_point: Point, sides: f32) -> Rectangle {
    Rectangle {
        top_left: Point { ..top_left_point },
        bottom_right: Point { x: top_left_point.x - sides, y: top_left_point.y + sides },
    }
}