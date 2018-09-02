fn main() {
    let v: Vec<i32> = vec![1, 2, 3];

    match v.get(2) {
        Some(i) => {
            println!("{}", i);
        },
        None => {
            println!("index is out of range");
        }
    }
}