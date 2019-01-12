struct Container {
    bytes: Box<u8>,
}

fn test() -> Container {
    Container {
        bytes: Box::new(23),
    }
}

fn main() {
    let x = test();
    println!("{}", x.bytes);
}
