fn largest<T>(list: &[T]) -> T
    where T: PartialOrd + Copy {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest2<T>(list: &[T]) -> &T
    where T: PartialOrd {
    let mut largest = &list[0];
    for item in list.iter() {
        if *item > *largest {
            largest = item;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y
        }
    }
}

impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("x is greater than or equal to y");
        } else {
            println!("y is greater than x");
        }
    }
}

fn main() {
    let list = vec![3, 2, 4, 6, 1];
    let largest = largest(&list);
    println!("largest: {}", largest);
    let largest2 = largest2(&list);
    println!("largest2: {}", largest2);

    let p1 = Point {x: 1, y: 2};
    println!("{:?}", p1);

    let p2 = Point {x: 1.0, y: 2.0};
    println!("{:?}", p2);

    let p3 = Point {x: 1, y: 2.0};
    println!("{:?}", p3);
    println!("{}", p3.x());

    let pair = Pair::new(1, 2);
    pair.cmp_display();
}
