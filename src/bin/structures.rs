#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Struct can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn rect_area(rect: &Rectangle) -> f32 {
    return (rect.p2.x - rect.p1.x) * (rect.p2.y - rect.p1.y);
}

fn square(p: &Point, w: f32) -> Rectangle {
    return Rectangle {
        p1: Point { x: p.x, y: p.y + w },
        p2: Point { x: p.x + w, y: p.y },
    };
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;

    // Print assigned my_x and my_y
    println!("({}, {})", my_x, my_y);

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_x, y: my_y },
        p2: point,
    };

    // Instantiate an unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let p = Point { x: 0.2, y: 0.3 };
    let square = square(&p, 1.2);
    println!("square {:?}", square);

    let rect = Rectangle {
        p1: Point { x: 0.1, y: 0.1 },
        p2: Point { x: 0.2, y: 0.2 },
    };

    let area = rect_area(&rect);
    println!("area is {}", area);
}
