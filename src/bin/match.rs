fn age() -> u32 {
    15
}

fn main() {
    match age() {
        0 => println!("0"),
        n @ 1 ... 12 => println!("{} is between 1 and 12", n),
        n @ 13 ... 19 => println!("{} is between 13 and 19", n),
        _ => println!("whatever")
    }
}
