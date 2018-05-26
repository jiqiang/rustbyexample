fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
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

fn main() {
    let list = vec![3, 2, 4, 6, 1];
    let largest = largest(&list);
    println!("{}", largest);

    let p1 = Point {x: 1, y: 2};
    println!("{:?}", p1);

    let p2 = Point {x: 1.0, y: 2.0};
    println!("{:?}", p2);

    let p3 = Point {x: 1, y: 2.0};
    println!("{:?}", p3);
}