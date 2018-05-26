fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let list = vec![3, 2, 4, 6, 1];
    let largest = largest(&list);
    println!("{}", largest);
}