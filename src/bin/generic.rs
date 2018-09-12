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
}
